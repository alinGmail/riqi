use ratatui::layout::Rect;

#[derive(Debug)]
pub struct MonthCalendarLayout {
    pub area: Rect,
    pub title: Rect,
    pub head: Rect,
    pub content: Rect,
    pub day_item_row: u32,
    pub day_item_column: u32,
}

#[derive(Debug)]
pub struct RiqiLayout {
    pub title: Rect,
    pub month_calendar: MonthCalendarLayout,
    pub bottom_line: Rect,
}
