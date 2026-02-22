use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    Github,
    Gitee,
}

impl Source {
    pub fn as_str(&self) -> &'static str {
        match self {
            Source::Github => "github",
            Source::Gitee => "gitee",
        }
    }
}

impl std::str::FromStr for Source {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "github" => Ok(Source::Github),
            "gitee" => Ok(Source::Gitee),
            _ => Err(format!("Unknown source: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub country: String,
    pub language: String,
    pub column: Option<u32>,
    pub row: Option<u32>,
    pub show_lunar: bool,
    pub show_holiday: bool,
    pub output: String,
    pub source: Source,
    pub hide_bg: bool,
    pub theme: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigFile {
    pub language: Option<String>,
    pub country: Option<String>,
    pub show_lunar: Option<bool>,
    pub show_holiday: Option<bool>,
    pub hide_bg: Option<bool>,
    pub column: Option<u32>,
    pub row: Option<u32>,
    pub output: Option<String>,
    pub source: Option<String>,
    pub theme: Option<String>,
}

