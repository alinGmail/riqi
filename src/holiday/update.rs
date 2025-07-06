use super::types::HolidayMeta;
use super::utils::load_cached_meta_file;
use crate::holiday::{types::MetaCache, utils::get_meta_cache_path};
use chrono::{TimeDelta, Utc};
use color_eyre::eyre::{eyre, Result};
use reqwest::Client;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Duration;

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
        println!("Meta cache is still valid.");
        let mut meta_updated = flag.lock().unwrap();
        *meta_updated = true;
        return;
    }

    if let Err(e) = download_and_cache_meta().await {
        eprintln!("Failed to update meta: {}", e);
    }

    let mut meta_updated = flag.lock().unwrap();
    *meta_updated = true;
    println!("Meta update finished.");
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

pub async fn update_holiday(flag: UpdateFlag) {
    {
        let meta_updated = flag.lock().unwrap();
        if !*meta_updated {
            println!("Holiday update cannot start: meta update is not complete. Exiting.");
            return;
        }
    }

    println!("Starting holiday update...");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Holiday update finished.");
}
