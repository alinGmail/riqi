use chrono::{Datelike, NaiveDate};

/// 给日期加 `n` 个月，如果目标月份没有该日期，则返回目标月份的最后一天
pub fn add_months_safe(date: NaiveDate, months: i32) -> NaiveDate {
    let mut year = date.year();
    let mut month = date.month() as i32 + months;
    let day = date.day();

    // 处理年份和月份溢出
    year += (month - 1).div_euclid(12);
    month = (month - 1).rem_euclid(12) + 1;

    // 检查目标月份是否有该日期（例如 2月31日 -> 无效）
    if let Some(d) = NaiveDate::from_ymd_opt(year, month as u32, day) {
        d // 如果日期有效，直接返回
    } else {
        // 如果无效，返回该月最后一天
        NaiveDate::from_ymd_opt(year, month as u32 + 1, 1) // 下个月的第1天
            .unwrap()
            .pred_opt() // 前一天 = 当前月最后一天
            .unwrap()
    }
}