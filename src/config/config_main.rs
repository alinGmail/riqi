use super::{locale, model::AppConfig};

pub fn get_app_config() -> AppConfig {
    let (language, country_option) = locale::get_system_language_country();
    let mut app_config = AppConfig {
        language,
        country: String::from("cn"),
    };
    if let Some(country) = country_option {
        app_config.country = country
    }

    app_config
}
