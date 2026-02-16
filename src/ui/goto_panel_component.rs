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

pub struct GotoPanelComponent {
    pub year: String,
    pub month: String,
    pub day: String,
    pub cursor: usize, // 0: Year, 1: Month, 2: Day
}

impl Widget for GotoPanelComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // 1. Define the layout: Inner area of the popup
        let outer_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" GOTO TO DATE")
            .title_alignment(Alignment::Center);

        let inner_area = outer_block.inner(area);
        outer_block.render(area, buf);

        // 2. Split into rows: input area and help text
        let rows = Layout::vertical([
            Constraint::Length(4),
            Constraint::Length(1),
        ]).split(inner_area);

        // 3. Split input area into 3 columns
        let cols = Layout::horizontal([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ]).split(rows[0]);

        let data = [
            ("年", &self.year),
            ("月", &self.month),
            ("日", &self.day),
        ];

        for (i, (label, value)) in data.iter().enumerate() {
            // Split each column into Title (1 row) and Box (3 rows)
            let field_rows = Layout::vertical([
                Constraint::Length(1),
                Constraint::Length(3),
            ]).split(cols[i]);

            // Render Title
            Paragraph::new(*label)
                .alignment(Alignment::Center)
                .render(field_rows[0], buf);

            // Render Value Box (Highlight border if focused)
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

        // Render help text
        Paragraph::new("h,l:左右导航;j,k:加或减;enter:选择日期")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray))
            .render(rows[1], buf);
    }
}
