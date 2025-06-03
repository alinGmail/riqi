pub fn number_to_lunar_day(day: u32) -> String {
    if day < 1 || day > 30 {
        return "无效日期".to_string();
    }

    match day {
        1 => "初一".to_string(),
        2 => "初二".to_string(),
        3 => "初三".to_string(),
        4 => "初四".to_string(),
        5 => "初五".to_string(),
        6 => "初六".to_string(),
        7 => "初七".to_string(),
        8 => "初八".to_string(),
        9 => "初九".to_string(),
        10 => "初十".to_string(),
        11 => "十一".to_string(),
        12 => "十二".to_string(),
        13 => "十三".to_string(),
        14 => "十四".to_string(),
        15 => "十五".to_string(),
        16 => "十六".to_string(),
        17 => "十七".to_string(),
        18 => "十八".to_string(),
        19 => "十九".to_string(),
        20 => "二十".to_string(),
        21 => "廿一".to_string(),
        22 => "廿二".to_string(),
        23 => "廿三".to_string(),
        24 => "廿四".to_string(),
        25 => "廿五".to_string(),
        26 => "廿六".to_string(),
        27 => "廿七".to_string(),
        28 => "廿八".to_string(),
        29 => "廿九".to_string(),
        30 => "三十".to_string(),
        _ => "无效日期".to_string(),
    }
}

pub fn number_to_lunar_month(month: u32) -> String {
    if month < 1 || month > 12 {
        return "无效月份".to_string();
    }

    match month {
        1 => "正月".to_string(),
        2 => "二月".to_string(),
        3 => "三月".to_string(),
        4 => "四月".to_string(),
        5 => "五月".to_string(),
        6 => "六月".to_string(),
        7 => "七月".to_string(),
        8 => "八月".to_string(),
        9 => "九月".to_string(),
        10 => "十月".to_string(),
        // 11 => "冬月".to_string(),
        11 => "十一月".to_string(),
        12 => "腊月".to_string(),
        _ => "无效月份".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_lunar_day() {
        assert_eq!(number_to_lunar_day(1), "初一");
        assert_eq!(number_to_lunar_day(10), "初十");
        assert_eq!(number_to_lunar_day(15), "十五");
        assert_eq!(number_to_lunar_day(20), "二十");
        assert_eq!(number_to_lunar_day(21), "廿一");
        assert_eq!(number_to_lunar_day(28), "廿八");
        assert_eq!(number_to_lunar_day(30), "三十");
        assert_eq!(number_to_lunar_day(31), "无效日期");
    }

    #[test]
    fn test_number_to_lunar_month() {
        assert_eq!(number_to_lunar_month(1), "正月");
        assert_eq!(number_to_lunar_month(2), "二月");
        assert_eq!(number_to_lunar_month(11), "冬月");
        assert_eq!(number_to_lunar_month(12), "腊月");
        assert_eq!(number_to_lunar_month(13), "无效月份");
    }
} 