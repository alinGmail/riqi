use ratatui::prelude::Buffer;
use ratatui::widgets::{BorderType, Widget};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};
use ratatui::layout::Alignment;
use ratatui::widgets::BorderType::Rounded;
use crate::ui::translate::Translate;

pub struct GotoPanelComponent<'a> {
    pub year: String,
    pub month: String,
    pub day: String,
    pub cursor: usize,
    pub translate: &'a Translate<'a>,
}

impl<'a> Widget for GotoPanelComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let outer_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(format!(" {} ", self.translate.goto_to_date))
            .title_alignment(Alignment::Center);

        let inner_area = outer_block.inner(area);
        outer_block.render(area, buf);

        let rows = Layout::vertical([
            Constraint::Length(4),
            Constraint::Length(1),
        ]).split(inner_area);

        let cols = Layout::horizontal([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ]).split(rows[0]);

        let data = [
            (self.translate.year, &self.year),
            (self.translate.month, &self.month),
            (self.translate.day, &self.day),
        ];

        for (i, (label, value)) in data.iter().enumerate() {
            let field_rows = Layout::vertical([
                Constraint::Length(1),
                Constraint::Length(3),
            ]).split(cols[i]);

            Paragraph::new(*label)
                .alignment(Alignment::Center)
                .render(field_rows[0], buf);

            let style = if i == self.cursor {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            Paragraph::new(value.as_str())
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(style))
                .render(field_rows[1], buf);
        }

        Paragraph::new(self.translate.goto_help)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray))
            .render(rows[1], buf);
    }
}
