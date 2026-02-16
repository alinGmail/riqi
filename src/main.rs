mod config;
mod data;
mod events;
mod holiday;
mod state;
mod theme;
mod ui;
mod utils;

use crate::config::model::AppConfig;
use crate::events::{handle_goto_mode_key_event, handle_normal_mode_key_event, AppEvent};
use crate::holiday::manager::HolidayManager;
use crate::holiday::modal::HolidayOfYearList;
use crate::holiday::utils::get_ylc_code;
use crate::state::{GotoPanelState, RiqiMode};
use crate::ui::bottom_line_component::BottomLineComponent;
use crate::ui::goto_panel_component::GotoPanelComponent;
use crate::utils::add_months_safe;
use chrono::{Datelike, Duration, Local, NaiveDate};
use clap::Parser;
use color_eyre::Result;
use config::{cli::Args, config_main::get_app_config};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use data::calendar::MonthCalendar;
use env_logger::{Builder, Target};
use log::{debug, LevelFilter};
use ratatui::prelude::*;
use ratatui::widgets::Clear;
use serde::Deserialize;
use state::RiqiState;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{self, stdout},
    sync::mpsc,
    thread,
};
use theme::theme_loader::load_theme_from_file;
use ui::{
    layout::get_layout,
    month_component::{self, MonthComponent},
};

#[derive(Deserialize, Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

fn setup_logger() {
    // 创建或覆盖日志文件
    let log_file = File::create("debug.log").expect("Failed to create log file");

    Builder::new()
        .target(Target::Pipe(Box::new(log_file))) // 输出到文件
        .filter_level(LevelFilter::Debug) // 设置日志级别
        .format_timestamp(None) // 可选：禁用时间戳
        .is_test(true) // 禁用颜色（避免乱码）
        .init();
}
#[tokio::main]
async fn main() -> Result<()> {
    setup_logger();
    color_eyre::install()?;
    // --- 1. 初始化终端 ---
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    // --- 2. 创建核心事件通道 ---
    let (tx, rx) = mpsc::channel();

    // --- 3. 状态与主循环 ---
    let mut todo_data: Option<Todo> = None;
    let mut error_msg: Option<String> = None;

    let now = Local::now();
    let args = Args::parse();
    let app_config = get_app_config(args);

    let theme = load_theme_from_file("ningmen").expect("主题加载失败");
    let mut riqi_state = RiqiState {
        select_day: now.date_naive(),
        today: now.date_naive(),
        theme,
        mode: RiqiMode::Normal,
        goto_panel: GotoPanelState {
            year: now.year() as u16,
            month: now.month() as u8,
            day: now.month() as u8,
            focus_inp: 0,
        },
        notification: vec![],
    };

    let now = Local::now();
    let mut holiday_map: HashMap<String, HolidayOfYearList> = HashMap::new();
    let mut calendar = MonthCalendar::new(now.year() as u32, now.month(), now.date_naive(), None);
    // 事件源 A: 终端输入监听线程 (将 crossterm 事件转发到 mpsc)
    let tx_input = tx.clone();
    thread::spawn(move || loop {
        if event::poll(std::time::Duration::from_millis(500)).unwrap() {
            if let Ok(ev) = event::read() {
                let _ = tx_input.send(AppEvent::TerminalEvent(ev));
            }
        }
    });
    let mut holiday_manager = HolidayManager::new(tx.clone());

    holiday_manager
        .load_ylc_holiday(
            &riqi_state.select_day.year().to_string(),
            &app_config.language,
            &app_config.country,
        )
        .await;

    // 初始手动触发一次渲染（显示“加载中”）
    draw_ui(&mut terminal, &calendar, &riqi_state, &app_config)?;

    loop {
        // 【关键】阻塞式接收：没有事件时，程序会停留在此处，不消耗 CPU
        match rx.recv().unwrap() {
            AppEvent::Quit => break,
            AppEvent::TerminalEvent(Event::Resize(_, _)) => {
                // 窗口大小改变，触发重绘
                draw_ui(&mut terminal, &calendar, &riqi_state, &app_config)?;
            }
            AppEvent::TerminalEvent(Event::Key(key)) => {
                if key.is_release() {
                    continue;
                }
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    if matches!(riqi_state.mode, RiqiMode::Normal) {
                        break;
                    }
                }
                // 判断是什么mode
                match riqi_state.mode {
                    RiqiMode::Normal => handle_normal_mode_key_event(key, &mut riqi_state),
                    RiqiMode::Goto => handle_goto_mode_key_event(key, &mut riqi_state),
                    _ => (),
                }

                let selected_day = riqi_state.select_day;
                let ylc_key = get_ylc_code(
                    &selected_day.year().to_string(),
                    &app_config.language,
                    &app_config.country,
                );
                holiday_manager
                    .load_ylc_holiday(
                        &riqi_state.select_day.year().to_string(),
                        &app_config.language,
                        &app_config.country,
                    )
                    .await;
                calendar = MonthCalendar::new(
                    riqi_state.select_day.year() as u32,
                    riqi_state.select_day.month(),
                    riqi_state.select_day,
                    holiday_map
                        .get(&ylc_key)
                        .map(|holiday_list| holiday_list.to_holiday_map()),
                );
                draw_ui(&mut terminal, &calendar, &riqi_state, &app_config)?;
            }
            AppEvent::UpdateHoliday(ylc_key, holiday_of_year) => {
                debug!("receive update holiday event, ylc: {}", &ylc_key);
                let old = holiday_map.get(&ylc_key);
                if let Some(old_holidays) = old {
                    if old_holidays.version >= holiday_of_year.version {
                        continue;
                    }
                }
                holiday_map.insert(ylc_key, holiday_of_year);
                let selected_day = riqi_state.select_day;
                let ylc_key = get_ylc_code(
                    &selected_day.year().to_string(),
                    &app_config.language,
                    &app_config.country,
                );
                calendar = MonthCalendar::new(
                    riqi_state.select_day.year() as u32,
                    riqi_state.select_day.month(),
                    riqi_state.select_day,
                    holiday_map
                        .get(&ylc_key)
                        .map(|holiday_list| holiday_list.to_holiday_map()),
                );
                draw_ui(&mut terminal, &calendar, &riqi_state, &app_config)?;
            }
            AppEvent::AddNotification(notification_message) => {
                riqi_state.notification.push(notification_message);
                draw_ui(&mut terminal, &calendar, &riqi_state, &app_config)?;
            }
            AppEvent::RemoveNotification(notification_message) => {
                riqi_state
                    .notification
                    .retain(|message| message.id != notification_message.id);
                draw_ui(&mut terminal, &calendar, &riqi_state, &app_config)?;
            }
            _ => {} // 其他按键暂不触发重绘
        }
    }

    // --- 4. 恢复终端 ---
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

