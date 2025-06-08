use ratatui::style::{Color, Style};

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub fg: Color,
    pub bg: Color,
    pub shadow: Color,
    pub holi_day: Color,
    pub work_day: Color,
    pub focus_day: Color,
    pub today: Color,
    pub not_cur_month: Color,

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

#[derive(Debug, Clone, Copy)]
pub struct ItemStyle {
    pub bg: Option<Color>,
    pub fg: Option<Color>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub under_line: Option<bool>,
    pub transparent_bg: Option<bool>,
}

pub const BLUE: Theme = Theme {
    fg: Color::Rgb(16, 24, 48),
    bg: Color::Rgb(48, 72, 144),
    shadow: Color::Rgb(32, 48, 96),
    holi_day: Color::Rgb(233, 101, 165),
    work_day: Color::Rgb(177, 242, 167),
    focus_day: Color::Rgb(177, 186, 244),
    today: Color::Rgb(235, 222, 118),
    not_cur_month: Color::Rgb(120, 120, 120),

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
        fg: Some(Color::Rgb(177, 186, 244)),
        bold: Some(false),
        italic: Some(false),
        under_line: Some(false),
        transparent_bg: Some(false),
    },

    holiday_adjacent: ItemStyle {
        bg: None,
        fg: Some(Color::Rgb(177, 186, 244)),
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
        fg: Some(Color::Rgb(177, 186, 244)),
        bold: Some(false),
        italic: Some(false),
        under_line: Some(false),
        transparent_bg: Some(false),
    },
    holiday: ItemStyle {
        bg: None,
        fg: Some(Color::Rgb(177, 186, 244)),
        bold: Some(false),
        italic: Some(false),
        under_line: Some(false),
        transparent_bg: Some(false),
    },
};
