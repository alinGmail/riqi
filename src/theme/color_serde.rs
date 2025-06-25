use ratatui::style::Color;
use serde::{self, Deserialize, Deserializer};

pub fn de_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_color(&s).map_err(serde::de::Error::custom)
}

pub fn de_opt_color<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => parse_color(&s).map(Some).map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

fn parse_color(s: &str) -> Result<Color, String> {
    if let Some(rgb) = s.strip_prefix("Rgb(").and_then(|s| s.strip_suffix(")")) {
        let parts: Vec<_> = rgb.split(',').map(|x| x.trim().parse::<u8>()).collect();
        if parts.len() == 3 && parts.iter().all(|x| x.is_ok()) {
            return Ok(Color::Rgb(
                parts[0].as_ref().unwrap().clone(),
                parts[1].as_ref().unwrap().clone(),
                parts[2].as_ref().unwrap().clone(),
            ));
        }
        return Err(format!("Rgb 格式错误: {}", s));
    }
    if let Some(hex) = s.strip_prefix('#') {
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "hex解析失败")?;
            let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "hex解析失败")?;
            let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "hex解析失败")?;
            return Ok(Color::Rgb(r, g, b));
        }
        return Err(format!("Hex 格式错误: {}", s));
    }
    Err(format!("未知颜色格式: {}", s))
} 