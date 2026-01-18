use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Widget, WidgetRef},
};

use crate::data::calendar::CalendarDay;

pub struct WeekRow<'a> {
    days_cell_state: &'a [CalendarDay],
}

impl Widget for WeekRow<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}
