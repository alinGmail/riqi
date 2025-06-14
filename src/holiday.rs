pub fn load_holidays_file(holiday_code: &String) -> std::io::Result<String> {
    // 开发环境：从项目目录加载
    #[cfg(debug_assertions)]
    {
        let path = format!("resources/holidays/{}.json", holiday_code);
        log::debug!("load path {}", path);
        std::fs::read_to_string(path)
    }

    // 生产环境：从嵌入资源或系统目录加载
    #[cfg(not(debug_assertions))]
    {
        // 方法1：嵌入资源
        match (lang, country) {
            ("zh", "cn") => Ok(include_str!("../resources/holidays/cn_zh.json").to_string()),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Holiday file not found",
            )),
        }

        // 或方法2：从系统目录加载
        // let system_path = get_system_data_path(lang, country);
        // std::fs::read_to_string(system_path)
    }
}

pub const AVAILABLE_HOLIDAY_REGION: [&str; 6] =
    ["de_de", "en_jp", "fr_fr", "ja_jp", "ko_kr", "zh_cn"];

pub fn get_holiday_code(
    user_defined_country: bool,
    country_opt: &Option<String>,
    language: &str,
) -> Result<Option<String>, String> {
    let country = match country_opt {
        Some(c) => c.to_lowercase(),
        None => return Ok(None),
    };
    let language = language.to_lowercase();
    let language_country_str = format!("{}_{}", language, country);
    let en_country_str = format!("en_{}", country);

    if user_defined_country {
        if AVAILABLE_HOLIDAY_REGION.contains(&language_country_str.as_str()) {
            Ok(Some(language_country_str))
        } else {
            Err(format!("cannot find holidays of {}", language_country_str))
        }
    } else if AVAILABLE_HOLIDAY_REGION.contains(&language_country_str.as_str()) {
        Ok(Some(language_country_str))
    } else if AVAILABLE_HOLIDAY_REGION.contains(&en_country_str.as_str()) {
        Ok(Some(en_country_str))
    } else {
        Ok(None)
    }
}
