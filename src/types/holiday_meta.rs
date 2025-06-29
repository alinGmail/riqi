use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct HolidayMeta {
    pub last_updated: String,
    pub files: HashMap<String, HolidayFileInfo>,
}

#[derive(Deserialize, Debug)]
pub struct HolidayFileInfo {
    pub last_modified: String,
}
