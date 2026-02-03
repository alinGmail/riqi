use super::utils::get_style_from_config;
use crate::config::model::AppConfig;
use crate::holiday::utils::get_holiday_state;
use crate::ui::lunar::{number_to_lunar_day, number_to_lunar_month};
use crate::{data::calendar::CalendarDay, state::RiqiState};
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Widget},
};

pub struct DayCell<'a> {
    day_data: &'a CalendarDay,
    riqi_state: &'a RiqiState,
    app_config: &'a AppConfig,
}

impl<'a> DayCell<'a> {
    pub fn new(
        day_data: &'a CalendarDay,
        riqi_state: &'a RiqiState,
        app_config: &'a AppConfig,
    ) -> Self {
        DayCell {
            day_data,
            riqi_state,
            app_config,
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

    fn render_out_border(&self, is_rest_day: bool, area: Rect, buf: &mut Buffer) -> Rect {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.get_day_item_style(is_rest_day).fg.unwrap()));
        let inner_area = block.inner(area);
        block.render(area, buf);
        inner_area
    }

    fn render_work_rest_icon(&self, is_rest_day: bool, area: Rect, buf: &mut Buffer, style: Style) {
        let holiday_line = Line::from(if is_rest_day { "休" } else { "班" })
            .style(style)
            .centered();
        holiday_line.render(area, buf);
    }

    fn render_content(
        &self,
        is_rest_day: bool,
        show_holiday_icon: bool,
        inner_area: Rect,
        buf: &mut Buffer,
    ) {
        let day_item_style = self.get_day_item_style(is_rest_day);
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

        if show_holiday_icon {
            self.render_work_rest_icon(
                is_rest_day,
                Rect {
                    x: icon_x,
                    y: inner_area.top(),
                    width: 2,
                    height: 1,
                },
                buf,
                day_item_style,
            );
            icon_x -= 2;
        }

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
            icon_x -= 2;
        }

        let mut content_lines: Vec<Line> = vec![];

        if let Some(true) = self.app_config.show_lunar {
            content_lines.push(self.get_lunar_line(day_item_style));
        }

        if let Some(holidays) = &self.day_data.holidays {
            for holiday in holidays {
                let holiday_name = holiday.name.clone();
                content_lines.push(Line::from(holiday_name).style(day_item_style))
            }
        }

        let paragraph = Paragraph::new(content_lines).wrap(Wrap { trim: false });
        paragraph.render(
            Rect {
                x: inner_area.left(),
                y: inner_area.top() + 1,
                width: inner_area.width,
                height: inner_area.height - 1,
            },
            buf,
        );
    }

    pub fn get_lunar_line(&self, style: Style) -> Line {
        // 显示农历日期
        let lunar_day = if self.day_data.lunar_day == 1 {
            // 如果是初一，显示月份
            number_to_lunar_month(self.day_data.lunar_month)
        } else {
            // 其他日期显示日期
            number_to_lunar_day(self.day_data.lunar_day)
        };
        let lunar_line = Line::from(lunar_day).style(style);
        lunar_line
    }
}

impl Widget for DayCell<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let (is_rest_day, show_holiday_icon) =
            get_holiday_state(&self.day_data.holidays, self.day_data.day_of_week as u16);
        let inner_area = self.render_out_border(is_rest_day, area, buf);
        self.render_content(is_rest_day, show_holiday_icon, inner_area, buf);
    }
}
