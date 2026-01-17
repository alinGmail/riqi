use crate::config::cli::Args;
use super::{config_file_loader::load_file_config, locale, model::AppConfig};

pub fn get_app_config(args:Args) -> AppConfig {
    
    let (language, country_option) = locale::get_system_language_country();
    let file_config_opt = load_file_config();
    let mut app_config = AppConfig {
        language,
        country: String::from("cn"),
    };
    if let Some(country) = country_option {
        app_config.country = country
    }
    
    // 文件配置覆盖默认
    if let Some(file_config) = file_config_opt {
        if let Some(language) = file_config.language{
            app_config.language = language;
        }
        if let Some(country) = file_config.country {
            app_config.country = country;
        }
    }
    
    if let Some(arg_country) = args.country {
        app_config.country = arg_country
    }
    
    if let Some(arg_language) = args.language {
        app_config.language = arg_language
    }
    

    app_config
}
