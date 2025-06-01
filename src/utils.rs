use chrono::{Datelike, NaiveDate};

/// 给日期加 `n` 个月，如果目标月份没有该日期，则返回目标月份的最后一天
pub fn add_months_safe(date: NaiveDate, months: i32) -> NaiveDate {
    let mut year = date.year();
    let mut month = date.month() as i32 + months;
    let day = date.day();

    // 处理年份溢出（例如 12月 + 3个月 = 次年3月）
    while month > 12 {
        year += 1;
        month -= 12;
    }

    while month < 1 {
        year -= 1;
        month += 12;
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_add_months_normal() {
        // 常规情况：1月 -> 2月（非闰年）
        let date = NaiveDate::from_ymd_opt(2023, 1, 15).unwrap();
        assert_eq!(
            add_months_safe(date, 1),
            NaiveDate::from_ymd_opt(2023, 2, 15).unwrap()
        );

        // 常规跨年：12月 -> 次年1月
        let date = NaiveDate::from_ymd_opt(2023, 12, 10).unwrap();
        assert_eq!(
            add_months_safe(date, 1),
            NaiveDate::from_ymd_opt(2024, 1, 10).unwrap()
        );
    }

    #[test]
    fn test_add_months_end_of_month() {
        // 1月31日 +1个月 -> 2月28日（非闰年）
        let date = NaiveDate::from_ymd_opt(2023, 1, 31).unwrap();
        assert_eq!(
            add_months_safe(date, 1),
            NaiveDate::from_ymd_opt(2023, 2, 28).unwrap()
        );

        // 1月31日 +1个月 -> 2月29日（闰年）
        let date = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
        assert_eq!(
            add_months_safe(date, 1),
            NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()
        );

        // 8月31日 +1个月 -> 9月30日
        let date = NaiveDate::from_ymd_opt(2023, 8, 31).unwrap();
        assert_eq!(
            add_months_safe(date, 1),
            NaiveDate::from_ymd_opt(2023, 9, 30).unwrap()
        );
    }

    #[test]
    fn test_add_multiple_months() {
        // 跨多个月份（含月末调整）
        let date = NaiveDate::from_ymd_opt(2023, 1, 31).unwrap();
        assert_eq!(
            add_months_safe(date, 3),
            NaiveDate::from_ymd_opt(2023, 4, 30).unwrap()
        );

        // 跨年多个月份
        let date = NaiveDate::from_ymd_opt(2023, 11, 30).unwrap();
        assert_eq!(
            add_months_safe(date, 4),
            NaiveDate::from_ymd_opt(2024, 3, 30).unwrap()
        );
    }

    #[test]
    fn test_edge_cases() {
        // 从2月28日加12个月（非闰年 -> 闰年）
        let date = NaiveDate::from_ymd_opt(2023, 2, 28).unwrap();
        assert_eq!(
            add_months_safe(date, 12),
            NaiveDate::from_ymd_opt(2024, 2, 28).unwrap()
        );

        // 从2月29日加12个月（闰年 -> 非闰年）
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(
            add_months_safe(date, 12),
            NaiveDate::from_ymd_opt(2025, 2, 28).unwrap()
        );
    }

    #[test]
    fn test_add_more_than_12_months() {
        // 1. 简单跨年（12个月整）
        let date = NaiveDate::from_ymd_opt(2023, 3, 15).unwrap();
        assert_eq!(
            add_months_safe(date, 12),
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "12个月应精确跨年"
        );

        // 2. 超过12个月（13个月）
        let date = NaiveDate::from_ymd_opt(2023, 5, 20).unwrap();
        assert_eq!(
            add_months_safe(date, 13),
            NaiveDate::from_ymd_opt(2024, 6, 20).unwrap(),
            "13个月应跨年+1月"
        );

        // 3. 多轮跨年（24个月）
        let date = NaiveDate::from_ymd_opt(2023, 8, 10).unwrap();
        assert_eq!(
            add_months_safe(date, 24),
            NaiveDate::from_ymd_opt(2025, 8, 10).unwrap(),
            "24个月应跨两年"
        );

        // 4. 月末日期跨多年（从闰年2月29日开始）
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(
            add_months_safe(date, 13), // 2024-02-29 + 13个月 = 2025-03-29
            NaiveDate::from_ymd_opt(2025, 3, 29).unwrap(),
            "闰年2月29日加超过12个月应保持日期一致性"
        );

        // 5. 极端大月份数（验证计算无溢出）
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        assert_eq!(
            add_months_safe(date, 1000), // 2023-01-01 + 1000个月 ≈ 2106-05-01
            NaiveDate::from_ymd_opt(2106, 5, 1).unwrap(),
            "超大月份数应正确计算"
        );
    }

    #[test]
    fn test_negative_months() {
        // 向前减月份
        let date = NaiveDate::from_ymd_opt(2023, 5, 15).unwrap();
        assert_eq!(
            add_months_safe(date, -3),
            NaiveDate::from_ymd_opt(2023, 2, 15).unwrap()
        );

        // 跨年减
        let date = NaiveDate::from_ymd_opt(2023, 1, 31).unwrap();
        assert_eq!(
            add_months_safe(date, -1),
            NaiveDate::from_ymd_opt(2022, 12, 31).unwrap()
        );

        // 月末处理
        let date = NaiveDate::from_ymd_opt(2023, 3, 31).unwrap();
        assert_eq!(
            add_months_safe(date, -1),
            NaiveDate::from_ymd_opt(2023, 2, 28).unwrap()
        );
    }

    #[test]
    fn test_large_negative_months() {
        // 大负数减
        let date = NaiveDate::from_ymd_opt(2023, 5, 15).unwrap();
        assert_eq!(
            add_months_safe(date, -24),
            NaiveDate::from_ymd_opt(2021, 5, 15).unwrap()
        );
    }
}
