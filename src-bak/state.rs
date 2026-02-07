use chrono::NaiveDate;

use crate::{
    config::config_struct::Config, events::MessageBus, holiday::types::HolidayMap, theme::Theme,
};

#[derive(Debug)]
pub struct RiqiState {
    pub select_day: NaiveDate,
    pub holiday_map: HolidayMap,
    pub today: NaiveDate,
    pub config: Config,
    pub theme: Theme,
    pub message_bus: MessageBus,
}
