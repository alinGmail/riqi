use std::path::PathBuf;
use crate::config::xdg::Xdg;

pub fn get_lc_code(language: &str, country: &str) -> String {
    return format!("{}_{}", language, country);
}

pub fn get_ylc_code(year: &str, language: &str, country: &str) -> String {
    return format!("{}_{}_{}", year, language, country);
}

pub fn get_holiday_cache_file_path(year: &str, language: &str, country: &str) -> Option<PathBuf> {
    let mut path = Xdg::cache_dir()?;
    path.push("holidays");
    path.push(year);
    path.push(format!("{}.json", get_lc_code(language, country)));
    Some(path)
}
