use crate::types::holiday::{parse_holidays, HolidayMap};
use crate::types::holiday_meta::{HolidayMeta, MetaCache};

use chrono::{TimeDelta, Utc};
use color_eyre::eyre::{eyre, Result};
use std::fs;
use std::path::PathBuf;

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

pub fn load_holidays(
    holiday_code_result: Result<Option<String>, String>,
    holiday_map: &mut HolidayMap,
    year: &String,
) {
    // 1. 获取 code
    let code = match holiday_code_result {
        Ok(Some(code)) => code,
        Ok(None) => {
            log::warn!("未找到假期区域代码");
            return;
        }
        Err(err_str) => {
            log::error!("获取假期区域代码失败: {}", err_str);
            return;
        }
    };
    // 2. 加载假期文件
    let file_str = match load_holidays_file(&code) {
        Ok(content) => content,
        Err(e) => {
            log::error!("加载假期文件失败: {}", e);
            return;
        }
    };
    // 3. 解析假期
    let holiday_response = match parse_holidays(&file_str) {
        Ok(data) => data,
        Err(e) => {
            log::error!("解析假期数据失败: {}", e);
            return;
        }
    };

    // 4. 正常处理假期数据
    log::debug!(
        "成功解析假期数据，共 {} 个假期",
        holiday_response.holidays.len()
    );
    holiday_response.add_to_holiday_map(holiday_map, &code, &year);
}

pub fn get_cached_meta() -> Result<Option<HolidayMeta>> {
    let cache = load_cached_meta_file().unwrap().unwrap();
    if (Utc::now() - cache.cache_time) < TimeDelta::days(1) {
        Ok(Some(cache.data))
    } else {
        Ok(None)
    }
}

pub fn load_cached_meta_file() -> Result<Option<MetaCache>> {
    let cache_path = get_meta_cache_path().ok_or_else(|| eyre!("Failed to get cache path"))?;
    if !cache_path.exists() {
        return Ok(None);
    }
    let json = fs::read_to_string(cache_path)?;
    let cache: MetaCache = serde_json::from_str(&json)?;
    Ok(Some(cache))
}

pub fn get_meta_cache_path() -> Option<PathBuf> {
    dirs::cache_dir().and_then(|mut path| {
        path.push("riqi");
        path.push("holidays");
        fs::create_dir_all(&path).ok()?;
        path.push("meta_cache.json");
        Some(path)
    })
}
