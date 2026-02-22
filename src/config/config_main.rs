use super::{config_file_loader::load_file_config, locale, model::AppConfig};
use crate::config::cli::Args;
use crate::config::model::Source;

pub fn get_app_config(args: Args) -> AppConfig {
    let (language, country_option) = locale::get_system_language_country();
    let file_config_opt = load_file_config();
    let mut app_config = AppConfig {
        language,
        country: String::from("cn"),
        column: None,
        row: None,
        show_lunar: false,
        show_holiday: false,
        output: "%Y-%m-%d".to_string(),
        source: Source::Github,
        hide_bg: true,
        theme: "ningmen".to_string(),
    };
    if let Some(country) = country_option {
        app_config.country = country
    }

    // 文件配置覆盖默认
    if let Some(file_config) = file_config_opt {
        if let Some(language) = file_config.language {
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
            app_config.show_lunar = show_lunar;
        }
        if let Some(show_holiday) = file_config.show_holiday {
            app_config.show_holiday = show_holiday;
        }
        if let Some(file_output) = file_config.output {
            app_config.output = file_output;
        }
        if let Some(source_str) = file_config.source {
            if let Ok(source) = source_str.parse::<Source>() {
                app_config.source = source;
            }
        }
        if let Some(hide_bg) = file_config.hide_bg {
            app_config.hide_bg = hide_bg;
        }
        if let Some(theme) = file_config.theme {
            app_config.theme = theme;
        }
    }

    if let Some(arg_country) = args.country {
        app_config.country = arg_country.to_string()
    }

    if let Some(arg_language) = args.language {
        app_config.language = arg_language.to_string()
    }

    if let Some(arg_column) = args.column {
        app_config.column = Some(arg_column);
    }

    if let Some(arg_row) = args.row {
        app_config.row = Some(arg_row);
    }

    if let Some(arg_show_lunar) = args.show_lunar {
        app_config.show_lunar = arg_show_lunar;
    }

    if let Some(arg_show_holiday) = args.show_holiday {
        app_config.show_holiday = arg_show_holiday;
    }

    if let Some(arg_output) = args.output {
        app_config.output = arg_output;
    }

    if let Some(arg_source) = args.source {
        if let Ok(source) = arg_source.parse::<Source>() {
            app_config.source = source;
        }
    }

    if let Some(arg_hide_bg) = args.hide_bg {
        app_config.hide_bg = arg_hide_bg;
    }

    if let Some(arg_theme) = args.theme {
        app_config.theme = arg_theme;
    }

    app_config
}
