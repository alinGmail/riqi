use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Widget},
};

use crate::{data::MonthCalendar, month_render::render_day_item, state::RiqiState, theme::Theme};

pub struct MonthComponent<'a> {
    pub data: &'a MonthCalendar,
    pub riqi_state: &'a RiqiState<'a>,
    pub day_gap: u16,
    pub theme: Theme,
}

impl<'a> Widget for MonthComponent<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {
        // 渲染星期标题
        let weekdays = ["日", "一", "二", "三", "四", "五", "六"];
        for (i, &day) in weekdays.iter().enumerate() {
            let line_txt = Line::from(day)
                .centered()
                .style(Style::default().fg(self.theme.holi_day));
            let day_block = Block::default().title(day).borders(Borders::ALL);
            line_txt.render(
                Rect::new(area.left() + 12 * i as u16, area.top(), 12, 1),
                buf,
            );
        }
        // todo
        let day_height = 5;
        for (week_idx, week) in self.data.day_data.iter().enumerate() {
            for (day_idx, day) in week.iter().enumerate() {
                let day_area = Rect::new(
                    area.left() + day_idx as u16 * (10 + self.day_gap),
                    area.top() + 3 + day_height * week_idx as u16,
                    10,
                    day_height,
                );
                render_day_item(buf, day, day_area, self.riqi_state);
            }
        }
    }
}
