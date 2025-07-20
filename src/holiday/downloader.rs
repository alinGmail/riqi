use lazy_static::lazy_static;
use log::{error, info};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

// 全局下载管理器单例
lazy_static! {
    pub static ref DOWNLOAD_MANAGER: DownloadManager = DownloadManager::new();
}

/// 用于管理文件下载、缓存和锁的结构体
pub struct DownloadManager {
    // 缓存: 存储文件路径和其内容
    cache: Mutex<HashMap<PathBuf, Arc<Vec<u8>>>>,
    // 文件锁: 为每个文件路径创建一个独立的锁，防止并发访问
    file_locks: Mutex<HashMap<PathBuf, Arc<Mutex<()>>>>,
}

impl DownloadManager {
    /// 创建一个新的 DownloadManager 实例
    fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
            file_locks: Mutex::new(HashMap::new()),
        }
    }

    /// 根据路径获取或创建文件锁
    async fn get_file_lock(&self, path: &Path) -> Arc<Mutex<()>> {
        let mut locks = self.file_locks.lock().await;
        let lock = locks
            .entry(path.to_path_buf())
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone();
        lock
    }

    /// 下载文件并保存到指定路径
    ///
    /// # Arguments
    /// * `url` - 要下载的文件的 URL
    /// * `path` - 保存文件的本地路径
    ///
    /// # Returns
    /// * `Result<(), String>` - 成功则返回 Ok，失败则返回错误信息
    pub async fn download_file(&self, url: &str, path: &Path) -> Result<(), String> {
        let file_lock = self.get_file_lock(path).await;
        let _lock_guard = file_lock.lock().await; // 获取文件锁，在此作用域结束前不会释放

        info!("Starting download from {} to {}", url, path.display());

        // 1. 下载文件内容
        let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
        if !response.status().is_success() {
            let err_msg = format!("Failed to download file: HTTP status {}", response.status());
            error!("{}", err_msg);
            return Err(err_msg);
        }
        let content = response.bytes().await.map_err(|e| e.to_string())?;

        // 2. 确保目录存在
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        // 3. 写入文件
        let mut file = fs::File::create(path).await.map_err(|e| e.to_string())?;
        file.write_all(&content).await.map_err(|e| e.to_string())?;

        info!(
            "Successfully downloaded and saved file to {}",
            path.display()
        );

        // 4. 更新缓存
        let mut cache = self.cache.lock().await;
        cache.insert(path.to_path_buf(), Arc::new(content.to_vec()));
        info!("Updated cache for {}", path.display());

        Ok(())
    }

    /// 从缓存或磁盘读取文件
    ///
    /// # Arguments
    /// * `path` - 要读取的文件的路径
    ///
    /// # Returns
    /// * `Result<Arc<Vec<u8>>, String>` - 返回文件内容的 Arc 引用
    pub async fn read_file_with_cache(&self, path: &Path) -> Result<Arc<Vec<u8>>, String> {
        // 1. 检查缓存
        {
            let cache = self.cache.lock().await;
            if let Some(content) = cache.get(path) {
                info!("Cache hit for {}", path.display());
                return Ok(content.clone());
            }
        }

        // 2. 缓存未命中，加锁并从磁盘读取
        let file_lock = self.get_file_lock(path).await;
        let _lock_guard = file_lock.lock().await;

        info!("Cache miss for {}. Reading from disk.", path.display());

        // 再次检查缓存，防止在等待锁的过程中，其他线程已经填充了缓存
        {
            let cache = self.cache.lock().await;
            if let Some(content) = cache.get(path) {
                info!("Cache hit for {} after acquiring lock.", path.display());
                return Ok(content.clone());
            }
        }

        // 3. 从磁盘读取
        let content = fs::read(path).await.map_err(|e| e.to_string())?;
        let content_arc = Arc::new(content);

        // 4. 填充缓存
        {
            let mut cache = self.cache.lock().await;
            cache.insert(path.to_path_buf(), content_arc.clone());
            info!("Populated cache for {}", path.display());
        }

        Ok(content_arc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 注意: 这个测试会真实地进行网络请求和文件I/O
    #[tokio::test]
    async fn test_download_and_read() {
        let url = "https://raw.githubusercontent.com/alinGmail/riqi/refs/heads/main/resources/holidays/meta.json";
        let path = Path::new("meta_tmp.json");

        // 1. 下载文件
        let download_result = DOWNLOAD_MANAGER.download_file(url, path).await;
        assert!(download_result.is_ok());
        assert!(path.exists());

        // 2. 用缓存读
        let content1 = DOWNLOAD_MANAGER.read_file_with_cache(path).await.unwrap();
        assert!(!content1.is_empty());

        // 3. 再次读取，应该还是从缓存
        let content2 = DOWNLOAD_MANAGER.read_file_with_cache(path).await.unwrap();
        assert_eq!(content1, content2);

        // 4. 清理
        fs::remove_file(path).await.unwrap();

        // 5. 验证缓存是否被清理（在这个实现中我们没有清理缓存的逻辑，但可以验证读取失败）
        let read_after_delete = DOWNLOAD_MANAGER.read_file_with_cache(path).await;
        // 因为文件被删了，所以从磁盘读会失败
        assert!(read_after_delete.is_err());
    }
}
