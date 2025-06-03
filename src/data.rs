use chrono::{Datelike, NaiveDate};
use tyme4rs::tyme::solar::SolarDay;

// 表示日历中的一天
#[derive(Debug, Clone)]
pub struct CalendarDay {
    pub year: u32,
    pub month: u32, // 1-12
    pub day: u32,
    pub day_of_week: u32,       // 0=Sunday, 6=Saturday
    pub is_current_month: bool, // 是否属于当前月份
    pub lunar_month: u32,       // 农历月份
    pub lunar_day: u32,         // 农历日期
}

impl CalendarDay {
    pub fn new(year: u32, month: u32, day: u32, day_of_week: u32, is_current_month: bool) -> Self {
        let solar = SolarDay::from_ymd(year as isize, month as usize, day as usize);
        let lunar_month = solar.get_lunar_day().get_month() as u32;
        let lunar_day = solar.get_lunar_day().get_day() as u32;

        CalendarDay {
            year,
            month,
            day,
            day_of_week,
            is_current_month,
            lunar_month,
            lunar_day,
        }
    }
}

#[derive(Debug)]
pub struct MonthCalendar {
    pub year: u32,
    pub month: u32,
    pub day_data: Vec<Vec<CalendarDay>>,
}

impl MonthCalendar {
    pub fn new(year: u32, month: u32) -> Self {
        let day_data = Self::generate_calendar_data(year, month);
        MonthCalendar {
            year,
            month,
            day_data,
        }
    }

