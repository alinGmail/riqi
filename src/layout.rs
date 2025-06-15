use crate::config::config_struct::Config;
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
pub fn get_layout(area: Rect, config: &Config) -> [Rect; 4] {
    let mut month_content_constraint: Constraint = Constraint::Min(32);
    if let Some(day_cell) = &config.day_cell {
        if let Some(height) = day_cell.height {
            let (day_cell_row, day_cell_column) = get_day_cell_size(height, 0);
            let (month_row, month_col) = get_month_calender_size(day_cell_row, day_cell_column);
            month_content_constraint = Constraint::Length(month_row as u16);
        }
    }

    let layout: [Rect; 4] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Max(1),
        month_content_constraint,
        Constraint::Length(1),
    ])
    .flex(Flex::Center)
    .areas(area);
    layout
}

/**
*  return [day_item_row,day_item_col]
*  根据day cell的内容，得到day cell的总大小，上下2行边框，左右多一个空格
*/
pub fn get_day_cell_size(content_row: u32, content_column: u32) -> (u32, u32) {
    return (content_row + 2, content_column + 4);
}

/**
*  return [day_item_row, day_item_col]
*  更具 day cell 的宽度 和高度，决定日历的宽度和高度，不包括头部（星期几部分和 几年几月的标题）
*/
pub fn get_month_calender_size(row: u32, column: u32) -> (u32, u32) {
    return (row * 7 + 6, column * 6);
}
