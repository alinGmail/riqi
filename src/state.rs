use crate::theme::theme_model::Theme;
use crate::config::model::AppConfig;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug)]
pub enum RiqiMode {
    Normal,
    Goto,
}

#[derive(Debug, Clone)]
pub struct NotificationMessage {
    pub id: String,
    pub message: String,
}

#[derive(Debug)]
pub struct GotoPanelState {
    pub year: u16,
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
    pub notification: Vec<NotificationMessage>,
}
