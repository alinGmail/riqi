use super::types::HolidayMeta;
use super::utils::{get_ylc_code, load_cached_meta_file};
use crate::holiday::{types::MetaCache, utils::get_meta_cache_path};
use chrono::{TimeDelta, Utc};
use color_eyre::eyre::{eyre, Result};
use lazy_static::lazy_static;
use reqwest::Client;
use std::collections::HashSet;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Duration;

lazy_static! {
    static ref DOWNLOADING_FILES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

pub async fn download_meta_file(client: &Client, url: &str) -> Result<HolidayMeta> {
    let response = client.get(url).send().await?;
    let meta = response.json::<HolidayMeta>().await?;
    Ok(meta)
}

pub type UpdateFlag = Arc<Mutex<bool>>;

pub fn get_cached_meta() -> Result<Option<HolidayMeta>> {
    match load_cached_meta_file() {
        Ok(Some(cache)) => {
            if (Utc::now() - cache.cache_time) < TimeDelta::days(1) {
                Ok(Some(cache.data))
            } else {
                log::debug!("Meta cache is outdated.");
                Ok(None)
            }
        }
        Ok(None) => Ok(None), // No cache file exists
        Err(e) => {
            log::warn!("Failed to load meta cache: {}", e);
            Err(e)
        }
    }
}

pub async fn update_meta(flag: UpdateFlag) {
    if let Ok(Some(_)) = get_cached_meta() {
        log::debug!("Meta cache is still valid.");
        let mut meta_updated = flag.lock().unwrap();
        *meta_updated = true;
        return;
    }

    if let Err(e) = download_and_cache_meta().await {
        log::error!("Failed to update meta: {}", e);
    }

    let mut meta_updated = flag.lock().unwrap();
    *meta_updated = true;
    log::debug!("Meta update finished.");
}

async fn download_and_cache_meta() -> Result<()> {
    let client = Client::new();
    let url = "https://raw.githubusercontent.com/alinGmail/riqi/refs/heads/main/resources/holidays/meta.json"; // Replace with your actual URL
    let meta = download_meta_file(&client, url).await?;

    let cache = MetaCache {
        data: meta,
        cache_time: Utc::now(),
    };

    let cache_path = get_meta_cache_path().ok_or_else(|| eyre!("Failed to get cache path"))?;
    let json = serde_json::to_string(&cache)?;
    log::debug!("cache meta.json in {:?}", cache_path);
    fs::write(cache_path, json)?;

    Ok(())
}

pub async fn update_holiday(year: &str, language: &str, country: &str) {
    let ylc_code = get_ylc_code(year, language, country);
    {
        let mut downloading = DOWNLOADING_FILES.lock().unwrap();
        if downloading.contains(&ylc_code) {
            log::debug!("Already downloading {}", ylc_code);
            return;
        }
        downloading.insert(ylc_code.clone());
    }

    // Simulate download
    log::debug!("Downloading {}", ylc_code);
    tokio::time::sleep(Duration::from_secs(2)).await;
    log::debug!("Downloaded {}", ylc_code);

    {
        let mut downloading = DOWNLOADING_FILES.lock().unwrap();
        downloading.remove(&ylc_code);
    }
}
