
use std::fs;
use std::path::Path;

use super::theme_model::Theme;

pub fn load_theme_from_file<P: AsRef<Path>>(path: P) -> Result<Theme, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let theme: Theme = toml::from_str(&content)?;
    Ok(theme)
} 
