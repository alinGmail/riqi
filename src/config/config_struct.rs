use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Config {
    pub country: Option<String>,
    pub language: String,
    pub calendar_type: CalendarType,
    pub show_lunar: bool,
    pub day_cell: Option<DayCell>,
    pub hide_bg: bool,
    pub HolidayConfig: HolidayConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HolidayConfig {
    pub github_url: String,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DayCell {
    pub width: Option<u32>,
    pub height: Option<u32>,
}
