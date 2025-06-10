use ratatui::{buffer::Buffer, layout::Rect, text::Line, widgets::Widget};

use crate::{data::MonthCalendar, i18n::weekday_name_i18n, state::RiqiState, theme::BLUE};

use super::{day_component::render_day_item, utils::get_style_from_config};

pub struct MonthComponent<'a> {
    pub data: &'a MonthCalendar,
    pub riqi_state: &'a RiqiState<'a>,
    pub day_gap: u16,
}

impl<'a> MonthComponent<'a> {
    fn normal_render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {
        let day_item_width = (area.width - 6) / 7;
        // 渲染星期标题
        for i in 0..7 {
            let day = weekday_name_i18n(i, &self.riqi_state.config.language);
            let line_txt = Line::from(day.clone())
                .centered()
                .style(get_style_from_config(
                    Some(self.riqi_state.theme.get_default_style()),
                    BLUE.month_head,
                ));
            line_txt.render(
                Rect::new(
                    area.left() + (day_item_width + self.day_gap) * i as u16,
                    area.top(),
                    day_item_width,
                    1,
                ),
                buf,
            );
        }
        let day_height = (area.height - 2) / 6;
        for (week_idx, week) in self.data.day_data.iter().enumerate() {
            for (day_idx, day) in week.iter().enumerate() {
                let day_area = Rect::new(
                    area.left() + day_idx as u16 * (day_item_width + self.day_gap),
                    area.top() + 2 + day_height * week_idx as u16,
                    day_item_width,
                    day_height,
                );
                render_day_item(buf, day, day_area, self.riqi_state);
            }
        }
    }
}

impl<'a> Widget for MonthComponent<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {
        self.normal_render(area, buf);
    }
}
