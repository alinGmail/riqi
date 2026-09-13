mod config;
mod data;
mod events;
mod holiday;
mod state;
mod theme;
mod ui;
mod utils;

use crate::config::model::AppConfig;
use crate::config::xdg::Xdg;
use crate::events::{
    handle_config_mode_key_event, handle_goto_mode_key_event, handle_normal_mode_key_event,
    handle_theme_select_mode_key_event, AppEvent,
};
use crate::holiday::holiday_map::HolidayMap;
use crate::holiday::manager::HolidayUpdateManager;
use crate::holiday::utils::get_ylc_code;
use crate::state::{
    ConfigPanelState, GotoPanelState, NotificationMessage, RiqiMode, ThemeSelectState,
};
use crate::ui::bottom_line_component::BottomLineComponent;
use crate::ui::config_panel_component::ConfigPanelComponent;
use crate::ui::goto_panel_component::GotoPanelComponent;
use crate::ui::notification_component::NotificationComponent;
use crate::ui::theme_select_component::ThemeSelectComponent;
use crate::ui::translate::{get_translate, Language};
use chrono::{Datelike, Local, NaiveDate};
use clap::{arg, Parser};
use color_eyre::Result;
use config::{cli::Args, config_main::get_app_config};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use data::calendar::MonthCalendar;
use env_logger::{Builder, Target};
use log::{debug, info, LevelFilter};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Clear};
use serde::Deserialize;
use state::RiqiState;
use std::collections::{HashMap, HashSet};
use std::io::IsTerminal;
use std::{
    fs::File,
    io::{self, stderr, stdout, Write},
    sync::mpsc,
    thread,
};
use theme::theme_loader::{available_theme_names, load_theme_with_fallback};
use ui::{
    layout::get_layout,
    month_component::{self, MonthComponent},
};

