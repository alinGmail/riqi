use super::types::HolidayResponse;
use super::types::MetaCache;
use color_eyre::eyre::eyre;
use color_eyre::eyre::Result;
use std::{fs, path::PathBuf};

pub fn get_lc_code(language: &str, country: &str) -> String {
    return format!("{}_{}", language, country);
}

pub fn get_ylc_code(year: &str, language: &str, country: &str) -> String {
    return format!("{}_{}_{}", year, language, country);
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

pub fn parse_holidays(json_str: &str) -> Result<HolidayResponse, serde_json::Error> {
    serde_json::from_str(json_str)
}
