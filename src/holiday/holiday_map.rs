use crate::holiday::modal::HolidayOfYearList;
use std::collections::HashMap;

/// 主线程持有的假期内存存储。
/// key 为 `{year}_{language}_{country}`（见 `get_ylc_code`）。
#[derive(Default)]
pub struct HolidayMap {
    inner: HashMap<String, HolidayOfYearList>,
}

impl HolidayMap {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// 写入一年的假期数据，并按 version 去重。
    /// 若已有同 key 且版本不低于新数据，则忽略并返回 false。
    pub fn upsert(&mut self, key: String, holiday_of_year: HolidayOfYearList) -> bool {
        if let Some(old) = self.inner.get(&key) {
            if old.version >= holiday_of_year.version {
                return false;
            }
        }
        self.inner.insert(key, holiday_of_year);
        true
    }

    pub fn contains(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&HolidayOfYearList> {
        self.inner.get(key)
    }
}
