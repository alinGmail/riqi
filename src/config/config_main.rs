use crate::config::cli::Args;
use super::{config_file_loader::load_file_config, locale, model::AppConfig};

pub fn get_app_config(args:Args) -> AppConfig {
    
    let (language, country_option) = locale::get_system_language_country();
    let file_config_opt = load_file_config();
    let mut app_config = AppConfig {
        language,
        country: String::from("cn"),
        column: None,
        row: None,
        show_lunar: Some(false),
        show_holiday: Some(false),
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
        if let Some(file_column) = file_config.column {
            app_config.column = Some(file_column);
        }
        if let Some(file_row) = file_config.row {
            app_config.row = Some(file_row);
        }
        if let Some(show_lunar) = file_config.show_lunar {
            app_config.show_lunar = Some(show_lunar);
        }
        if let Some(show_holiday) = file_config.show_holiday {
            app_config.show_holiday = Some(show_holiday);
        }
    }
    
    if let Some(arg_country) = args.country {
        app_config.country = arg_country
    }
    
    if let Some(arg_language) = args.language {
        app_config.language = arg_language
    }
    
    if let Some(arg_column) = args.column {
        app_config.column = Some(arg_column);
    }
    
    if let Some(arg_row) = args.row {
       app_config.row = Some(arg_row); 
    }
    
    if let Some(arg_show_lunar) = args.show_lunar {
        app_config.show_lunar = Some(arg_show_lunar);
    }
    
    if let Some(arg_show_holiday) = args.show_holiday {
        app_config.show_holiday = Some(arg_show_holiday);
    }
    

    app_config
}
