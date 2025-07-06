use super::{
    types::MetaCache,
    utils::{get_holiday_cache_file_path, get_ylc_code},
};
use color_eyre::eyre::{eyre, OptionExt, Result};

pub fn load_holidays_file(year: &str, language: &str, country: &str) -> Result<String> {
    // 开发环境：从项目目录加载
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
