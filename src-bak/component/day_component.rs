use chrono::Datelike;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap},
};

use crate::{
    holiday::types::{Holiday, PrimaryType},
    state::RiqiState,
};
use crate::{
    lunar::{number_to_lunar_day, number_to_lunar_month},
    types::calendar::CalendarDay,
};

use super::utils::get_style_from_config;

pub fn render_day_item(buffer: &mut Buffer, day: &CalendarDay, rect: Rect, riqi_state: &RiqiState) {
    let day_item = DayItem::new(day, riqi_state);
    day_item.render(rect, buffer);
}

#[derive(Debug, Clone)]
struct DayItem<'a> {
    day: &'a CalendarDay,
    riqi_state: &'a RiqiState,
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

    pub fn get_holidays(&self) -> Option<&Vec<Holiday>> {
        self.riqi_state.config.country.as_ref()?;

        let date_str = format!(
            "{:04}-{:02}-{:02}",
            self.day.year, self.day.month, self.day.day
        );
        let holiday_map_key = format!(
            "{}_{}_{}",
            &self.day.year.to_string().as_str(),
            &self.riqi_state.config.language,
            &self.riqi_state.config.country.clone().unwrap().clone()
        );
        self.riqi_state
            .holiday_map
            .get(&holiday_map_key)?
            .get(date_str.as_str())
    }

    // 判断今天是否是节日，
    // return (是否放假, true 放假，false 上班:bool  | 是否国家节日,用于是否显示图标:bool)
    pub fn is_holiday(&self) -> (bool, bool) {
        let holidays = self.get_holidays();
        if let Some(holiday_vec) = holidays {
            let is_holiday = holiday_vec.iter().any(|holiday| {
                matches!(
                    holiday.primary_type,
                    PrimaryType::SubstituteHoliday | PrimaryType::NationalHoliday
                )
            });
            if is_holiday {
                return (true, true);
            }
            let is_workday = holiday_vec
                .iter()
                .any(|holiday| matches!(holiday.primary_type, PrimaryType::WorkingDayOnWeekend));
            if is_workday {
                return (false, true);
            }
        };

        (
            self.day.day_of_week == 6 || self.day.day_of_week == 0,
            false,
        )
    }

    pub fn get_day_item_style(&self, is_holiday: bool) -> Style {
        let mut style = self.riqi_state.theme.get_default_style();
        if is_holiday {
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

    pub fn render_content(
        self,
        area: Rect,
        buf: &mut Buffer,
        is_holiday: bool,
        show_holiday_icon: bool,
    ) {
        let day_item_style = self.get_day_item_style(is_holiday);
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

        let mut icon_x = area.left() + area.width - 3;

        if show_holiday_icon {
            let holiday_line = Line::from(if is_holiday { "休" } else { "班" })
                .style(day_item_style)
                .centered();
            holiday_line.render(
                Rect {
                    x: icon_x,
                    y: area.top(),
                    width: 2,
                    height: 1,
                },
                buf,
            );
            icon_x -= 2;
        }
        if self.is_today() {
            let today_line = Line::from("今").style(day_item_style).centered();
            today_line.render(
                Rect {
                    x: icon_x,
                    y: area.top(),
                    width: 2,
                    height: 1,
                },
                buf,
            );
        };

        let mut content_lines: Vec<Line> = vec![];

        if self.riqi_state.config.show_lunar {
            content_lines.push(self.get_lunar_line(day_item_style));
        }

        // 使用
        if let Some(holidays) = self.get_holidays() {
            // 处理 holidays
            if let Some(holiday) = holidays.first() {
                let holiday = Line::from(holiday.name.clone()).style(day_item_style);
                content_lines.push(holiday);
            }
        }

        let paragraph = Paragraph::new(content_lines).wrap(Wrap { trim: false });
        paragraph.render(
            Rect {
                x: area.left() + 1,
                y: area.top() + 1,
                width: area.width - 2,
                height: area.height - 1,
            },
            buf,
        );
    }

    pub fn get_lunar_line(&self, style: Style) -> Line {
        // 显示农历日期
        let lunar_day = if self.day.lunar_day == 1 {
            // 如果是初一，显示月份
            number_to_lunar_month(self.day.lunar_month)
        } else {
            // 其他日期显示日期
            number_to_lunar_day(self.day.lunar_day)
        };
        let lunar_line = Line::from(lunar_day).style(style);
        lunar_line
    }

    pub fn widly_render_content(self, area: Rect, buf: &mut Buffer) {}
}

impl<'a> Widget for DayItem<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // area 包括了边框
        let (is_holiday, show_holiday_icon) = self.is_holiday();
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.get_day_item_style(is_holiday).fg.unwrap()));
        let inner_area = block.inner(area);
        block.render(area, buf);
        self.render_content(inner_area, buf, is_holiday, show_holiday_icon);
    }
}
