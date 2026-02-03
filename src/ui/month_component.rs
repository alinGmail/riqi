use super::{layout::RiqiLayout, week_row};
use crate::config::model::AppConfig;
use crate::ui::translate::{get_month_til_i18n, weekday_name_i18n};
use crate::ui::utils::get_style_from_config;
use crate::{data::calendar::MonthCalendar, state::RiqiState};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::{layout::Rect, widgets::Widget};

#[derive(Debug)]
pub struct MonthComponent<'a> {
    pub data: &'a MonthCalendar,
    pub riqi_layout: &'a RiqiLayout,
    pub riqi_state: &'a RiqiState,
    pub app_config: &'a AppConfig,
}

impl<'a> MonthComponent<'a> {
    pub fn new(
        data: &'a MonthCalendar,
        riqi_layout: &'a RiqiLayout,
        riqi_state: &'a RiqiState,
        app_config: &'a AppConfig,
    ) -> Self {
        MonthComponent {
            data,
            riqi_layout,
            riqi_state,
            app_config,
        }
    }

    pub fn render_month_til(&self, buf: &mut ratatui::prelude::Buffer) {
        let month_til_i18n_str = get_month_til_i18n(
            self.data.year as i32,
            self.data.month,
            &self.app_config.language,
        );
        let month_til_component =
            Line::from(month_til_i18n_str)
                .centered()
                .style(get_style_from_config(
                    Some(Style::default()),
                    self.riqi_state.theme.month_til,
                ));
        month_til_component.render(self.riqi_layout.month_calendar.title, buf);
    }

    pub fn render_month_header_row(&self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let day_item_width = self.riqi_layout.month_calendar.day_item_column as u16;
        // 渲染星期标题
        for i in 0..7 {
            let day = weekday_name_i18n(i, &self.app_config.language);
            let line_txt = Line::from(day.clone())
                .centered()
                .style(get_style_from_config(
                    Some(self.riqi_state.theme.get_default_style()),
                    self.riqi_state.theme.month_head,
                ));
            line_txt.render(
                Rect::new(
                    area.left()
                        + (day_item_width + self.riqi_layout.month_calendar.day_gap as u16)
                            * i as u16,
                    area.top() + 2,
                    day_item_width,
                    1,
                ),
                buf,
            );
        }
    }
}

impl<'a> Widget for MonthComponent<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let day_height = self.riqi_layout.month_calendar.day_item_row as u16;
        let day_width = self.riqi_layout.month_calendar.day_item_column as u16;
        self.render_month_til(buf);
        self.render_month_header_row(area, buf);
        for (week_idx, week) in self.data.day_data.iter().enumerate() {
            let week_row_area = Rect::new(
                area.left(),
                area.top() + 4 + day_height * week_idx as u16,
                area.width,
                day_height,
            );
            let week_row_item =
                week_row::WeekRow::new(week, self.riqi_state, self.riqi_layout, self.app_config);
            week_row_item.render(week_row_area, buf);
        }
    }
}
