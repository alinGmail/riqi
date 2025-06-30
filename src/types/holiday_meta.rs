use chrono::{DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Serialize)]
pub struct HolidayMeta {
    pub last_updated: String,
    pub files: HashMap<String, HolidayFileInfo>,
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
