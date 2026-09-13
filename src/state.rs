use crate::config::model::AppConfig;
use crate::theme::theme_model::Theme;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug)]
pub enum RiqiMode {
    Normal,
    Goto,
    Config,
    ThemeSelect,
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
pub struct ConfigPanelState {
    pub focus: usize,
}

#[derive(Debug)]
pub struct ThemeSelectState {
    pub selected: usize,
    pub original_theme: Theme,
}

#[derive(Debug)]
pub struct RiqiState {
    pub select_day: NaiveDate,
    pub today: NaiveDate,
    pub theme: Theme,
    pub theme_name: String,
    pub theme_names: Vec<&'static str>,
    pub mode: RiqiMode,
    pub goto_panel: GotoPanelState,
    pub config_panel: ConfigPanelState,
    pub theme_select: ThemeSelectState,
    pub notification: Vec<NotificationMessage>,
}
