use ratatui::{style::Style, widgets::Widget};

use crate::{data::calendar::CalendarDay, state::RiqiState};

use super::utils::get_style_from_config;

struct DayCell<'a> {
    day_data: &'a CalendarDay,
    riqi_state: &'a RiqiState,
}

impl<'a> DayCell<'a> {
    pub fn new(day_data: &'a CalendarDay, riqi_state: &'a RiqiState) -> Self {
        DayCell { day_data, riqi_state }
    }

    pub fn get_day_item_style(&self, is_holiday: bool) -> Style {
        let mut style = self.riqi_state.theme.get_default_style();
        if is_holiday {
            // 周六日使用节假日颜色
            if self.day_data.is_current_month {
                style = get_style_from_config(Some(style), self.riqi_state.theme.holiday);
            } else {
                style = get_style_from_config(Some(style), self.riqi_state.theme.holiday_adjacent);
            }
        } else {
            // 工作日使用工作颜色
            if self.day_data.is_current_month {
                style = get_style_from_config(Some(style), self.riqi_state.theme.workday);
            } else {
                style = get_style_from_config(Some(style), self.riqi_state.theme.workday_adjacent);
            }
        }

        if self.day_data.is_today {
            style = style.bold();
        }

        if self.day_data.is_select_day {
            style = get_style_from_config(Some(style), self.riqi_state.theme.focus_day)
        }

        style
    }
}

impl Widget for DayCell<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        todo!()
    }
}