fn setup_logger() {
    // 尝试在缓存目录创建日志文件
    let log_file = Xdg::cache_dir()
        .map(|mut path| {
            // 确保缓存目录存在
            let _ = std::fs::create_dir_all(&path);
            path.push("debug.log");
            File::create(path)
        })
        .and_then(|result| result.ok())
        .map(|file| Box::new(file) as Box<dyn Write + Send>)
        .unwrap_or_else(|| Box::new(io::sink()) as Box<dyn Write + Send>);

    Builder::new()
        .target(Target::Pipe(log_file)) // 输出到文件或 /dev/null
        .filter_level(LevelFilter::Debug) // 设置日志级别
        .format_timestamp(None) // 可选：禁用时间戳
        .is_test(true) // 禁用颜色（避免乱码）
        .init();
}
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    setup_logger();
    color_eyre::install()?;
    // --- 1. 初始化终端 ---
    enable_raw_mode()?;
    // 使用 Box 包装，这样后端就不再关心具体是哪种流
    let writer: Box<dyn Write> = if io::stdout().is_terminal() {
        stdout().execute(EnterAlternateScreen)?;
        Box::new(io::stdout())
    } else {
        stderr().execute(EnterAlternateScreen)?;
        Box::new(io::stderr())
    };

    let backend = CrosstermBackend::new(writer);
    let mut terminal = Terminal::new(backend)?;
    // --- 2. 创建核心事件通道 ---
    let (tx, rx) = mpsc::channel();

    let now = Local::now();
    let mut app_config = get_app_config(args);

    let (theme, theme_warning) = load_theme_with_fallback(&app_config.theme);

    let mut theme_names = available_theme_names();
    theme_names.sort_unstable();

    let mut riqi_state = RiqiState {
        select_day: now.date_naive(),
        today: now.date_naive(),
        theme,
        theme_name: app_config.theme.clone(),
        theme_names,
        mode: RiqiMode::Normal,
        goto_panel: GotoPanelState {
            year: now.year() as u16,
            month: now.month() as u8,
            day: now.day() as u8,
            focus_inp: 0,
        },
        config_panel: ConfigPanelState { focus: 0 },
        theme_select: ThemeSelectState {
            selected: 0,
            original_theme: theme,
        },
        notification: vec![],
    };

    if let Some(message) = theme_warning {
        push_notification_with_timeout(&mut riqi_state, &tx, "theme_fallback".to_string(), message);
    }

    let now = Local::now();
    let mut holiday_map = HolidayMap::new();
    // 记录加载失败的 key({year}_{language}_{country}) 及其消息，以及是否已提示过
    let mut failed_holiday_keys: HashMap<String, String> = HashMap::new();
    let mut notified_holiday_failures: HashSet<String> = HashSet::new();
    let mut calendar = MonthCalendar::new(
        now.year() as u32,
        now.month(),
        now.date_naive(),
        &holiday_map,
        &app_config.language,
        &app_config.country,
    );
    // 事件源 A: 终端输入监听线程 (将 crossterm 事件转发到 mpsc)
    let tx_input = tx.clone();
    thread::spawn(move || loop {
        if event::poll(std::time::Duration::from_millis(500)).unwrap() {
            if let Ok(ev) = event::read() {
                let _ = tx_input.send(AppEvent::TerminalEvent(ev));
            }
        }
    });
    let holiday_manager = HolidayUpdateManager::new(tx.clone());

    if app_config.show_holiday {
        let year = riqi_state.select_day.year();
        let lang = app_config.language.clone();
        let country = app_config.country.clone();
        let source = app_config.source.clone();
        let hm = holiday_manager.clone();

        tokio::spawn(async move {
            for y in [year - 1, year, year + 1] {
                hm.ensure_year(&y.to_string(), &lang, &country, source.clone())
                    .await;
            }
        });
    }

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
                if key.code == KeyCode::Enter {
                    if matches!(riqi_state.mode, RiqiMode::Normal) {
                        disable_raw_mode()?;
                        //
                        if io::stdout().is_terminal() {
                            stdout().execute(LeaveAlternateScreen)?;
                        } else {
                            stderr().execute(LeaveAlternateScreen)?;
                        }
                        print!("{}", riqi_state.select_day.format(&app_config.output));
                        stdout().flush()?;
                        return Ok(());
                    }
                }

                let pre_year = riqi_state.select_day.year();
                let pre_month = riqi_state.select_day.month();
                // 判断是什么mode
                match riqi_state.mode {
                    RiqiMode::Normal => handle_normal_mode_key_event(key, &mut riqi_state),
                    RiqiMode::Goto => handle_goto_mode_key_event(key, &mut riqi_state, tx.clone()),
                    RiqiMode::Config => {
                        handle_config_mode_key_event(key, &mut riqi_state, &mut app_config)
                    }
                    RiqiMode::ThemeSelect => {
                        handle_theme_select_mode_key_event(key, &mut riqi_state)
                    }
                }

                if app_config.show_holiday {
                    // 确保 当前年 / 前一年 / 后一年 数据可用
                    let current_year = riqi_state.select_day.year();
                    let lang = app_config.language.clone();
                    let country = app_config.country.clone();
                    let source = app_config.source.clone();
                    for y in [current_year - 1, current_year, current_year + 1] {
                        let year_str = y.to_string();
                        let key = get_ylc_code(&year_str, &lang, &country);
                        if holiday_map.contains(&key) {
                            continue;
                        }
                        holiday_manager
                            .ensure_year(&year_str, &lang, &country, source.clone())
                            .await;
                    }

                    // 只有当前显示年份加载失败时才提示，且每个 key 只提示一次
                    let current_key = get_ylc_code(
                        &current_year.to_string(),
                        &app_config.language,
                        &app_config.country,
                    );
                    if let Some(message) = failed_holiday_keys.get(&current_key).cloned() {
                        if notified_holiday_failures.insert(current_key.clone()) {
                            push_notification_with_timeout(
                                &mut riqi_state,
                                &tx,
                                current_key,
                                message,
                            );
                        }
                    }
                }

                calendar = MonthCalendar::new(
                    riqi_state.select_day.year() as u32,
                    riqi_state.select_day.month(),
                    riqi_state.select_day,
                    &holiday_map,
                    &app_config.language,
                    &app_config.country,
                );

                draw_ui(&mut terminal, &calendar, &riqi_state, &app_config)?;
            }
            AppEvent::UpdateHoliday(ylc_key, holiday_of_year) => {
                if !holiday_map.upsert(ylc_key, holiday_of_year) {
                    continue;
                }
                calendar = MonthCalendar::new(
                    riqi_state.select_day.year() as u32,
                    riqi_state.select_day.month(),
                    riqi_state.select_day,
                    &holiday_map,
                    &app_config.language,
                    &app_config.country,
                );
                draw_ui(&mut terminal, &calendar, &riqi_state, &app_config)?;
            }
            AppEvent::HolidayLoadFailed(ylc_key, message) => {
                failed_holiday_keys.insert(ylc_key.clone(), message.clone());
                // 仅当失败的是当前显示年份时才提示，且只提示一次
                let current_key = get_ylc_code(
                    &riqi_state.select_day.year().to_string(),
                    &app_config.language,
                    &app_config.country,
                );
                if ylc_key == current_key && notified_holiday_failures.insert(ylc_key.clone()) {
                    push_notification_with_timeout(&mut riqi_state, &tx, ylc_key, message);
                    draw_ui(&mut terminal, &calendar, &riqi_state, &app_config)?;
                }
            }
            AppEvent::AddNotification(notification_message) => {
                riqi_state.notification.push(notification_message);
                draw_ui(&mut terminal, &calendar, &riqi_state, &app_config)?;
            }
            AppEvent::RemoveNotification(notification_message) => {
                info!("in remove notification_message");
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
    if io::stdout().is_terminal() {
        stdout().execute(LeaveAlternateScreen)?;
    } else {
        stderr().execute(LeaveAlternateScreen)?;
    }
    Ok(())
}