// 将渲染逻辑抽离
fn draw_ui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    calendar: &MonthCalendar,
    riqi_state: &RiqiState,
    app_config: &AppConfig,
) -> io::Result<()> {
    terminal.draw(|f| {
        let frame_area = f.area();
        let layout = get_layout(frame_area, app_config.column, app_config.row);
        let month_item = MonthComponent::new(calendar, &layout, &riqi_state, app_config);
        month_item.render(layout.month_calendar.area, f.buffer_mut());
        let bottom_line = BottomLineComponent {
            app_config,
            riqi_state,
        };
        bottom_line.render(layout.bottom_line, f.buffer_mut());

        if matches!(riqi_state.mode, RiqiMode::Goto) {
            draw_goto_panel(riqi_state, f);
        }
    })?;
    Ok(())
}

fn draw_goto_panel(riqi_state: &RiqiState, f: &mut Frame) {
    let goto_panel = GotoPanelComponent {
        year: riqi_state.goto_panel.year.to_string(),
        month: riqi_state.goto_panel.month.to_string(),
        day: riqi_state.goto_panel.day.to_string(),
        cursor: riqi_state.goto_panel.focus_inp as usize,
    };
    // 1. 定义弹出层总大小 (45x8 字符左右)
    let area = f.area();
    let popup_area = area.centered(Constraint::Length(45), Constraint::Length(7));
    // 2. 清除背景并绘制外层边框
    f.render_widget(
        Clear,
        Rect {
            x: popup_area.x - 1,
            y: popup_area.y,
            width: popup_area.width + 2,
            height: popup_area.height,
        },
    );
    goto_panel.render(popup_area, f.buffer_mut());
}
