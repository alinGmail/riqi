use chrono::{Datelike, NaiveDate};
use ratatui::widgets::Widget;

use crate::data::calendar::MonthCalendar;

use super::{layout::RiqiLayout, week_row};


#[derive(Debug)]
pub struct MonthComponent<'a> {
    pub data: &'a MonthCalendar,
    pub riqi_layout: &'a RiqiLayout,
    pub day_gap: u16,

}


impl<'a> Widget for MonthComponent<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let day_height = self.riqi_layout.month_calendar.day_item_row as u16;
        
        for (week_idx, week) in self.data.day_data.iter().enumerate() {
            // let week_row_item = week_row::WeekRow::new();


        }
    }
}
