use chrono::{Datelike, Local};
use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

mod data;
use data::MonthCalendar;
mod month_render;
use month_render::render_day_item;
mod theme;

fn main() -> Result<()> {
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
    let calendar = MonthCalendar::new(now.year() as u32, now.month());

    loop {
        terminal.draw(|frame| {
            let size = frame.area();

            // 创建主框架
            let main_block = Block::default().title("日历").borders(Borders::ALL);
            frame.render_widget(main_block, size);

            // 为日历创建内部区域
            let calendar_area = Rect::new(size.x + 1, size.y + 1, size.width - 2, size.height - 2);

            // 创建7列的布局（一周7天）
            let constraints = vec![Constraint::Percentage(100 / 7); 7];
            let horizontal_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints)
                .split(calendar_area);

            // 渲染星期标题
            let weekdays = ["日", "一", "二", "三", "四", "五", "六"];
            for (i, &day) in weekdays.iter().enumerate() {
                let line_txt = Line::from(day).centered();
                let day_block = Block::default().title(day).borders(Borders::ALL);
                frame.render_widget(line_txt, horizontal_layout[i]);
            }

            // 计算每个日期块的高度
            // let day_height = (calendar_area.height - 3) / 6; // 减去标题行的高度
            let day_height = 3;

            // 渲染日期
            for (week_idx, week) in calendar.day_data.iter().enumerate() {
                for (day_idx, day) in week.iter().enumerate() {
                    let day_area = Rect::new(
                        horizontal_layout[day_idx].x,
                        calendar_area.y + 3 + (week_idx as u16 * day_height),
                        horizontal_layout[day_idx].width,
                        day_height,
                    );
                    render_day_item(frame, day, day_area);
                }
            }
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }
    Ok(())
}
