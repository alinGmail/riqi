use crate::config::xdg::Xdg;
use crate::holiday::modal::{Holiday, PrimaryType};
use std::path::PathBuf;

pub fn get_lc_code(language: &str, country: &str) -> String {
    return format!("{}_{}", language, country);
}

pub fn get_ylc_code(year: &str, language: &str, country: &str) -> String {
    return format!("{}_{}_{}", year, language, country);
}

pub fn get_holiday_cache_file_path(year: &str, language: &str, country: &str) -> Option<PathBuf> {
    let mut path = Xdg::cache_dir()?;
    path.push("holidays");
    path.push(year);
    path.push(format!("{}.json", get_lc_code(language, country)));
    Some(path)
}

// 判断今天是否是节日，
// return (是否放假, true 放假，false 上班:bool  | 是否国家节日,用于是否显示图标:bool)
pub fn get_holiday_state(holidays: &Option<Vec<Holiday>>, day_of_week: u16) -> (bool, bool) {
    if let Some(holiday_vec) = holidays {
        let is_holiday = holiday_vec.iter().any(|holiday| {
            matches!(
                holiday.primary_type,
                PrimaryType::SubstituteHoliday | PrimaryType::NationalHoliday
            )
        });
        if is_holiday {
            return (true, true);
        }
        let is_workday = holiday_vec
            .iter()
            .any(|holiday| matches!(holiday.primary_type, PrimaryType::WorkingDayOnWeekend));
        if is_workday {
            return (false, true);
        }
    };
    (
        day_of_week == 6 || day_of_week == 0,
        false,
    )
}
