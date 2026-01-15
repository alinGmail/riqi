use std::{fs, path::Path};

use super::{model::ConfigFile, xdg::Xdg};

pub fn load_file_config() -> Option<ConfigFile> {
    let config_dir = Xdg::config_dir();
    let riqi_file_str = config_dir.join("riqi.toml");
    let config_file_path = Path::new(&riqi_file_str);
    if !config_file_path.exists() || !config_file_path.is_file() {
        return None;
    }
    let content = fs::read_to_string(config_file_path).ok()?;
    toml::from_str(&content).ok()?
}
