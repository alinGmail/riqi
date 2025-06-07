pub fn get_month_til_i18n(year: i32, month: u32, lang: &str) -> String {
    let month_names = match lang {
        "en" => [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ],
        "fr" => [
            "Janvier",
            "Février",
            "Mars",
            "Avril",
            "Mai",
            "Juin",
            "Juillet",
            "Août",
            "Septembre",
            "Octobre",
            "Novembre",
            "Décembre",
        ],
        "de" => [
            "Januar",
            "Februar",
            "März",
            "April",
            "Mai",
            "Juni",
            "Juli",
            "August",
            "September",
            "Oktober",
            "November",
            "Dezember",
        ],
        "ru" => [
            "Январь",
            "Февраль",
            "Март",
            "Апрель",
            "Май",
            "Июнь",
            "Июль",
            "Август",
            "Сентябрь",
            "Октябрь",
            "Ноябрь",
            "Декабрь",
        ],
        "ja" => return format!("{}年{}月", year, month), // 日语也用“年x月”
        "ko" => return format!("{}년 {}월", year, month), // 韩语语法
        "zh" | "zh-TW" | "zh-HK" => return format!("{}年{}月", year, month),
        _ => return format!("{}-{}", year, month),
    };
    format!("{} {}", month_names[month as usize - 1], year)
}

pub fn weekday_name_i18n(weekday: u32, lang: &str) -> String {
    let names = match lang {
        "en" => [
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
        ],
        "fr" => [
            "Dimanche", "Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Samedi",
        ],
        "de" => [
            "Sonntag",
            "Montag",
            "Dienstag",
            "Mittwoch",
            "Donnerstag",
            "Freitag",
            "Samstag",
        ],
        "ru" => [
            "Воскресенье",
            "Понедельник",
            "Вторник",
            "Среда",
            "Четверг",
            "Пятница",
            "Суббота",
        ],
        "ja" => [
            "日曜日",
            "月曜日",
            "火曜日",
            "水曜日",
            "木曜日",
            "金曜日",
            "土曜日",
        ],
        "ko" => [
            "일요일",
            "월요일",
            "화요일",
            "수요일",
            "목요일",
            "금요일",
            "토요일",
        ],
        "zh" | "zh-TW" | "zh-HK" => [
            "星期日",
            "星期一",
            "星期二",
            "星期三",
            "星期四",
            "星期五",
            "星期六",
        ],
        _ => ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"], // fallback
    };

    names.get(weekday as usize % 7).unwrap_or(&"").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english() {
        assert_eq!(get_month_til_i18n(2025, 6, "en"), "June 2025");
    }

    #[test]
    fn test_french() {
        assert_eq!(get_month_til_i18n(2025, 6, "fr"), "Juin 2025");
    }

    #[test]
    fn test_german() {
        assert_eq!(get_month_til_i18n(2025, 6, "de"), "Juni 2025");
    }

    #[test]
    fn test_russian() {
        assert_eq!(get_month_til_i18n(2025, 6, "ru"), "Июнь 2025");
    }

    #[test]
    fn test_japanese() {
        assert_eq!(get_month_til_i18n(2025, 6, "ja"), "2025年6月");
    }

    #[test]
    fn test_korean() {
        assert_eq!(get_month_til_i18n(2025, 6, "ko"), "2025년 6월");
    }

    #[test]
    fn test_chinese() {
        assert_eq!(get_month_til_i18n(2025, 6, "zh"), "2025年6月");
    }

    #[test]
    fn test_unknown_language() {
        assert_eq!(get_month_til_i18n(2025, 6, "xx"), "2025-6");
    }

    #[test]
    fn test_month_bounds() {
        // 测试1月和12月边界
        assert_eq!(get_month_til_i18n(2025, 1, "en"), "January 2025");
        assert_eq!(get_month_til_i18n(2025, 12, "fr"), "Décembre 2025");
    }

    #[test]
    fn test_weekdays() {
        assert_eq!(weekday_name_i18n(0, "en"), "Sunday");
        assert_eq!(weekday_name_i18n(1, "zh"), "星期一");
        assert_eq!(weekday_name_i18n(2, "zh-TW"), "星期二");
        assert_eq!(weekday_name_i18n(3, "ko"), "수요일");
        assert_eq!(weekday_name_i18n(4, "ru"), "Четверг");
        assert_eq!(weekday_name_i18n(5, "de"), "Freitag");
        assert_eq!(weekday_name_i18n(6, "ja"), "土曜日");
    }
}
