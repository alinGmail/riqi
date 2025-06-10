use std::str::FromStr;

use ratatui::{
    text::{Line, Span},
    widgets::Widget,
};

use crate::{
    i18n::{get_translate, Language},
    state::RiqiState,
};

pub struct BottomLineComponent<'a> {
    pub riqi_state: &'a RiqiState<'a>,
}

impl<'a> Widget for BottomLineComponent<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let language = Language::from_str(&self.riqi_state.config.language);
        if let Ok(country_item) = language {
            let translate = get_translate(country_item);
            let line = Line::from(vec![
                Span::from(translate.navigation),
                Span::from(":"),
                Span::from("h,j,k,l"),
                Span::from(" | "),
                Span::from(translate.prev_month),
                Span::from(":"),
                Span::from("u"),
                Span::from(" | "),
                Span::from(translate.next_month),
                Span::from(":"),
                Span::from("d"),
                Span::from(" | "),
                Span::from(translate.next_year),
                Span::from(":"),
                Span::from("x"),
                Span::from(" | "),
                Span::from(translate.prev_year),
                Span::from(":"),
                Span::from("y"),
            ])
            .centered();
            line.render(area, buf);
        }
    }
}
