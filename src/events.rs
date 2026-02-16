use crate::holiday::modal::HolidayOfYearList;
use crate::state::{GotoPanelState, NotificationMessage, RiqiMode, RiqiState};
use crate::utils::add_months_safe;
use chrono::{Datelike, Duration, Local};
use crossterm::event::{Event, KeyCode, KeyEvent};

// 统一的事件枚举：合并了 UI 事件和业务数据事件
pub enum AppEvent {
    Quit,
    TerminalEvent(Event),
    UpdateHoliday(String, HolidayOfYearList),
    AddNotification(NotificationMessage),
    RemoveNotification(NotificationMessage),
}

pub fn handle_normal_mode_key_event(key: KeyEvent, riqi_state: &mut RiqiState) {
    let now = Local::now();

    if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
        return;
    }
    if key.code == KeyCode::Char('j') || key.code == KeyCode::Down {
        riqi_state.select_day += Duration::weeks(1);
    }
    if key.code == KeyCode::Char('k') || key.code == KeyCode::Up {
        riqi_state.select_day += Duration::weeks(-1);
    }
    if key.code == KeyCode::Char('h') || key.code == KeyCode::Left {
        riqi_state.select_day += Duration::days(-1);
    }
    if key.code == KeyCode::Char('l') || key.code == KeyCode::Right {
        riqi_state.select_day += Duration::days(1);
    }
    if key.code == KeyCode::Char('d') {
        riqi_state.select_day = add_months_safe(riqi_state.select_day, 1);
    }
    if key.code == KeyCode::Char('u') {
        riqi_state.select_day = add_months_safe(riqi_state.select_day, -1);
    }
    if key.code == KeyCode::Char('f') {
        riqi_state.select_day = add_months_safe(riqi_state.select_day, 12);
    }
    if key.code == KeyCode::Char('b') {
        riqi_state.select_day = add_months_safe(riqi_state.select_day, -12);
    }
    if key.code == KeyCode::Char('t') {
        riqi_state.select_day = now.date_naive();
    }

    if key.code == KeyCode::Char('g') {
        riqi_state.mode = RiqiMode::Goto;
        riqi_state.goto_panel = GotoPanelState {
            year: riqi_state.select_day.year() as u16,
            month: riqi_state.select_day.month() as u8,
            day: riqi_state.select_day.day() as u8,
            focus_inp: 0,
        }
    }
}

pub fn handle_goto_mode_key_event(key: KeyEvent, riqi_state: &mut RiqiState) {
    if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
        riqi_state.mode = RiqiMode::Normal;
    }
    if key.code == KeyCode::Char('j') || key.code == KeyCode::Down {
        match riqi_state.goto_panel.focus_inp {
            0 => riqi_state.goto_panel.year -= 1,
            1 => {
                riqi_state.goto_panel.month += 11;
                riqi_state.goto_panel.month = (riqi_state.goto_panel.month - 1) % 12 + 1;
            }
            2 => {
                riqi_state.goto_panel.day += 30;
                riqi_state.goto_panel.day = (riqi_state.goto_panel.day - 1) % 31 + 1;
            }
            _ => (),
        }
    }
    if key.code == KeyCode::Char('k') || key.code == KeyCode::Up {
        match riqi_state.goto_panel.focus_inp {
            0 => {
                riqi_state.goto_panel.year += 1;
            }
            1 => {
                riqi_state.goto_panel.month += 1;
                riqi_state.goto_panel.month = (riqi_state.goto_panel.month - 1) % 12 + 1;
            }
            2 => {
                riqi_state.goto_panel.day += 1;
                riqi_state.goto_panel.day = (riqi_state.goto_panel.day - 1) % 31 + 1;
            }
            _ => (),
        }
    }
    if key.code == KeyCode::Char('h') || key.code == KeyCode::Left {
        riqi_state.goto_panel.focus_inp += 2;
        riqi_state.goto_panel.focus_inp = riqi_state.goto_panel.focus_inp % 3;
    }
    if key.code == KeyCode::Char('l') || key.code == KeyCode::Right {
        riqi_state.goto_panel.focus_inp += 1;
        riqi_state.goto_panel.focus_inp = riqi_state.goto_panel.focus_inp % 3;
    }
}
