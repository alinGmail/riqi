use chrono::NaiveDate;

use crate::theme::theme_model::Theme;

#[derive(Debug)]
pub struct RiqiState {
    pub select_day: NaiveDate,
    pub today: NaiveDate,
    pub theme: Theme,
}
