use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub country: String,
    pub language: String,
    pub column: Option<u32>,
    pub row: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigFile {
    pub language: Option<String>,
    pub country: Option<String>,
    pub show_lunar: Option<bool>,
    pub hide_bg: Option<bool>,
    pub column: Option<u32>,
    pub row: Option<u32>,
}

