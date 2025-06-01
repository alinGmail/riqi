use chrono::{Datelike, Duration, Local, NaiveDate};
use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use env_logger::{Builder, Target};
use log::LevelFilter;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
    Terminal,
};
use state::RiqiState;
use std::{fs::File, io};
use theme::{Theme, BLUE};

mod data;
use data::MonthCalendar;
mod month_render;
use month_render::render_day_item;
mod theme;

mod month_component;
use month_component::MonthComponent;

mod state;

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
    setup_logger();
    log::debug!("aaaabbb-ccc");
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
    // 获取当前日期
    let now = Local::now();
    let mut riqi_state = RiqiState {
        select_day: now.date_naive(),
    };
    let mut calendar = MonthCalendar::new(now.year() as u32, now.month());

    loop {
        terminal.draw(|frame| {
            let size = frame.area();

            // 创建主框架
            let main_block = Block::default().title("日历").borders(Borders::ALL);
            frame.render_widget(main_block, size);

            // 为日历创建内部区域
            let calendar_area = Rect::new(size.x + 1, size.y + 1, size.width - 2, size.height - 2);

            //
            let layout = Layout::horizontal([Constraint::Length(82)])
                .flex(Flex::Center)
                .split(calendar_area);

            let month_component = MonthComponent {
                data: &calendar,
                riqi_state: &riqi_state,
                day_gap: 2,
                theme: BLUE,
            };
            month_component.render(layout[0], frame.buffer_mut());
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
            if key.code == KeyCode::Char('j') {
                // go to next week
                riqi_state.select_day = riqi_state.select_day + Duration::weeks(1);
            }
            if key.code == KeyCode::Char('k') {
                // go to pre week
                riqi_state.select_day = riqi_state.select_day + Duration::weeks(-1);
            }
            if key.code == KeyCode::Char('h') {
                // go to pre day
                riqi_state.select_day = riqi_state.select_day + Duration::days(-1);
            }
            if key.code == KeyCode::Char('l') {
                // go to pre day
                riqi_state.select_day = riqi_state.select_day + Duration::days(1);
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
