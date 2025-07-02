use chrono::{DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Serialize)]
pub struct HolidayMeta {
    pub last_updated: String,
    pub files: HashMap<String, HolidayFileInfo>,
}

impl HolidayMeta {
    /// get all availavle year holiday
    /// example return ["2025_zh_cn","2025_fr_fr"]
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - A vector containing the keys of the `files` map.
    pub fn get_available_year_holiday_keys(&self) -> Vec<String> {
        self.files.keys().cloned().collect()
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct HolidayFileInfo {
    pub last_modified: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaCache {
    pub data: HolidayMeta,
    pub cache_time: DateTime<Utc>,
}
