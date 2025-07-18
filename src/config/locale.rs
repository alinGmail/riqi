/// 解析 IETF BCP 47 语言标签或常见 locale 字符串，
/// 返回 (language, Option<country>)，
/// 会自动去除编码部分（如 ".UTF-8"）
/// 支持分隔符 '-' 或 '_'
pub fn parse_language_country(locale: &str) -> (String, Option<String>) {
    // 先去掉编码后缀，比如 .UTF-8 或 .utf8
    let locale = locale.split('.').next().unwrap_or(locale);

    // 拆分语言标签，支持 '-' 和 '_'
    let parts: Vec<&str> = locale.split(['-', '_']).collect();

    if parts.is_empty() {
        return (String::new(), None);
    }

    let language = parts[0].to_lowercase();

    let mut country: Option<String> = None;

    if parts.len() >= 2 {
        let last = parts.last().unwrap();
        if (last.len() == 2 && last.chars().all(|c| c.is_ascii_alphabetic()))
            || (last.len() == 3 && last.chars().all(|c| c.is_ascii_digit()))
        {
            country = Some(last.to_uppercase());
        }
    }

    (language, country)
}

#[test]
fn test_parse_language_country() {
    assert_eq!(parse_language_country("zh"), ("zh".to_string(), None));
    assert_eq!(
        parse_language_country("zh-CN"),
        ("zh".to_string(), Some("CN".to_string()))
    );
    assert_eq!(parse_language_country("zh-Hans"), ("zh".to_string(), None));
    assert_eq!(
        parse_language_country("zh-Hans-CN"),
        ("zh".to_string(), Some("CN".to_string()))
    );
    assert_eq!(
        parse_language_country("en-US"),
        ("en".to_string(), Some("US".to_string()))
    );
    assert_eq!(parse_language_country("fr"), ("fr".to_string(), None));
    assert_eq!(
        parse_language_country("fr-FR"),
        ("fr".to_string(), Some("FR".to_string()))
    );
    assert_eq!(
        parse_language_country("zh_Hant_TW"),
        ("zh".to_string(), Some("TW".to_string()))
    );
    assert_eq!(
        parse_language_country("en_419"),
        ("en".to_string(), Some("419".to_string()))
    );
    assert_eq!(
        parse_language_country("en_US.UTF-8"),
        ("en".to_string(), Some("US".to_string()))
    );
    assert_eq!(
        parse_language_country("zh_CN.utf8"),
        ("zh".to_string(), Some("CN".to_string()))
    );
}
