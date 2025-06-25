use ratatui::style::{Color, Style};
use serde::Deserialize;

pub mod color_serde;

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

pub const BLUE: Theme = Theme {
    fg: Color::Rgb(16, 24, 48),
    bg: Color::Rgb(40, 36, 51),

    month_til: ItemStyle {
        bg: None,
        fg: Some(Color::Rgb(177, 186, 244)),
        bold: Some(false),
        italic: Some(false),
        under_line: Some(false),
        transparent_bg: Some(false),
    },

    month_head: ItemStyle {
        bg: None,
        fg: Some(Color::Rgb(177, 186, 244)),
        bold: Some(false),
        italic: Some(false),
        under_line: Some(false),
        transparent_bg: Some(false),
    },

    workday_adjacent: ItemStyle {
        bg: None,
        fg: Some(Color::Rgb(147, 138, 173)),
        bold: Some(false),
        italic: Some(false),
        under_line: Some(false),
        transparent_bg: Some(false),
    },

    holiday_adjacent: ItemStyle {
        bg: None,
        fg: Some(Color::Rgb(126, 53, 88)),
        bold: Some(false),
        italic: Some(false),
        under_line: Some(false),
        transparent_bg: Some(false),
    },

    bottom_line: ItemStyle {
        bg: None,
        fg: Some(Color::Rgb(177, 186, 244)),
        bold: Some(false),
        italic: Some(false),
        under_line: Some(false),
        transparent_bg: Some(false),
    },

    workday: ItemStyle {
        bg: None,
        fg: Some(Color::Rgb(178, 244, 243)),
        bold: Some(false),
        italic: Some(false),
        under_line: Some(false),
        transparent_bg: Some(false),
    },
    holiday: ItemStyle {
        bg: None,
        fg: Some(Color::Rgb(233, 101, 165)),
        bold: Some(false),
        italic: Some(false),
        under_line: Some(false),
        transparent_bg: Some(false),
    },
    focus_day: ItemStyle {
        bg: None,
        fg: Some(Color::Rgb(177, 242, 167)),
        bold: Some(true),
        italic: Some(false),
        under_line: Some(false),
        transparent_bg: Some(false),
    },
};
