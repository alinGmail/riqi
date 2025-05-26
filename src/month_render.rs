use ratatui::{
    layout::{Constraint, Layout, Rect}, style::{Color, Style}, text::Line, widgets::{Block, Borders}, Frame
};

use crate::data::CalendarDay;
use crate::theme::Theme;



pub fn render_day_item<'a>(frame: &mut Frame, day: &'a CalendarDay, rect: Rect) {
    // 创建一个layout
    let rows_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Min(1), Constraint::Min(1)])
        .split(rect);

    let style = if day.is_current_month {
        Style::default().fg(Color::White)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let day_block = Block::default()
        .title(day.day.to_string())
        .borders(Borders::ALL)
        .style(style);

    frame.render_widget(day_block, rows_layout[1]);
}

#[derive(Debug,Clone)]
struct DayItem<'a> {
    label: Line<'a>,
    theme: Theme,
}

const BLUE: Theme = Theme {
    text: Color::Rgb(16, 24, 48),
    background: Color::Rgb(48, 72, 144),
    highlight: Color::Rgb(64, 96, 192),
    shadow: Color::Rgb(32, 48, 96),
};

/// A button with a label that can be themed.
impl<'a> DayItem<'a> {
    pub fn new<T: Into<Line<'a>>>(label: T) -> Self {
        DayItem {
            label: label.into(),
            theme: BLUE,
        }
    }

    pub const fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

}


