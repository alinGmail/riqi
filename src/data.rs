use chrono::{Datelike, NaiveDate};

// 表示日历中的一天
#[derive(Debug, Clone)]
pub struct CalendarDay {
    pub year: u32,
    pub month: u32, // 1-12
    pub day: u32,
    pub is_current_month: bool, // 是否属于当前月份
}

impl CalendarDay {
    pub fn new(year: u32, month: u32, day: u32, is_current_month: bool) -> Self {
        CalendarDay {
            year,
            month,
            day,
            is_current_month,
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
                false,
            ));
        }

        // 添加当前月的日期
        for day in 1..=last_day.day() {
            current_week.push(CalendarDay::new(year, month, day, true));

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
                current_week.push(CalendarDay::new(
                    next_month_first_day.year() as u32,
                    next_month_first_day.month(),
                    next_day,
                    false,
                ));
                next_day += 1;
            }

            weeks.push(current_week);
        }

        weeks
    }
}
