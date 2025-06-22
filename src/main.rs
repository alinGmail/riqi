use chrono::{Datelike, Duration, Local};
use clap::Parser;
use cli::Args;
use color_eyre::Result;
use component::{bottom_line_component::BottomLineComponent, month_component::MonthComponent};
use config::{
    config_init::{get_config, get_system_language_country},
    file_config_loader::load_file_config,
};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use env_logger::{Builder, Target};
use layout::get_layout;
use log::LevelFilter;
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Widget},
    Terminal,
};
use state::RiqiState;
use std::{collections::HashMap, fs::File, io};
use theme::BLUE;
use types::{calendar::MonthCalendar, holiday::HolidayMap};
use utils::add_months_safe;

mod holiday;
mod theme;
use holiday::{get_holiday_code, load_holidays};
mod state;
mod utils;

mod cli;
mod component;
mod config;
mod layout;
mod lunar;
mod translate;
mod types;

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

fn main() -> Result<()> {
    // 处理 JSON 文件
    // if let Err(e) = riqi::json_processor::process_holiday_json() {
    //    eprintln!("处理 JSON 文件时出错: {}", e);
    //}
    setup_logger();

    log::debug!("start");

    // 设置终端
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;

    // 设置全局 panic hook
    std::panic::set_hook(Box::new(|panic_info| {
        let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s
        } else {
            "Unknown panic"
        };

        // 记录 panic 信息到日志
        log::error!("PANIC: {}", msg);

        // 可选：打印 panic 位置（文件 + 行号）
        if let Some(location) = panic_info.location() {
            log::error!(
                "Panic occurred in file '{}' at line {}",
                location.file(),
                location.line()
            );
        }
    }));

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // 运行应用
    let result = run(&mut terminal);
    if let Err(e) = result {
        log::error!("Operation failed: {}", e); // 记录错误
        return Err(e.into());
    }
    // 清理终端
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let file_config = load_file_config();
    log::debug!("file_config {:?}", file_config);
    let args = Args::parse();

    let (sys_language, sys_country) = get_system_language_country();
    log::debug!("sys_language {:?}", sys_language);
    log::debug!("sys_country {:?}", sys_country);

    // 获取当前日期
    let now = Local::now();
    let mut holiday_map: HolidayMap = HashMap::new();
    let mut calendar = MonthCalendar::new(now.year() as u32, now.month());

    let config = get_config(&sys_language, &sys_country, &file_config, &args);

    log::debug!("init config {:?}", config);

    let holiday_code_result = get_holiday_code(true, &config.country, &config.language);
    load_holidays(
        holiday_code_result,
        &mut holiday_map,
        &now.year().to_string(),
    );

    let mut riqi_state = RiqiState {
        select_day: now.date_naive(),
        holiday_map: &holiday_map,
        today: now.date_naive(),
        config: &config,
        theme: &BLUE,
    };

    loop {
        terminal.draw(|frame| {
            // 2. 创建一个填充背景色的 `Block`
            // 深灰色背景

            // 3. 渲染 `Block` 到整个窗口
            frame.render_widget(
                Block::default().style(Style::default().bg(Color::Rgb(30, 30, 30))),
                frame.area(),
            );

            let size = frame.area();
            let riqi_layout = get_layout(size, riqi_state.config);

            let bottom_line = BottomLineComponent {
                riqi_state: &riqi_state,
            };
            bottom_line.render(riqi_layout.bottom_line, frame.buffer_mut());

            let month_component = MonthComponent {
                data: &calendar,
                riqi_state: &riqi_state,
                day_gap: 1,
                riqi_layout: &riqi_layout,
            };
            month_component.render(riqi_layout.month_calendar.area, frame.buffer_mut());
        })?;

        if let Event::Key(key) = event::read()? {
            if key.is_release() {
                continue;
            }
            if key.code == KeyCode::Char('q') {
                break;
            }
            if key.code == KeyCode::Char('j') || key.code == KeyCode::Down {
                // go to next week
                riqi_state.select_day += Duration::weeks(1);
            }
            if key.code == KeyCode::Char('k') || key.code == KeyCode::Up {
                // go to pre week
                riqi_state.select_day += Duration::weeks(-1);
            }
            if key.code == KeyCode::Char('h') || key.code == KeyCode::Left {
                // go to pre day
                riqi_state.select_day += Duration::days(-1);
            }
            if key.code == KeyCode::Char('l') || key.code == KeyCode::Right {
                // go to pre day
                riqi_state.select_day += Duration::days(1);
            }
            if key.code == KeyCode::Char('d') {
                // go to next month
                riqi_state.select_day = add_months_safe(riqi_state.select_day, 1);
            }

            if key.code == KeyCode::Char('u') {
                // go to pre month
                riqi_state.select_day = add_months_safe(riqi_state.select_day, -1);
            }

            if key.code == KeyCode::Char('y') {
                // go to pre year
                riqi_state.select_day = add_months_safe(riqi_state.select_day, -12);
            }

            if key.code == KeyCode::Char('x') {
                // go to next year
                riqi_state.select_day = add_months_safe(riqi_state.select_day, 12);
            }

            if key.code == KeyCode::Char('t') {
                // got back to today
                riqi_state.select_day = now.date_naive();
            }

            if riqi_state.select_day.year() as u32 != calendar.year
                || riqi_state.select_day.month() != calendar.month
            {
                calendar = MonthCalendar::new(
                    riqi_state.select_day.year() as u32,
                    riqi_state.select_day.month(),
                );
            }
        }
    }
    Ok(())
}
