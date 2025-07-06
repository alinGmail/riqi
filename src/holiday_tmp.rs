use crate::holiday_utils::{get_lc_code, get_ylc_code};
use crate::types::holiday::{parse_holidays, HolidayMap};
use crate::types::holiday_meta::{HolidayMeta, MetaCache};

use chrono::{TimeDelta, Utc};
use color_eyre::eyre::{eyre, OptionExt, Result};
use std::fs;
use std::path::PathBuf;

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
