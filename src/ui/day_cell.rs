use ratatui::{buffer::Buffer, layout::Rect, widgets::WidgetRef};

pub struct DayCellState {
    day_number: u32,
    is_today: bool,
    is_selected: bool,
}

struct DayCell<'a> {
    day: &'a DayCellState,
}

impl WidgetRef for DayCell<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        // todo 

    }
}
