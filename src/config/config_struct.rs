use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    pub country: String,
    pub language: String,
    pub calendar_type: CalendarType,
    pub show_lunar: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarType {
    // 普通模式 3行6列，中文的首选模式
    Normal,
    // 宽度无限，高3行，英文的首选模式
    WideScreen,
}

impl FromStr for CalendarType {
    type Err = String;
    fn from_str(calendar_type: &str) -> Result<Self, Self::Err> {
        match calendar_type.to_lowercase().as_str() {
            "normal" => Ok(CalendarType::Normal),
            "widescreen" => Ok(CalendarType::WideScreen),
            _ => Err(format!("Unknown language code: {}", calendar_type)),
        }
    }
}
