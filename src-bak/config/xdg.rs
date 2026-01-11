use std::{env, path::PathBuf};

pub struct Xdg;

impl Xdg {
    pub fn config_dir() -> PathBuf {
        #[cfg(windows)]
        {
            dirs::config_dir()
                .map(|p| p.join("riqi").join("config"))
                .expect("Failed to get config directory")
        }
        #[cfg(unix)]
        {
            dirs::home_dir()
                .map(|h| h.join(".config"))
                .map(|p| p.join("riqi"))
                .expect("Failed to get config directory")
        }
    }

    #[inline]
    pub fn cache_dir() -> PathBuf {
        #[cfg(unix)]
        let s = "yazi";

        #[cfg(windows)]
        let s = "yazi";

        env::temp_dir().join(s)
    }
}