// 推送通知，并在 5 秒后自动移除（与 Goto 非法日期提示一致）
fn push_notification_with_timeout(
    riqi_state: &mut RiqiState,
    sender: &mpsc::Sender<AppEvent>,
    id: String,
    message: String,
) {
    riqi_state.notification.push(NotificationMessage {
        id: id.clone(),
        message: message.clone(),
    });
    let sender = sender.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        let _ = sender.send(AppEvent::RemoveNotification(NotificationMessage {
            id,
            message,
        }));
    });
}

// 将渲染逻辑抽离
fn draw_ui<W: io::Write>(
    terminal: &mut Terminal<CrosstermBackend<W>>,
    calendar: &MonthCalendar,
    riqi_state: &RiqiState,
    app_config: &AppConfig,
) -> io::Result<()> {
    terminal.draw(|f| {
        if !app_config.hide_bg {
            f.render_widget(
                Block::default().style(Style::default().bg(riqi_state.theme.bg)),
                f.area(),
            );
        }

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
            draw_goto_panel(riqi_state, app_config, f);
        }

        if matches!(riqi_state.mode, RiqiMode::Config) {
            draw_config_panel(riqi_state, app_config, f);
        }

        if matches!(riqi_state.mode, RiqiMode::ThemeSelect) {
            draw_theme_select_panel(riqi_state, app_config, f);
        }

        if !riqi_state.notification.is_empty() {
            let notification_component = NotificationComponent {
                notifications: &riqi_state.notification,
            };
            notification_component.render(frame_area, f.buffer_mut());
        }
    })?;
    Ok(())
}

fn draw_goto_panel(riqi_state: &RiqiState, app_config: &AppConfig, f: &mut Frame) {
    let language = app_config
        .language
        .parse::<Language>()
        .unwrap_or(Language::EN);
    let translate = get_translate(language);

    let goto_panel = GotoPanelComponent {
        year: riqi_state.goto_panel.year.to_string(),
        month: riqi_state.goto_panel.month.to_string(),
        day: riqi_state.goto_panel.day.to_string(),
        cursor: riqi_state.goto_panel.focus_inp as usize,
        translate,
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

fn draw_config_panel(riqi_state: &RiqiState, app_config: &AppConfig, f: &mut Frame) {
    let language = app_config
        .language
        .parse::<Language>()
        .unwrap_or(Language::EN);
    let translate = get_translate(language);

    let popup_area = f.area().centered(Constraint::Length(40), Constraint::Length(7));
    f.render_widget(
        Clear,
        Rect {
            x: popup_area.x - 1,
            y: popup_area.y,
            width: popup_area.width + 2,
            height: popup_area.height,
        },
    );

    let config_panel = ConfigPanelComponent {
        translate,
        theme: &riqi_state.theme,
        theme_name: &riqi_state.theme_name,
        show_lunar: app_config.show_lunar,
        show_holiday: app_config.show_holiday,
        focus: riqi_state.config_panel.focus,
    };
    config_panel.render(popup_area, f.buffer_mut());
}

fn draw_theme_select_panel(riqi_state: &RiqiState, app_config: &AppConfig, f: &mut Frame) {
    let language = app_config
        .language
        .parse::<Language>()
        .unwrap_or(Language::EN);
    let translate = get_translate(language);

    let height = riqi_state.theme_names.len() as u16 + 4;
    let popup_area = f.area().centered(Constraint::Length(40), Constraint::Length(height));
    f.render_widget(
        Clear,
        Rect {
            x: popup_area.x - 1,
            y: popup_area.y,
            width: popup_area.width + 2,
            height: popup_area.height,
        },
    );

    let theme_select = ThemeSelectComponent {
        translate,
        theme: &riqi_state.theme,
        theme_names: &riqi_state.theme_names,
        selected: riqi_state.theme_select.selected,
    };
    theme_select.render(popup_area, f.buffer_mut());
}
