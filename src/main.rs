mod config;
mod data;
mod holiday;
mod state;
mod theme;
mod ui;

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
use ratatui::prelude::*;
use serde::Deserialize;
use state::RiqiState;
use std::{
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

// 统一的事件枚举：合并了 UI 事件和业务数据事件
enum AppEvent {
    Quit,
    TerminalEvent(Event),
}

#[tokio::main]
async fn main() -> Result<()> {
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
    let appConfig = get_app_config(args);

    let theme = load_theme_from_file("resources/theme/ningmen.toml").expect("主题加载失败");
    let mut riqi_state = RiqiState {
        select_day: now.date_naive(),
        today: now.date_naive(),
        theme,
    };

    let now = Local::now();
    let mut calendar = MonthCalendar::new(now.year() as u32, now.month(), now.date_naive());

    // 事件源 A: 终端输入监听线程 (将 crossterm 事件转发到 mpsc)
    let tx_input = tx.clone();
    thread::spawn(move || loop {
        if event::poll(std::time::Duration::from_millis(500)).unwrap() {
            if let Ok(ev) = event::read() {
                if let Event::Key(key) = ev {
                    if key.code == KeyCode::Char('q') {
                        let _ = tx_input.send(AppEvent::Quit);
                        break;
                    }
                }
                let _ = tx_input.send(AppEvent::TerminalEvent(ev));
            }
        }
    });

    // 初始手动触发一次渲染（显示“加载中”）
    draw_ui(&mut terminal, &calendar, &riqi_state)?;

    loop {
        // 【关键】阻塞式接收：没有事件时，程序会停留在此处，不消耗 CPU
        match rx.recv().unwrap() {
            AppEvent::Quit => break,
            AppEvent::TerminalEvent(Event::Resize(_, _)) => {
                // 窗口大小改变，触发重绘
                draw_ui(&mut terminal, &calendar, &riqi_state)?;
            }
            AppEvent::TerminalEvent(Event::Key(key)) => {
                if key.is_release() {
                    continue;
                }
                if key.code == KeyCode::Char('j') || key.code == KeyCode::Down {
                    riqi_state.select_day += Duration::weeks(1);
                    calendar = MonthCalendar::new(
                        riqi_state.select_day.year() as u32,
                        riqi_state.select_day.month(),
                        riqi_state.select_day,
                    );
                }
                if key.code == KeyCode::Char('k') || key.code == KeyCode::Up {
                    riqi_state.select_day += Duration::weeks(-1);
                    calendar = MonthCalendar::new(
                        riqi_state.select_day.year() as u32,
                        riqi_state.select_day.month(),
                        riqi_state.select_day,
                    );
                }
                if key.code == KeyCode::Char('h') || key.code == KeyCode::Left {
                    riqi_state.select_day += Duration::days(-1);
                    calendar = MonthCalendar::new(
                        riqi_state.select_day.year() as u32,
                        riqi_state.select_day.month(),
                        riqi_state.select_day,
                    );
                }
                if key.code == KeyCode::Char('l') || key.code == KeyCode::Right {
                    riqi_state.select_day += Duration::days(1);
                    calendar = MonthCalendar::new(
                        riqi_state.select_day.year() as u32,
                        riqi_state.select_day.month(),
                        riqi_state.select_day,
                    );
                }
                draw_ui(&mut terminal, &calendar, &riqi_state)?;
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
) -> io::Result<()> {
    terminal.draw(|f| {
        let frame_area = f.area();
        let layout = get_layout(frame_area, None, None);
        // let data =
        let month_item = MonthComponent::new(calendar, &layout, &riqi_state);
        month_item.render(layout.month_calendar.area, f.buffer_mut());
    })?;
    Ok(())
}
