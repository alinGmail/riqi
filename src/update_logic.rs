use chrono::{DateTime, TimeDelta, Utc};
use color_eyre::eyre::{eyre, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::types::holiday_meta::HolidayMeta;

#[derive(Serialize, Deserialize, Debug)]
struct MetaCache {
    data: HolidayMeta,
    cache_time: DateTime<Utc>,
}

pub type UpdateFlag = Arc<Mutex<bool>>;

pub async fn update_meta(flag: UpdateFlag) {
    if let Ok(Some(_)) = get_cached_meta() {
        println!("Meta cache is still valid.");
        let mut meta_updated = flag.lock().unwrap();
        *meta_updated = true;
        return;
    }

    println!("Starting meta update...");
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
    let meta = crate::holiday_updater::download_meta_file(&client, url).await?;

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

fn get_cached_meta() -> Result<Option<HolidayMeta>> {
    let cache_path = get_meta_cache_path().ok_or_else(|| eyre!("Failed to get cache path"))?;
    if !cache_path.exists() {
        return Ok(None);
    }

    let json = fs::read_to_string(cache_path)?;
    let cache: MetaCache = serde_json::from_str(&json)?;

    if (Utc::now() - cache.cache_time) < TimeDelta::days(1) {
        Ok(Some(cache.data))
    } else {
        Ok(None)
    }
}

fn get_meta_cache_path() -> Option<PathBuf> {
    dirs::cache_dir().and_then(|mut path| {
        path.push("riqi");
        path.push("holidays");
        fs::create_dir_all(&path).ok()?;
        path.push("meta_cache.json");
        Some(path)
    })
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

