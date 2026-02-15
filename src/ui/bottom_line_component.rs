use crate::config::model::AppConfig;
use crate::state::RiqiState;
use crate::ui::translate::{get_translate, Language};
use crate::ui::utils::get_style_from_config;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use std::str::FromStr;

pub struct BottomLineComponent<'a> {
    pub app_config: &'a AppConfig,
    pub riqi_state: &'a RiqiState,
}

impl<'a> Widget for BottomLineComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let language = Language::from_str(&self.app_config.language);

        if area.height == 0 {
            return;
        }

        if let Ok(language) = language {
            let translate = get_translate(language);
            let line = Line::from(vec![
                Span::from("h,j,k,l"),
                Span::from(":"),
                Span::from(translate.navigation),
                Span::from(" | "),
                Span::from("u"),
                Span::from(":"),
                Span::from(translate.prev_month),
                Span::from(" | "),
                Span::from("d"),
                Span::from(":"),
                Span::from(translate.next_month),
                Span::from(" | "),
                Span::from("f"),
                Span::from(":"),
                Span::from(translate.next_year),
                Span::from(" | "),
                Span::from("b"),
                Span::from(":"),
                Span::from(translate.prev_year),
                Span::from(" | "),
                Span::from("t"),
                Span::from(":"),
                Span::from(translate.back_to_today),
            ])
            .centered()
            .style(get_style_from_config(
                Some(Style::default()),
                self.riqi_state.theme.bottom_line,
            ));

            line.render(
                Rect {
                    x: area.x,
                    y: area.y + area.height.saturating_sub(1),
                    width: area.width,
                    height: 1,
                },
                buf,
            );
        }
    }
}
