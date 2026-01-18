use ratatui::layout::{Constraint, Flex, Layout, Rect};

#[derive(Debug)]
pub struct MonthCalendarLayout {
    pub area: Rect,
    pub title: Rect,
    pub head: Rect,
    pub content: Rect,
    pub day_item_row: u32,
    pub day_item_column: u32,
    pub day_gap: u32,
}

#[derive(Debug)]
pub struct RiqiLayout {
    pub title: Rect,
    pub month_calendar: MonthCalendarLayout,
    pub bottom_line: Rect,
}

pub fn get_layout(frame_area: Rect, day_cell_width: Option<u32>, day_cell_height: Option<u32>) -> RiqiLayout {
    // terminal 的最少行是 24 行，最少列是 41 列
    // 月历部分的默认高度
    let mut month_calendar_row_constraint: Constraint = Constraint::Min(34);
    // 月历部分的默认高度
    let mut month_content_column_constraint: Constraint = Constraint::Length(frame_area.width - 2);
    if let Some(row) = day_cell_height {
        let day_cell_row = get_day_cell_row(row);
        let month_row = get_month_calender_row(day_cell_row);
        // 4 行是标题 和 星期的行
        month_calendar_row_constraint = Constraint::Length(month_row as u16 + 4);
    }
    if let Some(column) = day_cell_width {
        let day_cell_column = get_day_cell_column(column);
        let month_column = get_month_calender_column(day_cell_column);
        month_content_column_constraint = Constraint::Length(month_column as u16);
    }

    // [calendar_area,_command_line]
    let main_rows: [Rect; 2] =
        Layout::vertical([month_calendar_row_constraint, Constraint::Max(2)])
            .flex(Flex::Center)
            .areas(frame_area);

    let month_calendar_area = *main_rows.first().unwrap();

    let month_cal_center_area = *Layout::horizontal([month_content_column_constraint])
        .flex(Flex::Center)
        .split(month_calendar_area)
        .first()
        .unwrap();

    let month_calendar = MonthCalendarLayout {
        area: month_cal_center_area,
        title: Rect {
            x: month_cal_center_area.x,
            y: month_cal_center_area.y,
            width: month_cal_center_area.width,
            height: 2,
        },
        head: Rect {
            x: month_cal_center_area.x,
            y: month_cal_center_area.y + 2,
            width: month_cal_center_area.width,
            height: 2,
        },
        content: Rect {
            x: month_cal_center_area.x,
            y: month_cal_center_area.y + 4,
            width: month_cal_center_area.width,
            height: month_cal_center_area.height - 4,
        },
        day_item_column: (month_cal_center_area.width as u32 - 6) / 7,
        day_item_row: (month_cal_center_area.height as u32 - 4) / 6,
        day_gap:1,
    };
    let riqi_layout = RiqiLayout {
        title: Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        month_calendar,
        bottom_line: *main_rows.get(1).unwrap(),
    };
    riqi_layout
}

pub fn get_day_cell_row(content_row: u32) -> u32 {
    content_row + 2
}

/// 左右多一个空格
pub fn get_day_cell_column(content_column: u32) -> u32 {
    content_column + 4
}

pub fn get_month_calender_row(day_cell_row: u32) -> u32 {
    day_cell_row * 6
}

pub fn get_month_calender_column(day_cell_column: u32) -> u32 {
    day_cell_column * 7 + 6
}