    fn generate_calendar_data(year: u32, month: u32) -> Vec<Vec<CalendarDay>> {
        let first_day = NaiveDate::from_ymd_opt(year as i32, month, 1).unwrap();
        let last_day = if month == 12 {
            NaiveDate::from_ymd_opt(year as i32 + 1, 1, 1)
                .unwrap()
                .pred_opt()
                .unwrap()
        } else {
            NaiveDate::from_ymd_opt(year as i32, month + 1, 1)
                .unwrap()
                .pred_opt()
                .unwrap()
        };

        // 获取第一天是星期几（0=Sunday, 6=Saturday）
        let first_weekday = first_day.weekday().num_days_from_sunday() as usize;

        // 获取上个月的最后一天
        let prev_month_last_day = first_day.pred_opt().unwrap();

        // 初始化日历数据
        let mut weeks = Vec::new();
        let mut current_week = Vec::new();

        // 添加上个月的日期
        for i in (0..first_weekday).rev() {
            let day = prev_month_last_day.day() - i as u32;
            current_week.push(CalendarDay::new(
                prev_month_last_day.year() as u32,
                prev_month_last_day.month(),
                day,
                first_weekday as u32 - 1 - i as u32,
                false,
            ));
        }

        // 添加当前月的日期
        for day in 1..=last_day.day() {
            let day_of_week = (first_weekday as u32 + day - 1) % 7;
            current_week.push(CalendarDay::new(year, month, day, day_of_week, true));

            // 如果当前星期已满7天或这是最后一天，则开始新的一周
            if current_week.len() == 7 {
                weeks.push(current_week);
                current_week = Vec::new();
            }
        }

        // 添加下个月的日期
        if !current_week.is_empty() {
            let next_month_first_day = last_day.succ_opt().unwrap();
            let mut next_day = 1;

            while current_week.len() < 7 {
                let day_of_week = (current_week.len() as u32) % 7;
                current_week.push(CalendarDay::new(
                    next_month_first_day.year() as u32,
                    next_month_first_day.month(),
                    next_day,
                    day_of_week,
                    false,
                ));
                next_day += 1;
            }

            weeks.push(current_week);
        }

        weeks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_day_creation() {
        let day = CalendarDay::new(2024, 3, 15, 5, true);
        assert_eq!(day.year, 2024);
        assert_eq!(day.month, 3);
        assert_eq!(day.day, 15);
        assert_eq!(day.day_of_week, 5); // 周五
        assert!(day.is_current_month);
    }

    #[test]
    fn test_month_calendar_creation() {
        let calendar = MonthCalendar::new(2024, 3);
        assert_eq!(calendar.year, 2024);
        assert_eq!(calendar.month, 3);
        assert!(!calendar.day_data.is_empty());
    }

    #[test]
    fn test_month_calendar_weeks() {
        let calendar = MonthCalendar::new(2024, 3);
        // 2024年3月有6周
        assert_eq!(calendar.day_data.len(), 6);

        // 检查第一周
        let first_week = &calendar.day_data[0];
        assert_eq!(first_week.len(), 7);

        // 检查最后一周
        let last_week = &calendar.day_data[5];
        assert_eq!(last_week.len(), 7);
    }

    #[test]
    fn test_month_calendar_days() {
        let calendar = MonthCalendar::new(2024, 3);

        // 检查3月1日
        let first_week = &calendar.day_data[0];
        let march_first = first_week
            .iter()
            .find(|day| day.day == 1 && day.is_current_month)
            .unwrap();
        assert_eq!(march_first.year, 2024);
        assert_eq!(march_first.month, 3);
        assert_eq!(march_first.day_of_week, 5); // 3月1日是周五

        // 检查3月31日
        let last_week = &calendar.day_data[5];
        let march_last = last_week
            .iter()
            .find(|day| day.day == 31 && day.is_current_month)
            .unwrap();
        assert_eq!(march_last.year, 2024);
        assert_eq!(march_last.month, 3);
        assert_eq!(march_last.day_of_week, 0); // 3月31日是周日
    }

    #[test]
    fn test_month_calendar_weekdays() {
        let calendar = MonthCalendar::new(2024, 3);

        // 检查所有当前月份的日期
        for week in &calendar.day_data {
            for day in week {
                if day.is_current_month {
                    // 验证星期几的计算是否正确
                    let date =
                        NaiveDate::from_ymd_opt(day.year as i32, day.month, day.day).unwrap();
                    let expected_weekday = date.weekday().num_days_from_sunday() as u32;
                    assert_eq!(
                        day.day_of_week, expected_weekday,
                        "日期 {}-{}-{} 的星期几计算错误，期望 {}，实际 {}",
                        day.year, day.month, day.day, expected_weekday, day.day_of_week
                    );
                }
            }
        }
    }

    #[test]
    fn test_month_calendar_adjacent_months() {
        let calendar = MonthCalendar::new(2024, 3);

        // 检查2月的最后几天
        let first_week = &calendar.day_data[0];
        let feb_days: Vec<&CalendarDay> = first_week
            .iter()
            .filter(|day| !day.is_current_month && day.month == 2)
            .collect();
        assert!(!feb_days.is_empty());

        // 检查4月的开始几天
        let last_week = &calendar.day_data[5];
        let apr_days: Vec<&CalendarDay> = last_week
            .iter()
            .filter(|day| !day.is_current_month && day.month == 4)
            .collect();
        assert!(!apr_days.is_empty());
    }

    #[test]
    fn test_april_30_in_may_calendar() {
        // 测试2025年5月份日历中的4月30日
        let calendar = MonthCalendar::new(2025, 5);

        print!("{:?}", calendar);
        // 找到4月30日
        let april_30 = calendar
            .day_data
            .iter()
            .flat_map(|week| week.iter())
            .find(|day| day.year == 2025 && day.month == 4 && day.day == 30)
            .unwrap();

        // 使用 chrono 验证正确的星期几
        let date = NaiveDate::from_ymd_opt(2025, 4, 30).unwrap();
        let expected_weekday = date.weekday().num_days_from_sunday() as u32;

        assert_eq!(
            april_30.day_of_week, expected_weekday,
            "2025年4月30日的星期几计算错误，期望 {}，实际 {}",
            expected_weekday, april_30.day_of_week
        );

        // 验证其他属性
        assert!(!april_30.is_current_month, "4月30日不应该被标记为当前月份");
        assert_eq!(april_30.year, 2025, "年份应该是2025");
        assert_eq!(april_30.month, 4, "月份应该是4");
        assert_eq!(april_30.day, 30, "日期应该是30");
    }
}
