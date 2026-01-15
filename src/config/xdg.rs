use std::path::PathBuf;
use std::fs;

pub struct Xdg;

impl Xdg {
    /// 获取配置文件目录
    /// - Linux:   $XDG_CONFIG_HOME/riqi  或  ~/.config/riqi
    /// - macOS:   ~/Library/Application Support/riqi
    /// - Windows: C:\Users\Alice\AppData\Roaming\riqi
    pub fn config_dir() -> PathBuf {
        let path = dirs::config_dir()
            .expect("无法获取系统配置目录")
            .join("riqi");
        
        // 自动创建目录（可选，建议在获取时或写入前执行）
        let _ = fs::create_dir_all(&path);
        path
    }

    /// 获取数据存储目录 (用于存放数据库、大文件等)
    /// - Linux:   $XDG_DATA_HOME/riqi  或  ~/.local/share/riqi
    /// - macOS:   ~/Library/Application Support/riqi
    /// - Windows: C:\Users\Alice\AppData\Roaming\riqi
    pub fn data_dir() -> PathBuf {
        dirs::data_dir()
            .expect("无法获取系统数据目录")
            .join("riqi")
    }

    /// 获取缓存目录
    /// - Linux:   $XDG_CACHE_HOME/riqi  或  ~/.cache/riqi
    /// - macOS:   ~/Library/Caches/riqi
    /// - Windows: C:\Users\Alice\AppData\Local\riqi (注：Windows缓存通常存放在Local而非Roaming)
    pub fn cache_dir() -> PathBuf {
        dirs::cache_dir()
            .unwrap_or_else(|| std::env::temp_dir()) // 如果找不到标准缓存路径，则回退到临时目录
            .join("riqi")
    }

    /// 获取日志/状态目录 (2026年针对 Unix 的新规范)
    /// - Linux:   $XDG_STATE_HOME/riqi 或 ~/.local/state/riqi
    /// - 其他:    自动回退到 data_dir
    pub fn state_dir() -> PathBuf {
        #[cfg(unix)]
        {
            // dirs v5+ 支持 state_dir
            dirs::state_dir()
                .unwrap_or_else(|| Self::data_dir())
                .join("riqi")
        }
        #[cfg(not(unix))]
        {
            Self::data_dir()
        }
    }
}

