use ratatui::style::Color;

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
}

#[derive(Debug, Clone, Copy)]
pub struct ItemStyle {
    pub bg: Option<Color>,
    pub fg: Color,
    pub bold: bool,
    pub italic: bool,
    pub under_line: bool,
}

pub const BLUE: Theme = Theme {
    fg: Color::Rgb(16, 24, 48),
    bg: Color::Rgb(48, 72, 144),
    shadow: Color::Rgb(32, 48, 96),
    holi_day: Color::Rgb(233, 101, 165),
    work_day: Color::Rgb(177, 242, 167),
    focus_day: Color::Rgb(177, 186, 244),
    //today: Color::Rgb(179, 244, 143),
    // focus_day: Color::Rgb(32, 48, 96),
    today: Color::Rgb(235, 222, 118),
    not_cur_month: Color::Rgb(120, 120, 120),

    month_til: ItemStyle {
        bg: None,
        fg: Color::Rgb(177, 186, 244),
        bold: false,
        italic: false,
        under_line: false,
    },
};
