use ratatui::layout::{Constraint, Flex, Layout, Rect};

pub struct RiqiLayout {
    // 不包括边框
    pub day_item_row: u32,
    // 不包括边框
    pub day_item_col: u32,
}

/*
*  return [title_area, _ ,calendar_area,_command_line]
*
*/
pub fn get_layout(area: Rect) -> [Rect; 4] {
    let layout: [Rect; 4] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Max(1),
        Constraint::Length(32),
        Constraint::Length(1),
    ])
    .flex(Flex::Center)
    .areas(area);
    layout
}
