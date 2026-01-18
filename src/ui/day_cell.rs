use ratatui::{buffer::Buffer, layout::Rect, widgets::WidgetRef};
use tyme4rs::tyme::solar::SolarDay;

#[derive(Debug)]
pub struct DayCellState {
    pub year: u32,
    pub month: u32, // 1-12
    pub day: u32,
    pub day_of_week: u32,       // 0=Sunday, 6=Saturday
    pub is_current_month: bool, // 是否属于当前月份
    pub lunar_month: i32,       // 农历月份
    pub lunar_day: i32,         // 农历日期
}
impl DayCellState {
    pub fn new(year: u32, month: u32, day: u32, day_of_week: u32, is_current_month: bool) -> Self {
        let solar = SolarDay::from_ymd(year as isize, month as usize, day as usize);
        let lunar_month = solar.get_lunar_day().get_month() as i32;
        let lunar_day = solar.get_lunar_day().get_day() as i32;

        DayCellState {
            year,
            month,
            day,
            day_of_week,
            is_current_month,
            lunar_month,
            lunar_day,
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
