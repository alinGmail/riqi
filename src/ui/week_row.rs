use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Widget, WidgetRef},
};

use crate::{data::calendar::CalendarDay, state::RiqiState, ui::day_cell::DayCell};

use super::layout::RiqiLayout;

pub struct WeekRow<'a> {
    pub days_cell_state: &'a [CalendarDay],
    pub riqi_state: &'a RiqiState,
    pub riqi_layout: &'a RiqiLayout,
}

impl<'a> WeekRow<'a> {
    pub fn new(
     days_cell_state: &'a [CalendarDay],
     riqi_state: &'a RiqiState,
     riqi_layout: &'a RiqiLayout,
    ) -> Self {
       WeekRow{
            days_cell_state,
            riqi_state,
            riqi_layout
        }
    }
}


impl Widget for WeekRow<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let day_item_width = self.riqi_layout.month_calendar.day_item_column as u16;
        let day_item_height = self.riqi_layout.month_calendar.day_item_row as u16;
        for (day_idx, day_data) in self.days_cell_state.iter().enumerate() {
            let day_cell_item = DayCell::new(day_data, self.riqi_state);
            let day_cell_area = Rect::new(
                area.left()
                    + day_idx as u16
                        * (day_item_width + self.riqi_layout.month_calendar.day_gap as u16),
                area.top(),
                day_item_width,
                day_item_height,
            );
            day_cell_item.render(day_cell_area, buf);
        }
        todo!()
    }
}
