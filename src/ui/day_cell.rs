use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Widget},
};

use crate::{data::calendar::CalendarDay, state::RiqiState};

use super::utils::get_style_from_config;

struct DayCell<'a> {
    day_data: &'a CalendarDay,
    riqi_state: &'a RiqiState,
}

impl<'a> DayCell<'a> {
    pub fn new(day_data: &'a CalendarDay, riqi_state: &'a RiqiState) -> Self {
        DayCell {
            day_data,
            riqi_state,
        }
    }

    pub fn get_day_item_style(&self, is_holiday: bool) -> Style {
        let mut style = self.riqi_state.theme.get_default_style();
        if is_holiday {
            // 周六日使用节假日颜色
            if self.day_data.is_current_month {
                style = get_style_from_config(Some(style), self.riqi_state.theme.holiday);
            } else {
                style = get_style_from_config(Some(style), self.riqi_state.theme.holiday_adjacent);
            }
        } else {
            // 工作日使用工作颜色
            if self.day_data.is_current_month {
                style = get_style_from_config(Some(style), self.riqi_state.theme.workday);
            } else {
                style = get_style_from_config(Some(style), self.riqi_state.theme.workday_adjacent);
            }
        }

        if self.day_data.is_today {
            style = style.bold();
        }

        if self.day_data.is_select_day {
            style = get_style_from_config(Some(style), self.riqi_state.theme.focus_day)
        }

        style
    }

    fn render_out_border(&self, area: Rect, buf: &mut Buffer) -> Rect {
        let (is_holiday, show_holiday_icon) = (false, false);
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.get_day_item_style(is_holiday).fg.unwrap()));
        let inner_area = block.inner(area);
        block.render(area, buf);
        inner_area
    }
    fn render_conternt(&self, inner_area: Rect, buf: &mut Buffer) {
        let day_item_style = self.get_day_item_style(false);
        let line = Line::from(self.day_data.day.to_string()).style(day_item_style);

        line.render(
            Rect {
                x: inner_area.left() + 1,
                y: inner_area.top(),
                width: 2,
                height: 1,
            },
            buf,
        );

        let mut icon_x = inner_area.left() + inner_area.width - 3;
        if self.day_data.is_today {
            let today_line = Line::from("今").style(day_item_style).centered();
            today_line.render(
                Rect {
                    x: icon_x,
                    y: inner_area.top(),
                    width: 2,
                    height: 1,
                },
                buf,
            );
        }
    }
}

impl Widget for DayCell<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let inner_area = self.render_out_border(area, buf);
        self.render_conternt(inner_area, buf);
        todo!();
    }
}
