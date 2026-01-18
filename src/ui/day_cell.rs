use ratatui::{buffer::Buffer, layout::Rect, widgets::WidgetRef};

#[derive(Debug)]
pub struct DayCellState {
    day_number: u32,
    is_today: bool,
    is_selected: bool,
}
impl DayCellState {
    pub fn new(year: u32, month: u32, day: u32, day_of_week: u32, is_current_month: bool) -> Self{
        DayCellState{
            day_number:day,
            is_today:false,
            is_selected:false,
        }
    }
}

struct DayCell<'a> {
    day: &'a DayCellState,
}

impl WidgetRef for DayCell<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        // todo 

    }
}
