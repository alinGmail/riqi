use chrono::{Datelike, Duration, Local};
use clap::Parser;
use cli::Args;
use color_eyre::Result;
use component::{
    bottom_line_component::BottomLineComponent, month_component::MonthComponent,
    utils::get_style_from_config,
};
use config::config_init::get_default_config;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use env_logger::{Builder, Target};
use i18n::get_month_til_i18n;
use layout::get_layout;
use log::LevelFilter;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Flex, Layout},
    style::Style,
    text::Line,
    widgets::Widget,
    Terminal,
};
use state::RiqiState;
use std::{collections::HashMap, fs::File, io};
use theme::BLUE;
use utils::add_months_safe;

mod data;
use data::MonthCalendar;
mod theme;

mod holiday;
use holiday::load_holidays_file;
mod holiday_data;
use holiday_data::parse_holidays;
use holiday_data::HolidayMap;
mod state;
mod utils;

mod cli;
mod component;
mod config;
mod i18n;
mod layout;
mod locale;
mod lunar;

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

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // 运行应用
    let result = run(&mut terminal);

    // 清理终端
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let args = Args::parse();
    // 获取当前日期
    let now = Local::now();
    let mut holiday_map: HolidayMap = HashMap::new();
    let mut calendar = MonthCalendar::new(now.year() as u32, now.month());

    let mut config = get_default_config();

    if let Some(language) = args.language {
        config.language = language;
    }

    if let Some(country) = args.country {
        config.country = country;
    }

    let file_str = load_holidays_file(&config.language, &config.country)?;

    match parse_holidays(&file_str) {
        Ok(holiday_response) => {
            log::debug!(
                "成功解析假期数据，共 {} 个假期",
                holiday_response.holidays.len()
            );
            holiday_response.add_to_holiday_map(&mut holiday_map, "cn", "zh", "2025");
        }
        Err(e) => {
            log::error!("解析假期数据失败: {}", e);
        }
    }

    let mut riqi_state = RiqiState {
        select_day: now.date_naive(),
        holiday_map: &holiday_map,
        today: now.date_naive(),
        config: &config,
        theme: &BLUE,
    };

    loop {
        terminal.draw(|frame| {
            let size = frame.area();
            let [til_area, _, calendar_area, bottom_line_area] = get_layout(size);

            let month_til_i18n_str = get_month_til_i18n(
                calendar.year as i32,
                calendar.month,
                &riqi_state.config.language,
            );

            let bottom_line = BottomLineComponent {
                riqi_state: &riqi_state,
            };
            bottom_line.render(bottom_line_area, frame.buffer_mut());

            let month_til_component =
                Line::from(month_til_i18n_str)
                    .centered()
                    .style(get_style_from_config(
                        Some(Style::default()),
                        riqi_state.theme.month_til,
                    ));
            month_til_component.render(til_area, frame.buffer_mut());

            //
            let layout = Layout::horizontal([Constraint::Length(82)])
                .flex(Flex::Center)
                .split(calendar_area);

            let month_component = MonthComponent {
                data: &calendar,
                riqi_state: &riqi_state,
                day_gap: 2,
            };
            month_component.render(layout[0], frame.buffer_mut());
        })?;

        if let Event::Key(key) = event::read()? {
            if key.is_release() {
                continue;
            }
            if key.code == KeyCode::Char('q') {
                break;
            }
            if key.code == KeyCode::Char('j') {
                // go to next week
                riqi_state.select_day += Duration::weeks(1);
            }
            if key.code == KeyCode::Char('k') {
                // go to pre week
                riqi_state.select_day += Duration::weeks(-1);
            }
            if key.code == KeyCode::Char('h') {
                // go to pre day
                riqi_state.select_day += Duration::days(-1);
            }
            if key.code == KeyCode::Char('l') {
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
