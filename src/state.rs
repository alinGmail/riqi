use chrono::NaiveDate;
use serde::Deserialize;
use crate::theme::theme_model::Theme;


#[derive(Debug)]
pub enum RiqiMode {
    Normal,
    Goto,
}


#[derive(Debug)]
pub struct GotoPanelState{
    pub year:u16,
    pub month: u8,
    pub day: u8,
    pub focus_inp: u8,
}

#[derive(Debug)]
pub struct RiqiState {
    pub select_day: NaiveDate,
    pub today: NaiveDate,
    pub theme: Theme,
    pub mode: RiqiMode,
    pub goto_panel: GotoPanelState,
}
