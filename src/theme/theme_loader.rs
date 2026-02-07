
use include_dir::{include_dir, Dir};

use super::theme_model::Theme;

static THEMES: Dir = include_dir!("$CARGO_MANIFEST_DIR/resources/theme");

pub fn load_theme_from_file(name: &str) -> Result<Theme, Box<dyn std::error::Error>> {
    let filename = if name.ends_with(".toml") {
        name.to_string()
    } else {
        format!("{}.toml", name)
    };
    
    let file = THEMES
        .get_file(&filename)
        .ok_or_else(|| format!("Theme file '{}' not found", filename))?;
    
    let content = file
        .contents_utf8()
        .ok_or_else(|| format!("Invalid UTF-8 in theme file '{}'", filename))?;
    
    let theme: Theme = toml::from_str(content)?;
    Ok(theme)
} 
