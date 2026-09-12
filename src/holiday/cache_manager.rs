use crate::holiday::modal::{parse_holidays_of_year, HolidayOfYearList};
use crate::holiday::utils::get_holiday_cache_file_path;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use color_eyre::eyre::{bail, Result};
use log::info;

/// 已命中的本地缓存内容及其文件修改时间。
pub struct CachedHoliday {
    pub data: HolidayOfYearList,
    pub modify_time: NaiveDateTime,
}

/// 本地假期缓存管理器，全部使用同步 IO。
pub struct HolidayCacheManager;

impl HolidayCacheManager {
    /// 读取并解析本地缓存；文件缺失或损坏时返回 None。
    pub fn load(year: &str, language: &str, country: &str) -> Option<CachedHoliday> {
        let path = get_holiday_cache_file_path(year, language, country)?;
        let metadata = std::fs::metadata(&path).ok()?;
        let modified = metadata.modified().ok()?;
        let modify_time: NaiveDateTime = DateTime::<Utc>::from(modified).naive_utc();
        let content = std::fs::read_to_string(&path).ok()?;
        let data = parse_holidays_of_year(&content).ok()?;
        Some(CachedHoliday { data, modify_time })
    }

    /// TTL 判定：修改时间在 10 天以内视为新鲜。
    pub fn is_fresh(modify_time: NaiveDateTime) -> bool {
        let now = Utc::now().naive_utc();
        modify_time > now - Duration::days(10)
    }

    /// 建目录并写盘。
    pub fn save(year: &str, language: &str, country: &str, content: &[u8]) -> Result<()> {
        let Some(path) = get_holiday_cache_file_path(year, language, country) else {
            bail!("get holiday cache file path failed")
        };
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, content)?;
        info!("Successfully saved holiday file to {}", path.display());
        Ok(())
    }
}
