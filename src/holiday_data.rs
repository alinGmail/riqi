use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Holiday {
    pub name: String,
    pub date: Date,
    #[serde(rename = "type")]
    pub holiday_type: Vec<String>,
    pub primary_type: String,
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