use chrono::NaiveDate;

#[derive(Debug)]
pub struct RiqiState {
    pub select_day: NaiveDate,
    pub today: NaiveDate,

}
