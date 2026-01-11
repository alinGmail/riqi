use crate::events::AppEvent;
use lazy_static::lazy_static;
use std::collections::HashSet;
use tokio::sync::{mpsc, Mutex};

use super::downloader::DOWNLOAD_MANAGER;
use super::utils::{get_holiday_cache_file_path, get_meta_cache_path};

lazy_static! {
    static ref EXECUTED_TASKS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

pub fn get_holiday_data_file_url(year: &str, language: &str, country: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/alinGmail/riqi/refs/heads/main/resources/holidays/{}/{}_{}.json",
        year,
        language,
        country
    )
}

pub fn get_holiday_data_meta_file_url() -> String {
    let url = "https://raw.githubusercontent.com/alinGmail/riqi/refs/heads/main/resources/holidays/meta.json"; // Replace with your actual URL
    url.to_string()
}

pub async fn update_meta(tx: mpsc::Sender<AppEvent>) {
    {
        let mut executed_tasks = EXECUTED_TASKS.lock().await;
        let executed = executed_tasks.contains("update_meta");
        if executed {
            return;
        }
        executed_tasks.insert(String::from("update_meta"));
    }

    let url = get_holiday_data_meta_file_url();
    let meta_file_path = get_meta_cache_path();
    let file_path = match meta_file_path {
        Some(path) => path,
        _ => return,
    };
    let download_res = DOWNLOAD_MANAGER.download_file(&url, &file_path).await;
    match download_res {
        Ok(_) => {
            if let Err(e) = tx.send(AppEvent::RequestResult("update_meta".to_string())).await {
                log::error!("Failed to send update_meta result: {}", e);
            }
        }
        Err(err) => {
            log::error!("download meta file err {}", err);
        }
    }
}

pub async fn update_holiday_data(
    year: &str,
    language: &str,
    country: &str,
    tx: mpsc::Sender<AppEvent>,
) {
    log::debug!("update_holiday_data");
    {
        let mut executed_tasks = EXECUTED_TASKS.lock().await;
        let executed = executed_tasks.contains("update_meta");
        if executed {
            return;
        }
        executed_tasks.insert(String::from("update_meta"));
    }
    let url = get_holiday_data_file_url(year, language, country);
    let holiday_data_file_path = get_holiday_cache_file_path(year, language, country);

    let file_path = match holiday_data_file_path {
        Some(path) => path,
        _ => return,
    };
    let download_res = DOWNLOAD_MANAGER.download_file(&url, &file_path).await;
    match download_res {
        Ok(_) => {
            if let Err(e) = tx
                .send(AppEvent::RequestResult(
                    "update_holiday_data".to_string(),
                ))
                .await
            {
                log::error!("Failed to send update_holiday_data result: {}", e);
            }
        }
        Err(err) => {
            log::error!("download meta file err {}", err);
        }
    }
}