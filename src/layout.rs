use ratatui::layout::{Constraint, Flex, Layout, Rect};

pub struct RiqiLayout {
    // 不包括边框
    pub day_item_row: u32,
    // 不包括边框
    pub day_item_col: u32,
}

pub fn get_layout(area: Rect) -> [Rect; 3] {
    let layout: [Rect; 3] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(32),
    ])
    .flex(Flex::Center)
    .areas(area);
    layout
}
