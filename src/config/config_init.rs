use sys_locale::get_locale;

use crate::locale::parse_language_country;

use super::config_struct::Config;

pub fn get_default_config() -> Config {
    let locale = get_locale().unwrap_or_else(|| String::from("en-US"));

    let (language, country) = parse_language_country(&locale);
    Config {
        language: language.to_lowercase(),
        country: country.unwrap_or_else(|| String::from("us")).to_lowercase(),
    }
}
