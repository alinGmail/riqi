use ratatui::style::Color;

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub text: Color,
    pub background: Color,
    pub highlight: Color,
    pub shadow: Color,

    pub holi_day: Color,
    pub work_day: Color,
    pub focus_day: Color,
    pub today: Color,
}

pub const BLUE: Theme = Theme {
    text: Color::Rgb(16, 24, 48),
    background: Color::Rgb(48, 72, 144),
    highlight: Color::Rgb(64, 96, 192),
    shadow: Color::Rgb(32, 48, 96),
    holi_day: Color::Rgb(233, 101, 165),
    work_day: Color::Rgb(177, 242, 167),
    focus_day: Color::Rgb(177, 186, 244),
    //today: Color::Rgb(179, 244, 143),
    // focus_day: Color::Rgb(32, 48, 96),
    today: Color::Rgb(235, 222, 118),
};
