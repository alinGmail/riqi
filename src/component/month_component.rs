use super::{day_component::render_day_item, utils::get_style_from_config};
use crate::layout_struct::RiqiLayout;
use crate::translate::get_month_til_i18n;
use crate::{data::MonthCalendar, state::RiqiState, theme::BLUE, translate::weekday_name_i18n};
use ratatui::prelude::Style;
use ratatui::{buffer::Buffer, layout::Rect, text::Line, widgets::Widget};

pub struct MonthComponent<'a> {
    pub data: &'a MonthCalendar,
    pub riqi_state: &'a RiqiState<'a>,
    pub day_gap: u16,
    pub riqi_layout: &'a RiqiLayout,
}

impl<'a> MonthComponent<'a> {
    fn normal_render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {
        let month_til_i18n_str = get_month_til_i18n(
            self.data.year as i32,
            self.data.month,
            &self.riqi_state.config.language,
        );
        let month_til_component =
            Line::from(month_til_i18n_str)
                .centered()
                .style(get_style_from_config(
                    Some(Style::default()),
                    self.riqi_state.theme.month_til,
                ));
        month_til_component.render(self.riqi_layout.month_calendar.title, buf);

        let day_item_width = self.riqi_layout.month_calendar.day_item_column as u16;
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
                    area.top() + 2,
                    day_item_width as u16,
                    1,
                ),
                buf,
            );
        }
        let day_height = self.riqi_layout.month_calendar.day_item_row as u16;
        for (week_idx, week) in self.data.day_data.iter().enumerate() {
            for (day_idx, day) in week.iter().enumerate() {
                let day_area = Rect::new(
                    area.left() + day_idx as u16 * (day_item_width + self.day_gap),
                    area.top() + 4 + day_height * week_idx as u16,
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
