use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FileConfig {
    pub language: Option<String>,
    pub country: Option<String>,
    pub show_lunar: Option<bool>,
    pub day_cell: Option<DayCell>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DayCell {
    pub width: Option<u32>,
    pub height: Option<u32>,
}
