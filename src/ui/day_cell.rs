use ratatui::widgets::Widget;

use crate::{data::calendar::CalendarDay, state::RiqiState};

struct DayCell<'a> {
    day: &'a CalendarDay,
    riqi_state: &'a RiqiState,
}

impl<'a> DayCell<'a> {
    pub fn new(day: &'a CalendarDay, riqi_state: &'a RiqiState) -> Self {
        DayCell { day, riqi_state }
    }
}

impl Widget for DayCell<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        todo!()
    }
}
