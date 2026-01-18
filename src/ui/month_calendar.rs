use chrono::{Datelike, NaiveDate};

use super::day_cell::DayCellState;

#[derive(Debug)]
pub struct MonthCalendar {
    pub year: u32,
    pub month: u32,
    pub day_cell_state: Vec<Vec<DayCellState>>,
}

impl MonthCalendar {
    pub fn new(year: u32, month: u32) -> Self {
        todo!()
    }

    fn generate_calendar_data(year: u32, month: u32) -> Vec<Vec<DayCellState>> {
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
            current_week.push(DayCellState::new(
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
            current_week.push(DayCellState::new(year, month, day, day_of_week, true));
            // 如果当前星期已满7天或这是最后一天，则开始新的一周
            if current_week.len() == 7 {
                weeks.push(current_week);
                current_week = Vec::new();
            }
        }

        let next_month_first_day = last_day.succ_opt().unwrap();
        let mut next_day = 1;
        // 添加下个月的日期
        while weeks.len() < 6 {
            while current_week.len() < 7 {
                let day_of_week = (current_week.len() as u32) % 7;
                current_week.push(DayCellState::new(
                    next_month_first_day.year() as u32,
                    next_month_first_day.month(),
                    next_day,
                    day_of_week,
                    false,
                ));
                next_day += 1;
            }

            weeks.push(current_week);
            current_week = Vec::new();
        }

        weeks
    }
}
