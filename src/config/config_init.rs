use sys_locale::get_locale;

use crate::locale::parse_language_country;
use crate::{cli::Args, i18n::Language};

use super::{
    config_struct::{CalendarType, Config},
    file_config_struct::FileConfig,
};

pub fn get_config(
    sys_language: &str,
    sys_country: &Option<String>,
    file_config: &Option<FileConfig>,
    args: &Args,
) -> Config {
    let country = get_country(sys_country, file_config, args);
    let language = get_language(sys_language, file_config, args);

    let calendar_type = get_calendar_type(&country);
    let show_lunar = get_show_lunar(&country, file_config, args);

    let config = Config {
        language,
        country,
        calendar_type,
        show_lunar,
    };

    config
}

fn get_language(sys_language: &str, file_config: &Option<FileConfig>, args: &Args) -> String {
    if let Some(language) = &args.language {
        return language.clone();
    }
    if let Some(config) = file_config {
        if let Some(language) = &config.language {
            return language.clone();
        }
    }
    sys_language.to_string()
}

fn get_country(
    sys_country: &Option<String>,
    file_config: &Option<FileConfig>,
    args: &Args,
) -> Option<String> {
    if let Some(country) = &args.country {
        return Some(country.clone());
    }

    if let Some(config) = file_config {
        if let Some(country) = &config.country {
            return Some(country.clone());
        }
    }
    sys_country.clone()
}

fn get_show_lunar(
    final_country: &Option<String>,
    file_config: &Option<FileConfig>,
    args: &Args,
) -> bool {
    if let Some(config) = file_config {
        if let Some(show_lunar) = config.show_lunar {
            return show_lunar;
        }
    }
    if let Some(country) = final_country {
        if country == "cn" {
            return true;
        }
    }
    return false;
}

fn get_calendar_type(final_country: &Option<String>) -> CalendarType {
    if let Some(country) = final_country {
        if country == "cn" {
            return CalendarType::Normal;
        }
    }
    CalendarType::WideScreen
}

// 获取系统的语言和国家
// 返回 (language, Option<country>)，
pub fn get_system_language_country() -> (String, Option<String>) {
    let locale = get_locale().unwrap_or_else(|| String::from("en-US"));
    parse_language_country(&locale)
}
