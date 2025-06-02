use chrono::Datelike;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, StatefulWidget, Widget, Wrap},
};

use crate::{
    data::CalendarDay,
    holiday_data::{Holiday, HolidayMap},
    theme::BLUE,
};
use crate::{state::RiqiState, theme::Theme};

pub fn render_day_item<'a>(
    buffer: &mut Buffer,
    day: &'a CalendarDay,
    rect: Rect,
    riqi_state: &RiqiState,
) {
    let day_item = CnDayItem::new(day, riqi_state);

    StatefulWidget::render(
        day_item,
        rect,
        buffer,
        &mut DayItemState { selected: false },
    );
}

#[derive(Debug, Clone)]
struct DayItemState {
    selected: bool,
}

#[derive(Debug, Clone)]
struct DayItem {
    day: u32,
    theme: Theme,
}

#[derive(Debug, Clone)]
struct CnDayItem<'a> {
    day: &'a CalendarDay,
    theme: Theme,
    riqi_state: &'a RiqiState<'a>,
}

/// A button with a label that can be themed.
impl DayItem {
    pub fn new(day: u32) -> Self {
        DayItem { day, theme: BLUE }
    }

    pub const fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}

fn get_holidays<'a>(
    holiday_map: &'a HolidayMap,
    key1: &str,
    key2: &str,
) -> Option<&'a Vec<Holiday>> {
    holiday_map.get(key1)?.get(key2)
}

/// A button with a label that can be themed.
impl<'a> CnDayItem<'a> {
    pub fn new(day: &'a CalendarDay, riqi_state: &'a RiqiState) -> Self {
        CnDayItem {
            day,
            theme: BLUE,
            riqi_state,
        }
    }

    pub fn is_selected_day(&self) -> bool {
        let select_day = self.riqi_state.select_day;
        return select_day.year() as u32 == self.day.year
            && select_day.month() == self.day.month
            && select_day.day() == self.day.day;
    }

    pub fn get_fg_color(&self) -> Style {
        // 是不是今天
        if self.is_selected_day() {
            return Style::default().fg(BLUE.focus_day).bold();
        }

        // 是不是 这个月
        if !self.day.is_current_month {
            return Style::default().fg(BLUE.not_cur_month);
        }

        let style = if self.day.day_of_week == 6 || self.day.day_of_week == 0 {
            // 周六日使用节假日颜色
            Style::default().fg(BLUE.holi_day)
        } else {
            // 工作日使用工作颜色
            Style::default().fg(BLUE.work_day)
        };
        return style;
    }

    pub const fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    pub fn render_content_3row6col(self, area: Rect, buf: &mut Buffer) {
        let line = Line::from(self.day.day.to_string()).style(self.get_fg_color());
        line.render(
            Rect {
                x: area.left() + 1,
                y: area.top(),
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
        if let Some(holidays) = get_holidays(&self.riqi_state.holiday_map, "2025_cn_zh", &date_str)
        {
            // 处理 holidays
            if let Some(holiday) = holidays.get(0) {
                let holiday = Line::from(holiday.name.clone())
                    .centered()
                    .style(self.get_fg_color());
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

    pub fn render_content_2row6col(self, area: Rect, buf: &mut Buffer) {
        let line = Line::from(self.day.day.to_string()).style(Style::default());
        let holiday = Line::from("中秋节").centered().style(Style::default());
        line.render(
            Rect {
                x: area.left() + 1,
                y: area.top(),
                width: 6,
                height: 1,
            },
            buf,
        );
        holiday.render(
            Rect {
                x: area.left() + 1,
                y: area.top() + 1,
                width: 6,
                height: 1,
            },
            buf,
        );
    }
}

impl<'a> StatefulWidget for CnDayItem<'a> {
    type State = DayItemState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(self.get_fg_color());
        let inner_area = block.inner(area);
        block.render(area, buf);
        self.render_content_3row6col(inner_area, buf);
    }
}

impl StatefulWidget for DayItem {
    type State = DayItemState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        // 1. 首先渲染Block
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded);
        let line =
            Line::from(self.day.to_string()).style(Style::default().fg(self.theme.highlight));

        block.render(area, buf);
        line.render(
            Rect {
                x: area.left() + 1,
                y: area.top() + 1,
                width: 2,
                height: 1,
            },
            buf,
        );
        let holiday_text = Line::from("Juneteenth National Independence Day");
        let paragraph = Paragraph::new(holiday_text)
            .wrap(Wrap { trim: true }) // 启用自动换行
            .style(Style::default());
        paragraph.render(
            Rect {
                x: area.left() + 1,
                y: area.top() + 2,
                width: 6,
                height: 2,
            },
            buf,
        );
    }
}
