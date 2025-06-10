use sys_locale::get_locale;

use crate::locale::parse_language_country;

use super::config_struct::{CalendarType, Config};

pub fn get_default_config() -> Config {
    let (sys_language, sys_country) = get_system_language_country();

    let mut calendar_type = CalendarType::WideScreen;
    let mut show_lunar = false;
    if let Some(country) = &sys_country {
        if country.to_lowercase().as_str() == "cn" {
            calendar_type = CalendarType::Normal;
            show_lunar = true;
        }
    };

    Config {
        language: sys_language.to_lowercase(),
        country: sys_country
            .unwrap_or_else(|| String::from("us"))
            .to_lowercase(),
        calendar_type,
        show_lunar,
    }
}

// 获取系统的语言和国家
// 返回 (language, Option<country>)，
pub fn get_system_language_country() -> (String, Option<String>) {
    let locale = get_locale().unwrap_or_else(|| String::from("en-US"));
    parse_language_country(&locale)
}
