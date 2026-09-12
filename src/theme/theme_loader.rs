use include_dir::{include_dir, Dir};

use super::theme_model::Theme;

static THEMES: Dir = include_dir!("$CARGO_MANIFEST_DIR/resources/theme");

pub const DEFAULT_THEME: &str = "ningmen";

pub fn available_theme_names() -> Vec<&'static str> {
    THEMES
        .files()
        .filter_map(|file| file.path().file_stem().and_then(|stem| stem.to_str()))
        .collect()
}

pub fn load_theme_with_fallback(name: &str) -> (Theme, Option<String>) {
    match load_theme_from_file(name) {
        Ok(theme) => (theme, None),
        Err(_) => {
            let fallback =
                load_theme_from_file(DEFAULT_THEME).expect("embedded default theme must load");
            let warning = format!(
                "Unknown theme '{}'. Using '{}'. Available: {}",
                name,
                DEFAULT_THEME,
                available_theme_names().join(", ")
            );
            (fallback, Some(warning))
        }
    }
}

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
