use crate::{
    events::MessageBus,
    holiday::{
        update::{update_holiday_data, update_meta},
        utils::{load_cached_meta_file, parse_holidays},
    },
};
use color_eyre::eyre::{eyre, OptionExt, Result};
use std::collections::HashMap;

use super::{
    types::{HolidayMap, MetaCache},
    utils::{get_holiday_cache_file_path, get_ylc_code},
};

pub fn load_holidays_file(year: &str, language: &str, country: &str) -> Result<String> {
    let path = get_holiday_cache_file_path(year, language, country)
        .ok_or_eyre("get holiday cache file failed")?;
    let content = std::fs::read_to_string(path.as_path())?;
    Ok(content)
}

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

pub fn load_holidays(
    user_defined_country: bool,
    country_opt: &Option<String>,
    language: &str,
    year: &String,
    message_bus: &MessageBus,
) -> Option<HolidayMap> {
    let mut holiday_map: HolidayMap = HashMap::new();
    let country = match country_opt {
        Some(cou) => cou,
        None => return None,
    };
    let meta_cache_res = load_cached_meta_file();
    let meta_cache = match meta_cache_res {
        Ok(Some(cache)) => cache,
        _ => {
            let tx = message_bus.get_sender();
            tokio::spawn(async move {
                update_meta(tx).await;
            });
            return None;
        }
    };

    let holiday_code_result = get_holiday_code(
        meta_cache,
        user_defined_country,
        country_opt,
        language,
        year,
    );

    let (language, country) = match holiday_code_result {
        Ok((language, country)) => (language, country),
        Err(err_str) => {
            log::error!("获取假期区域代码失败: {}", err_str);
            return None;
        }
    };

    let file_str = match load_holidays_file(year, &language, &country) {
        Ok(content) => content,
        Err(e) => {
            log::error!(
                "加载假期文件 {} 失败: {}",
                get_ylc_code(year, &language, &country),
                e
            );
            let tx = message_bus.get_sender();
            let year_clone = year.to_string();
            let language_clone = language.to_string();
            let country_clone = country.to_string();
            tokio::spawn(async move {
                update_holiday_data(&year_clone, &language_clone, &country_clone, tx).await;
            });
            return None;
        }
    };

    let holiday_response = match parse_holidays(&file_str) {
        Ok(data) => data,
        Err(e) => {
            log::error!("解析假期数据失败: {}", e);
            return None;
        }
    };

    holiday_response.add_to_holiday_map(&mut holiday_map, year, &language, &country);
    Some(holiday_map)
}