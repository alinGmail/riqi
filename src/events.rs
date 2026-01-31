use crossterm::event::Event;
use crate::holiday::modal::HolidayOfYearList;

// 统一的事件枚举：合并了 UI 事件和业务数据事件
pub enum AppEvent {
    Quit,
    TerminalEvent(Event),
    UpdateHoliday(String, HolidayOfYearList),
}
