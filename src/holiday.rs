use crate::types::holiday::{parse_holidays, HolidayMap};
use crate::types::holiday_meta::{HolidayMeta, MetaCache};

use chrono::{TimeDelta, Utc};
use color_eyre::eyre::{eyre, Result};
use std::fs;
use std::path::PathBuf;

/// 根据假期代码加载对应的假期文件。
///
/// 该函数在开发环境和生产环境下有不同的行为：
/// - **开发环境 (`debug_assertions`)**: 从项目目录 `resources/holidays/2025/` 加载文件。
/// - **生产环境**: 从编译时嵌入的资源中加载。
///
/// # Arguments
///
/// * `holiday_code` - 假期代码字符串 (例如, "zh_cn")。
///
/// # Returns
///
/// * `std::io::Result<String>` - 成功时返回文件内容的 `Result`，失败时返回 `std::io::Error`。
pub fn load_holidays_file(holiday_code: &String) -> std::io::Result<String> {
    // 开发环境：从项目目录加载
    #[cfg(debug_assertions)]
    {
        let path = format!("resources/holidays/2025/{}.json", holiday_code);
        log::debug!("load path {}", path);
        std::fs::read_to_string(path)
    }

    // 生产环境：从嵌入资源加载
    #[cfg(not(debug_assertions))]
    {
        // 从 holiday_code 解析 lang 和 country
        let parts: Vec<&str> = holiday_code.split('_').collect();
        let lang = parts.get(0).cloned().unwrap_or("");
        let country = parts.get(1).cloned().unwrap_or("");

        // The path is relative to this source file.
        match (lang, country) {
            ("zh", "cn") => {
                Ok(include_str!("../../resources/holidays/2025/zh_cn.json").to_string())
            }
            ("de", "de") => {
                Ok(include_str!("../../resources/holidays/2025/de_de.json").to_string())
            }
            ("en", "jp") => {
                Ok(include_str!("../../resources/holidays/2025/en_jp.json").to_string())
            }
            ("fr", "fr") => {
                Ok(include_str!("../../resources/holidays/2025/fr_fr.json").to_string())
            }
            ("ja", "jp") => {
                Ok(include_str!("../../resources/holidays/2025/ja_jp.json").to_string())
            }
            ("ko", "kr") => {
                Ok(include_str!("../../resources/holidays/2025/ko_kr.json").to_string())
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "Holiday file for '{}' not found for release build",
                    holiday_code
                ),
            )),
        }
    }
}

/// 根据用户配置和系统语言确定要使用的假期代码。
///
/// # Arguments
///
/// * `user_defined_country` - 一个布尔值，指示国家/地区是否由用户明确定义。
/// * `country_opt` - `Option<String>`，包含用户定义的国家/地区代码。
/// * `language` - 当前的语言代码。
///
/// # Returns
///
/// * `Result<Option<String>, String>` - 成功时返回 `Ok(Some(String))`（如果找到有效的代码）或 `Ok(None)`（如果未找到），失败时返回 `Err(String)`。
pub fn get_holiday_code(
    meta_cache: MetaCache,
    user_defined_country: bool,
    country_opt: &Option<String>,
    language: &str,
    year: &str,
) -> Result<Option<String>, String> {
    let country = match country_opt {
        Some(c) => c.to_lowercase(),
        None => return Ok(None),
    };
    let language = language.to_lowercase();
    let language_country_str = format!("{}_{}_{}", year, language, country);
    let en_country_str = format!("{}_en_{}", year, country);
    let available_holiday_region = meta_cache.data.get_available_year_holiday_keys();

    if user_defined_country {
        if available_holiday_region.contains(&language_country_str) {
            Ok(Some(language_country_str))
        } else {
            Err(format!("cannot find holidays of {}", language_country_str))
        }
    } else if available_holiday_region.contains(&language_country_str) {
        Ok(Some(language_country_str))
    } else if available_holiday_region.contains(&en_country_str) {
        Ok(Some(en_country_str))
    } else {
        Ok(None)
    }
}

/// 加载、解析假期数据并将其填充到 `HolidayMap` 中。
///
/// # Arguments
///
/// * `holiday_code_result` - `get_holiday_code` 函数的结果。
/// * `holiday_map` - 一个可变的 `HolidayMap` 引用，用于存储假期数据。
/// * `year` - 需要加载假期的年份。
pub fn load_holidays(
    user_defined_country: bool,
    country_opt: &Option<String>,
    language: &str,
    holiday_map: &mut HolidayMap,
    year: &String,
) {
    let meta_cache_res = load_cached_meta_file();
    let meta_cache = match meta_cache_res {
        Ok(Some(cache)) => cache,
        _ => return, // 出错或空就直接返回
    };

    let holiday_code_result: Result<Option<String>, String> = get_holiday_code(
        meta_cache,
        user_defined_country,
        country_opt,
        language,
        year,
    );
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
            log::error!("加载假期文件 '{}' 失败: {}", &code, e);
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

/// 从缓存中获取假期元数据。
///
/// 如果缓存存在且在一天内是有效的，则返回缓存的元数据。
///
/// # Returns
///
/// * `Result<Option<HolidayMeta>>` - 成功时返回 `Ok(Some(HolidayMeta))` 或 `Ok(None)`，失败时返回 `eyre::Result`。
pub fn get_cached_meta() -> Result<Option<HolidayMeta>> {
    match load_cached_meta_file() {
        Ok(Some(cache)) => {
            if (Utc::now() - cache.cache_time) < TimeDelta::days(1) {
                Ok(Some(cache.data))
            } else {
                log::debug!("Meta cache is outdated.");
                Ok(None)
            }
        }
        Ok(None) => Ok(None), // No cache file exists
        Err(e) => {
            log::warn!("Failed to load meta cache: {}", e);
            Err(e)
        }
    }
}

/// 从文件系统加载缓存的元数据。
///
/// # Returns
///
/// * `Result<Option<MetaCache>>` - 成功时返回 `Ok(Some(MetaCache))` 或 `Ok(None)`，失败时返回 `eyre::Result`。
pub fn load_cached_meta_file() -> Result<Option<MetaCache>> {
    let cache_path = get_meta_cache_path().ok_or_else(|| eyre!("Failed to get cache path"))?;
    if !cache_path.exists() {
        return Ok(None);
    }
    let json = fs::read_to_string(cache_path)?;
    let cache: MetaCache = serde_json::from_str(&json)?;
    Ok(Some(cache))
}

/// 获取元数据缓存文件的路径。
///
/// 该函数会确保缓存目录存在。
///
/// # Returns
///
/// * `Option<PathBuf>` - 如果成功，返回缓存文件的路径 `PathBuf`。
pub fn get_meta_cache_path() -> Option<PathBuf> {
    dirs::cache_dir().and_then(|mut path| {
        path.push("riqi");
        path.push("holidays");
        fs::create_dir_all(&path).ok()?;
        path.push("meta_cache.json");
        Some(path)
    })
}
