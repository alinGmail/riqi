use crate::holiday_utils::get_ylc_code;
use chrono::Utc;
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
    pub cache_time: chrono::DateTime<Utc>,
}

pub type HolidayMap = HashMap<String, HashMap<String, Vec<Holiday>>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PrimaryType {
    #[serde(rename = "Substitute holiday")]
    SubstituteHoliday,
    #[serde(rename = "National holiday")]
    NationalHoliday,
    #[serde(rename = "Common holiday")]
    CommonHoliday,
    #[serde(rename = "Working Day on a Weekend")]
    WorkingDayOnWeekend,
    #[serde(rename = "Observance")]
    Observance,
    #[serde(rename = "Season")]
    Season,
    #[serde(rename = "Half day holiday")]
    HalfDayHoliday,
    #[serde(rename = "Regional holiday")]
    RegionalHoliday,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Holiday {
    pub name: String,
    pub date: Date,
    #[serde(rename = "type")]
    pub holiday_type: Vec<String>,
    pub primary_type: PrimaryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    pub iso: String,
    pub datetime: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateTime {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    #[serde(default)]
    pub hour: Option<i32>,
    #[serde(default)]
    pub minute: Option<i32>,
    #[serde(default)]
    pub second: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HolidayResponse {
    pub holidays: Vec<Holiday>,
}

pub fn parse_holidays(json_str: &str) -> Result<HolidayResponse, serde_json::Error> {
    serde_json::from_str(json_str)
}

impl HolidayResponse {
    pub fn add_to_holiday_map(
        &self,
        holiday_map: &mut HolidayMap,
        year: &str,
        languange: &str,
        country: &str,
    ) {
        let key = get_ylc_code(year, languange, country);
        let mut date_map: HashMap<String, Vec<Holiday>> = HashMap::new();

        // 按日期对节假日进行分组
        for holiday in &self.holidays {
            let date_key = holiday.date.iso.clone();
            date_map.entry(date_key).or_default().push(holiday.clone());
        }

        // 将分组后的数据插入到 holiday_map 中，如果已存在则替换
        holiday_map.insert(key, date_map);
    }
}

// 为 Holiday 实现 Clone trait
impl Clone for Holiday {
    fn clone(&self) -> Self {
        Holiday {
            name: self.name.clone(),
            date: Date {
                iso: self.date.iso.clone(),
                datetime: DateTime {
                    year: self.date.datetime.year,
                    month: self.date.datetime.month,
                    day: self.date.datetime.day,
                    hour: self.date.datetime.hour,
                    minute: self.date.datetime.minute,
                    second: self.date.datetime.second,
                },
            },
            holiday_type: self.holiday_type.clone(),
            primary_type: self.primary_type.clone(),
        }
    }
}

// 为 Date 实现 Clone trait
impl Clone for Date {
    fn clone(&self) -> Self {
        Date {
            iso: self.iso.clone(),
            datetime: DateTime {
                year: self.datetime.year,
                month: self.datetime.month,
                day: self.datetime.day,
                hour: self.datetime.hour,
                minute: self.datetime.minute,
                second: self.datetime.second,
            },
        }
    }
}

// 为 DateTime 实现 Clone trait
impl Clone for DateTime {
    fn clone(&self) -> Self {
        DateTime {
            year: self.year,
            month: self.month,
            day: self.day,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
        }
    }
}
