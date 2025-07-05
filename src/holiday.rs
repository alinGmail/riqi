use crate::holiday_utils::{get_lc_code, get_ylc_code};
use crate::types::holiday::{parse_holidays, HolidayMap};
use crate::types::holiday_meta::{HolidayMeta, MetaCache};

use chrono::{TimeDelta, Utc};
use color_eyre::eyre::{eyre, OptionExt, Result};
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
pub fn load_holidays_file(year: &str, language: &str, country: &str) -> Result<String> {
    // 开发环境：从项目目录加载
    let path = get_holiday_cache_file_path(year, language, country)
        .ok_or_eyre("get holiday cache file failed")?;
    let content = std::fs::read_to_string(path.as_path())?;
    Ok(content)
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
/// 返回 (language, country)
pub fn get_holiday_code(
    meta_cache: MetaCache,
    user_defined_country: bool,
    country_opt: &Option<String>,
    language: &str,
    year: &str,
) -> Result<(String, String)> {
    let country = match country_opt {
        Some(c) => c.to_lowercase(),
        None => return Err(eyre!("country is empty")),
    };
    let language = language.to_lowercase();
    let language_country_str = get_ylc_code(year, &language, &country);
    let en_country_str = get_ylc_code(year, "en", &country);
    let available_holiday_region = meta_cache.data.get_available_year_holiday_keys();

    if user_defined_country {
        if available_holiday_region.contains(&language_country_str) {
            Ok((language, country))
        } else {
            return Err(eyre!("cannot find holidays of {}", language_country_str));
        }
    } else if available_holiday_region.contains(&language_country_str) {
        Ok((language, country))
    } else if available_holiday_region.contains(&en_country_str) {
        Ok((String::from("en"), country))
    } else {
        return Err(eyre!("no available holiday data found"));
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
    let country = match country_opt {
        Some(cou) => cou,
        None => return,
    };
    let meta_cache_res = load_cached_meta_file();
    let meta_cache = match meta_cache_res {
        Ok(Some(cache)) => cache,
        _ => return, // 出错或空就直接返回
    };

    let holiday_code_result = get_holiday_code(
        meta_cache,
        user_defined_country,
        country_opt,
        language,
        year,
    );
    // 1. 获取 code
    let (languange, country) = match holiday_code_result {
        Ok((language, country)) => (language, country),
        Err(err_str) => {
            log::error!("获取假期区域代码失败: {}", err_str);
            return;
        }
    };
    // 2. 加载假期文件
    let file_str = match load_holidays_file(year, language, &country) {
        Ok(content) => content,
        Err(e) => {
            log::error!(
                "加载假期文件 {} 失败: {}",
                get_ylc_code(year, language, &country),
                e
            );
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
    holiday_response.add_to_holiday_map(holiday_map, year, &languange, &country);
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

/// 获取 holiday 文件的位置
///
/// holiday_ylc_str: 代表 年_语言_国家 的字符串，例如 2025_zh_cn
///
pub fn get_holiday_cache_file_path(year: &str, language: &str, country: &str) -> Option<PathBuf> {
    dirs::cache_dir().map(|mut path| {
        path.push("riqi");
        path.push("holidays");
        path.push(year);
        path.push(format!("{}.json", get_lc_code(language, country)));
        path
    })
}
