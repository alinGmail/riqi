use crate::config::config_struct::DayCell;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FileConfig {
    pub language: Option<String>,
    pub country: Option<String>,
    pub show_lunar: Option<bool>,
    pub day_cell: Option<DayCell>,
    pub hide_bg: Option<bool>,
}
