use crate::config::cli::Args;
use super::{locale, model::AppConfig};

pub fn get_app_config(args:Args) -> AppConfig {
    
    let (language, country_option) = locale::get_system_language_country();
    let mut app_config = AppConfig {
        language,
        country: String::from("cn"),
    };
    if let Some(country) = country_option {
        app_config.country = country
    }
    
    if let Some(arg_country) = args.country {
        app_config.country = arg_country
    }
    
    if let Some(arg_language) = args.language {
        app_config.language = arg_language
    }

    app_config
}
