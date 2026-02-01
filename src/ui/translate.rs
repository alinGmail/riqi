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
        "ja" => return format!("{}年{}月", year, month), // 日语也用"年x月"
        "ko" => return format!("{}년 {}월", year, month), // 韩语语法
        "zh" | "zh-TW" | "zh-HK" => return format!("{}年{}月", year, month),
        _ => return format!("{}-{}", year, month),
    };
    format!("{} {}", month_names[month as usize - 1], year)
}

pub fn weekday_name_i18n(weekday: u32, lang: &str) -> String {
    let names = match lang {
        "en" => ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
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
        "ja" => ["日", "月", "火", "水", "木", "金", "土"],
        "ko" => [
            "일요일",
            "월요일",
            "화요일",
            "수요일",
            "목요일",
            "금요일",
            "토요일",
        ],
        "zh" | "zh-TW" | "zh-HK" => ["日", "一", "二", "三", "四", "五", "六"],
        _ => ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"], // fallback
    };

    names.get(weekday as usize % 7).unwrap_or(&"").to_string()
}