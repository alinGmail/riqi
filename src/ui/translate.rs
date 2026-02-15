use std::str::FromStr;

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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    ZH, // 中文
    JA, // 日语
    KO, // 韩语
    DE, // 德语
    EN, // 英语
    FR, // 法语
    RU, // 俄语
}

impl FromStr for Language {
    type Err = String;
    fn from_str(code: &str) -> Result<Self, Self::Err> {
        match code.to_lowercase().as_str() {
            "zh" => Ok(Language::ZH),
            "ja" => Ok(Language::JA),
            "ko" => Ok(Language::KO),
            "de" => Ok(Language::DE),
            "en" => Ok(Language::EN),
            "fr" => Ok(Language::FR),
            "ru" => Ok(Language::RU),
            _ => Err(format!("Unknown language code: {}", code)),
        }
    }
}

pub struct Translate<'a> {
    pub navigation: &'a str,
    pub next_month: &'a str,
    pub prev_month: &'a str,
    pub next_year: &'a str,
    pub prev_year: &'a str,
    pub back_to_today: &'a str,
}

const ZH_TRANSLATE: Translate<'static> = Translate {
    navigation: "导航",
    next_month: "下一月",
    prev_month: "上一月",
    next_year: "下一年",
    prev_year: "上一年",
    back_to_today: "返回今天",
};

// Japanese (日本語)
const JA_TRANSLATE: Translate<'static> = Translate {
    navigation: "ナビゲーション",
    next_month: "翌月",
    prev_month: "前月",
    next_year: "翌年",
    prev_year: "前年",
    back_to_today: "今日に戻る",
};

// Korean (한국어)
const KO_TRANSLATE: Translate<'static> = Translate {
    navigation: "탐색",
    next_month: "다음 달",
    prev_month: "이전 달",
    next_year: "다음 해",
    prev_year: "이전 해",
    back_to_today: "오늘로 돌아가기",
};

// German (Deutsch)
const DE_TRANSLATE: Translate<'static> = Translate {
    navigation: "Navigation",
    next_month: "Nächster Monat",
    prev_month: "Vorheriger Monat",
    next_year: "Nächstes Jahr",
    prev_year: "Vorheriges Jahr",
    back_to_today: "Zurück zum Heute",
};

// French (Français)
const FR_TRANSLATE: Translate<'static> = Translate {
    navigation: "Navigation",
    next_month: "Mois suivant",
    prev_month: "Mois précédent",
    next_year: "Année suivante",
    prev_year: "Année précédente",
    back_to_today: "Retour à aujourd'hui",
};

// Russian (Русский)
const RU_TRANSLATE: Translate<'static> = Translate {
    navigation: "Навигация",
    next_month: "Следующий месяц",
    prev_month: "Предыдущий месяц",
    next_year: "Следующий год",
    prev_year: "Предыдущий год",
    back_to_today: "Вернуться к сегодняшнему дню",
};

// English (English)
const EN_TRANSLATE: Translate<'static> = Translate {
    navigation: "Navigation",
    next_month: "Next month",
    prev_month: "Previous month",
    next_year: "Next year",
    prev_year: "Previous year",
    back_to_today: "Back to today",
};

pub fn get_translate(language: Language) -> &'static Translate<'static> {
    match language {
        Language::ZH => &ZH_TRANSLATE,
        Language::JA => &JA_TRANSLATE,
        Language::KO => &KO_TRANSLATE,
        Language::DE => &DE_TRANSLATE,
        Language::FR => &FR_TRANSLATE,
        Language::RU => &RU_TRANSLATE,
        Language::EN => &EN_TRANSLATE,
    }
}
