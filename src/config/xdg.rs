use std::fs;
use std::path::PathBuf;

pub struct Xdg;

impl Xdg {
    /// 获取配置文件目录
    /// - Linux:   $XDG_CONFIG_HOME/riqi  或  ~/.config/riqi
    /// - macOS:   ~/Library/Application Support/riqi
    /// - Windows: C:\Users\Alice\AppData\Roaming\riqi
    pub fn config_dir() -> Option<PathBuf> {
        let path = dirs::config_dir()?
            .join("riqi");
        if !path.exists(){
            // 自动创建目录（可选，建议在获取时或写入前执行）
            let _ = fs::create_dir_all(&path);
        }
        Some(path)
    }

    /// 获取数据存储目录 (用于存放数据库、大文件等)
    /// - Linux:   $XDG_DATA_HOME/riqi  或  ~/.local/share/riqi
    /// - macOS:  ~/Library/Application Support/riqi
    /// - Windows: C:\Users\Alice\AppData\Roaming\riqi
    pub fn data_dir() -> Option<PathBuf> {
        dirs::data_dir().map(|path| path.join("riqi"))
    }

    /// 获取缓存目录
    /// - Linux:   $XDG_CACHE_HOME/riqi  或  ~/.cache/riqi
    /// - macOS:   ~/Library/Caches/riqi
    /// - Windows: C:\Users\Alice\AppData\Local\riqi (注：Windows缓存通常存放在Local而非Roaming)
    pub fn cache_dir() -> Option<PathBuf> {
        let path = dirs::cache_dir()?.join("riqi");
        Some(path)
    }

}
