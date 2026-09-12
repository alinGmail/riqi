use crate::config::model::Source;
use crate::config::xdg::Xdg;
use crate::holiday::modal::{Holiday, PrimaryType};
use std::path::PathBuf;

/// 当前远端仓库实际提供数据的语言/地区组合。
/// 不在此列表中的 locale 会在本地快速失败，不做网络请求。
pub const SUPPORTED_LOCALES: &[&str] = &[
    "zh_cn", "en_cn", "en_de", "de_de", "en_fr", "fr_fr", "en_jp", "ja_jp", "en_kr", "ko_kr",
];

pub fn get_lc_code(language: &str, country: &str) -> String {
    format!("{}_{}", language.to_lowercase(), country.to_lowercase())
}

pub fn get_ylc_code(year: &str, language: &str, country: &str) -> String {
    format!("{}_{}", year, get_lc_code(language, country))
}

pub fn is_locale_supported(language: &str, country: &str) -> bool {
    SUPPORTED_LOCALES.contains(&get_lc_code(language, country).as_str())
}

pub fn get_holiday_cache_file_path(year: &str, language: &str, country: &str) -> Option<PathBuf> {
    let mut path = Xdg::cache_dir()?;
    path.push("holidays");
    path.push(year);
    path.push(format!("{}.json", get_lc_code(language, country)));
    Some(path)
}

pub fn get_holiday_data_file_url(
    year: &str,
    language: &str,
    country: &str,
    source: &Source,
) -> String {
    let lc_code = get_lc_code(language, country);
    match source {
        Source::Github => format!(
            "https://raw.githubusercontent.com/alinGmail/riqi/refs/heads/main/resources/holidays/{}/{}.json",
            year, lc_code
        ),
        Source::Gitee => format!(
            "https://gitee.com/zhaixiaolin/riqi/raw/main/resources/holidays/{}/{}.json",
            year, lc_code
        ),
    }
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
    (day_of_week == 6 || day_of_week == 0, false)
}
