use ratatui::{buffer::Buffer, layout::Rect, widgets::WidgetRef};

use super::day_cell::DayCellState;

pub struct WeekRow<'a> {
    days_cell_state: &'a [DayCellState],
}

impl WidgetRef for WeekRow<'_> {
    #[doc = " Draws the current state of the widget in the given buffer. That is the only method required"]
    #[doc = " to implement a custom widget."]
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}
