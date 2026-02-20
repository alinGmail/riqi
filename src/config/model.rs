use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub country: String,
    pub language: String,
    pub column: Option<u32>,
    pub row: Option<u32>,
    pub show_lunar: bool,
    pub show_holiday:bool,
    pub output: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigFile {
    pub language: Option<String>,
    pub country: Option<String>,
    pub show_lunar: Option<bool>,
    pub show_holiday: Option<bool>,
    pub hide_bg: Option<bool>,
    pub column: Option<u32>,
    pub row: Option<u32>,
    pub output: Option<String>,
}

