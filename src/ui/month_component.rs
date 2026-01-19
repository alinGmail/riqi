use chrono::{Datelike, NaiveDate};
use ratatui::widgets::Widget;

use crate::{data::calendar::MonthCalendar, state::RiqiState};

use super::{layout::RiqiLayout, week_row};

#[derive(Debug)]
pub struct MonthComponent<'a> {
    pub data: &'a MonthCalendar,
    pub riqi_layout: &'a RiqiLayout,
    pub riqi_state: &'a RiqiState,
}

impl<'a> MonthComponent<'a> {
    pub fn new(
        data: &'a MonthCalendar,
        riqi_layout: &'a RiqiLayout,
        riqi_state: &'a RiqiState,
    ) -> Self {
        MonthComponent {
            data,
            riqi_layout,
            riqi_state,
        }
    }
}

impl<'a> Widget for MonthComponent<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let day_height = self.riqi_layout.month_calendar.day_item_row as u16;
        for (week_idx, week) in self.data.day_data.iter().enumerate() {
            let week_row_item = week_row::WeekRow::new(week, self.riqi_state, self.riqi_layout);
            week_row_item.render(area, buf);
        }
    }
}
