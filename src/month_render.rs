use clap::builder::Str;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, StatefulWidget, Widget, Wrap},
    Frame,
};

use crate::data::CalendarDay;
use crate::theme::Theme;

pub fn render_day_item<'a>(buffer: &mut Buffer, day: &'a CalendarDay, rect: Rect) {
    let day_item = CnDayItem::new(day.day);

    StatefulWidget::render(
        day_item,
        rect,
        buffer,
        &mut DayItemState { selected: false },
    );
}

#[derive(Debug, Clone)]
struct DayItemState {
    selected: bool,
}

#[derive(Debug, Clone)]
struct DayItem {
    day: u32,
    theme: Theme,
}

#[derive(Debug, Clone)]
struct CnDayItem {
    day: u32,
    theme: Theme,
}

const BLUE: Theme = Theme {
    text: Color::Rgb(16, 24, 48),
    background: Color::Rgb(48, 72, 144),
    highlight: Color::Rgb(64, 96, 192),
    shadow: Color::Rgb(32, 48, 96),
    holi_day: Color::Rgb(233, 101, 165),
    work_day: Color::Rgb(177, 242, 167),
    focus_day: Color::Rgb(32, 48, 96),
};

/// A button with a label that can be themed.
impl DayItem {
    pub fn new(day: u32) -> Self {
        DayItem { day, theme: BLUE }
    }

    pub const fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}

/// A button with a label that can be themed.
impl CnDayItem {
    pub fn new(day: u32) -> Self {
        CnDayItem { day, theme: BLUE }
    }

    pub const fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn render_content_3row6col(self, area: Rect, buf: &mut Buffer) {
        log::debug!("{:?}", area);
        let line = Line::from(self.day.to_string()).style(Style::default().fg(BLUE.holi_day));
        let holiday = Line::from("中秋节")
            .centered()
            .style(Style::default().fg(BLUE.holi_day));
        line.render(
            Rect {
                x: area.left() + 1,
                y: area.top(),
                width: 6,
                height: 1,
            },
            buf,
        );
        holiday.render(
            Rect {
                x: area.left() + 1,
                y: area.top() + 2,
                width: 6,
                height: 1,
            },
            buf,
        );
    }

    pub fn render_content_2row6col(self, area: Rect, buf: &mut Buffer) {
        let line = Line::from(self.day.to_string()).style(Style::default());
        let holiday = Line::from("中秋节").centered().style(Style::default());
        line.render(
            Rect {
                x: area.left() + 1,
                y: area.top(),
                width: 6,
                height: 1,
            },
            buf,
        );
        holiday.render(
            Rect {
                x: area.left() + 1,
                y: area.top() + 1,
                width: 6,
                height: 1,
            },
            buf,
        );
    }
}

impl StatefulWidget for CnDayItem {
    type State = DayItemState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(BLUE.holi_day));
        let inner_area = block.inner(area);
        block.render(area, buf);
        self.render_content_3row6col(inner_area, buf);
    }
}

impl StatefulWidget for DayItem {
    type State = DayItemState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        // 1. 首先渲染Block
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded);
        let line =
            Line::from(self.day.to_string()).style(Style::default().fg(self.theme.highlight));

        block.render(area, buf);
        line.render(
            Rect {
                x: area.left() + 1,
                y: area.top() + 1,
                width: 2,
                height: 1,
            },
            buf,
        );
        let holiday_text = Line::from("Juneteenth National Independence Day");
        let paragraph = Paragraph::new(holiday_text)
            .wrap(Wrap { trim: true }) // 启用自动换行
            .style(Style::default());
        paragraph.render(
            Rect {
                x: area.left() + 1,
                y: area.top() + 2,
                width: 6,
                height: 2,
            },
            buf,
        );
    }
}
