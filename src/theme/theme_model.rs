use ratatui::style::{Color, Style};
use serde::Deserialize;



#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Theme {
    #[serde(deserialize_with = "crate::theme::color_serde::de_color")]
    pub fg: Color,
    #[serde(deserialize_with = "crate::theme::color_serde::de_color")]
    pub bg: Color,
    pub focus_day: ItemStyle,

    pub month_til: ItemStyle,
    pub month_head: ItemStyle,
    pub workday_adjacent: ItemStyle,
    pub workday: ItemStyle,
    pub holiday_adjacent: ItemStyle,
    pub holiday: ItemStyle,
    pub bottom_line: ItemStyle,
}

impl Theme {
    pub fn get_default_style(&self) -> Style {
        Style::default().fg(self.fg)
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ItemStyle {
    #[serde(default, deserialize_with = "crate::theme::color_serde::de_opt_color")]
    pub bg: Option<Color>,
    #[serde(default, deserialize_with = "crate::theme::color_serde::de_opt_color")]
    pub fg: Option<Color>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub under_line: Option<bool>,
    pub transparent_bg: Option<bool>,
}

