use crate::config::model::Source;
use crate::events::AppEvent;
use crate::holiday::cache_manager::HolidayCacheManager;
use crate::holiday::modal::{parse_holidays_of_year, HolidayOfYearList};
use crate::holiday::utils::{get_holiday_data_file_url, get_lc_code, get_ylc_code, is_locale_supported};
use color_eyre::eyre::{bail, eyre, Report, Result};
use log::{error, info};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

/// 首次请求 + 2 次重试。
const MAX_ATTEMPTS: u32 = 3;
const REQUEST_TIMEOUT_SECS: u64 = 10;

#[derive(Clone, Copy, PartialEq, Eq)]
enum KeyState {
    Idle,
    Loading,
    Ready,
    Failed,
}

#[derive(Clone)]
pub struct HolidayUpdateManager {
    state: Arc<Mutex<HashMap<String, KeyState>>>,
    tx: Sender<AppEvent>,
}

struct RemoteHoliday {
    raw: String,
    data: HolidayOfYearList,
}

impl HolidayUpdateManager {
    pub fn new(tx: Sender<AppEvent>) -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
            tx,
        }
    }

    /// 确保某年的假期数据可用：缓存优先，过期则异步刷新。
    ///
    /// 策略：
    /// 1. 按 key 去重（Loading / Ready / Failed 直接返回）
    /// 2. 不支持的 locale 快速失败并通知
    /// 3. cache-first：命中即回传旧数据；新鲜则结束，过期则后台刷新
    /// 4. 远端刷新失败重试 2 次，仍失败则置 Failed 并通知
    pub async fn ensure_year(&self, year: &str, language: &str, country: &str, source: Source) {
        let key = get_ylc_code(year, language, country);

        // 1. 请求去重
        {
            let mut state = self.state.lock().await;
            match state.entry(key.clone()).or_insert(KeyState::Idle) {
                KeyState::Idle => {}
                _ => return,
            }
        }

        // 2. 不支持的 locale：不读缓存、不联网
        if !is_locale_supported(language, country) {
            self.set_state(&key, KeyState::Failed).await;
            let _ = self.tx.send(AppEvent::HolidayLoadFailed(
                key,
                format!("暂不支持该语言/地区: {}", get_lc_code(language, country)),
            ));
            return;
        }

        self.set_state(&key, KeyState::Loading).await;

        // 3. cache-first（同步 IO）
        let cached = HolidayCacheManager::load(year, language, country);
        let old_version = cached.as_ref().map(|c| c.data.version);
        if let Some(cached) = cached {
            // 先显示旧数据
            let _ = self
                .tx
                .send(AppEvent::UpdateHoliday(key.clone(), cached.data.clone()));
            if HolidayCacheManager::is_fresh(cached.modify_time) {
                self.set_state(&key, KeyState::Ready).await;
                return;
            }
            info!("holiday cache is stale, refresh in background");
        }

        // 4. 异步刷新（带重试）
        let tx = self.tx.clone();
        let state = self.state.clone();
        let year_owned = year.to_string();
        let language_owned = language.to_string();
        let country_owned = country.to_string();
        tokio::spawn(async move {
            let result =
                Self::fetch_with_retry(&year_owned, &language_owned, &country_owned, &source).await;
            match result {
                Ok(remote) => {
                    if old_version.map_or(true, |v| v < remote.data.version) {
                        if let Err(e) = HolidayCacheManager::save(
                            &year_owned,
                            &language_owned,
                            &country_owned,
                            remote.raw.as_bytes(),
                        ) {
                            error!("failed to save holiday cache: {}", e);
                        }
                    }
                    let _ = tx.send(AppEvent::UpdateHoliday(key.clone(), remote.data));
                    Self::set_state_shared(&state, &key, KeyState::Ready).await;
                }
                Err(e) => {
                    error!("failed to load remote holiday {}: {}", key, e);
                    Self::set_state_shared(&state, &key, KeyState::Failed).await;
                    let _ = tx.send(AppEvent::HolidayLoadFailed(
                        key,
                        "假期数据加载失败".to_string(),
                    ));
                }
            }
        });
    }

    async fn fetch_with_retry(
        year: &str,
        language: &str,
        country: &str,
        source: &Source,
    ) -> Result<RemoteHoliday> {
        let url = get_holiday_data_file_url(year, language, country, source);
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .build()?;
        let mut last_err: Option<Report> = None;
        for attempt in 1..=MAX_ATTEMPTS {
            info!("fetch holiday {} (attempt {}/{})", url, attempt, MAX_ATTEMPTS);
            match Self::fetch_once(&client, &url).await {
                Ok(remote) => return Ok(remote),
                Err(e) => {
                    error!("fetch attempt {} failed: {}", attempt, e);
                    last_err = Some(e);
                }
            }
        }
        Err(last_err.unwrap_or_else(|| eyre!("fetch holiday failed")))
    }

    async fn fetch_once(client: &reqwest::Client, url: &str) -> Result<RemoteHoliday> {
        let response = client.get(url).send().await?;
        if !response.status().is_success() {
            bail!("Fail to download file: HTTP status {}", response.status());
        }
        let bytes = response.bytes().await?;
        let raw = String::from_utf8(bytes.to_vec())?;
        let data = parse_holidays_of_year(&raw)?;
        Ok(RemoteHoliday { raw, data })
    }

    async fn set_state(&self, key: &str, value: KeyState) {
        Self::set_state_shared(&self.state, key, value).await;
    }

    async fn set_state_shared(
        state: &Arc<Mutex<HashMap<String, KeyState>>>,
        key: &str,
        value: KeyState,
    ) {
        let mut state = state.lock().await;
        state.insert(key.to_string(), value);
    }
}
