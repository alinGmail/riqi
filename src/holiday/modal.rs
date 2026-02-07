use crate::holiday::utils::get_ylc_code;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub enum HolidayLoadStatus {
    Loading,
    Finish,
    Fail,
}
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Holiday {
    pub name: String,
    pub date: Date,
    #[serde(rename = "type")]
    pub holiday_type: Vec<String>,
    pub primary_type: PrimaryType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Date {
    pub iso: String,
    pub datetime: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
pub struct HolidayOfYearList {
    pub version: i32,
    pub holidays: Vec<Holiday>,
}

impl HolidayOfYearList {
    pub fn to_holiday_map(
        &self,
    ) -> HashMap<String, Vec<Holiday>> {
        let mut date_map: HashMap<String, Vec<Holiday>> = HashMap::new();
        for holiday in &self.holidays {
            let date_key = holiday.date.iso.clone();
            date_map.entry(date_key).or_default().push(holiday.clone());
        }
        return date_map;
    }
}

pub fn parse_holidays_of_year(json_str: &str) -> Result<HolidayOfYearList, serde_json::Error> {
    serde_json::from_str(json_str)
}
