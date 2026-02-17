use crate::state::NotificationMessage;
use ratatui::prelude::Buffer;
use ratatui::widgets::Widget;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub struct NotificationComponent<'a> {
    pub notifications: &'a [NotificationMessage],
}

impl<'a> Widget for NotificationComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let max_width = 20;
        let notification_height = 3;
        let spacing = 0;

        for (i, notification) in self.notifications.iter().enumerate() {
            let y_offset = i as u16 * (notification_height + spacing);
            
            if area.y + y_offset + notification_height > area.bottom() {
                break;
            }

            let notification_area = Rect {
                x: area.right().saturating_sub(max_width + 1),
                y: area.y + y_offset + 1,
                width: max_width.min(area.width),
                height: notification_height,
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Red))
                .style(Style::default());

            let inner_area = block.inner(notification_area);
            block.render(notification_area, buf);

            let lines: Vec<String> = notification
                .message
                .chars()
                .collect::<Vec<char>>()
                .chunks(inner_area.width as usize)
                .map(|chunk| chunk.iter().collect())
                .collect();

            let text = lines.join("\n");
            let paragraph = Paragraph::new(text).style(Style::default().fg(Color::Red));
            paragraph.render(inner_area, buf);
        }
    }
}
