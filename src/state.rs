use chrono::NaiveDate;

use crate::{config::config_struct::Config, holiday::types::HolidayMap, theme::Theme};

#[derive(Debug)]
pub struct RiqiState<'a> {
    pub select_day: NaiveDate,
    pub holiday_map: &'a HolidayMap,
    pub today: NaiveDate,
    pub config: &'a Config,
    pub theme: &'a Theme,
}
