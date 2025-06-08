use chrono::Datelike;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Widget},
};

use crate::lunar::{number_to_lunar_day, number_to_lunar_month};
use crate::state::RiqiState;
use crate::{
    data::CalendarDay,
    holiday_data::{Holiday, HolidayMap},
};

use super::utils::get_style_from_config;

pub fn render_day_item(buffer: &mut Buffer, day: &CalendarDay, rect: Rect, riqi_state: &RiqiState) {
    let day_item = DayItem::new(day, riqi_state);
    day_item.render(rect, buffer);
}

#[derive(Debug, Clone)]
struct DayItem<'a> {
    day: &'a CalendarDay,
    riqi_state: &'a RiqiState<'a>,
}

fn get_holidays<'a>(
    holiday_map: &'a HolidayMap,
    key1: &str,
    key2: &str,
) -> Option<&'a Vec<Holiday>> {
    holiday_map.get(key1)?.get(key2)
}

/// A button with a label that can be themed.
impl<'a> DayItem<'a> {
    pub fn new(day: &'a CalendarDay, riqi_state: &'a RiqiState) -> Self {
        DayItem { day, riqi_state }
    }

    pub fn is_selected_day(&self) -> bool {
        let select_day = self.riqi_state.select_day;
        select_day.day() == self.day.day
            && select_day.month() == self.day.month
            && select_day.year() as u32 == self.day.year
    }
    pub fn is_today(&self) -> bool {
        let today = self.riqi_state.today;
        today.day() == self.day.day
            && today.month() == self.day.month
            && today.year() as u32 == self.day.year
    }

    pub fn get_day_item_style(&self) -> Style {
        let mut style = self.riqi_state.theme.get_default_style();
        if self.day.day_of_week == 6 || self.day.day_of_week == 0 {
            // 周六日使用节假日颜色
            if self.day.is_current_month {
                style = get_style_from_config(Some(style), self.riqi_state.theme.holiday);
            } else {
                style = get_style_from_config(Some(style), self.riqi_state.theme.holiday_adjacent);
            }
        } else {
            // 工作日使用工作颜色
            if self.day.is_current_month {
                style = get_style_from_config(Some(style), self.riqi_state.theme.workday);
            } else {
                style = get_style_from_config(Some(style), self.riqi_state.theme.workday_adjacent);
            }
        }

        if self.is_today() {
            style = style.bold();
        }

        if self.is_selected_day() {
            style = get_style_from_config(Some(style), self.riqi_state.theme.focus_day)
        }

        style
    }

    pub fn normal_render_content(self, area: Rect, buf: &mut Buffer) {
        let day_item_style = self.get_day_item_style();
        let line = Line::from(self.day.day.to_string()).style(day_item_style);
        line.render(
            Rect {
                x: area.left() + 1,
                y: area.top(),
                width: 2,
                height: 1,
            },
            buf,
        );

        if self.is_today() {
            let today_line = Line::from("今").style(day_item_style).centered();
            today_line.render(
                Rect {
                    x: area.left() + 5,
                    y: area.top(),
                    width: 2,
                    height: 1,
                },
                buf,
            );
        };

        // 显示农历日期
        let lunar_day = if self.day.lunar_day == 1 {
            // 如果是初一，显示月份
            number_to_lunar_month(self.day.lunar_month)
        } else {
            // 其他日期显示日期
            number_to_lunar_day(self.day.lunar_day)
        };
        let lunar_line = Line::from(lunar_day).style(day_item_style);
        lunar_line.render(
            Rect {
                x: area.left() + 1,
                y: area.top() + 1,
                width: 6,
                height: 1,
            },
            buf,
        );

        let date_str = format!(
            "{:04}-{:02}-{:02}",
            self.day.year, self.day.month, self.day.day
        );
        // 使用
        if let Some(holidays) = get_holidays(self.riqi_state.holiday_map, "2025_cn_zh", &date_str) {
            // 处理 holidays
            if let Some(holiday) = holidays.first() {
                let holiday = Line::from(holiday.name.clone())
                    .centered()
                    .style(day_item_style);
                holiday.render(
                    Rect {
                        x: area.left() + 1,
                        y: area.top() + 2,
                        width: 6,
                        height: 1,
                    },
                    buf,
                );
            }
        }
    }

    pub fn widly_render_content(self, area: Rect, buf: &mut Buffer) {}
}

impl<'a> Widget for DayItem<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.get_day_item_style().fg.unwrap()));
        let inner_area = block.inner(area);
        block.render(area, buf);
        self.normal_render_content(inner_area, buf);
    }
}
