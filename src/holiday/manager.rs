use crate::holiday::modal::{parse_holidays_of_year, HolidayOfYearList};
use crate::holiday::utils::{get_holiday_cache_file_path, get_ylc_code};
use chrono::{NaiveDate, Utc, DateTime, NaiveDateTime, Duration};
use color_eyre::eyre::{bail, OptionExt};
use color_eyre::Result;
use log::{debug, error, info};
use std::ffi::c_void;
use std::{collections::HashMap, fs, sync::Arc};
use tokio::sync::Mutex;

pub enum LoadRemoteState {
    None,
    Loading,
    Finish,
    Failed,
}

pub struct HolidayManagerProperty {
    ylc_holiday_update_state: HashMap<String, YlcHolidayUpdateState>,
}

pub struct YlcHolidayUpdateState {
    loaded_local_cache: bool,
    local_cache_time: Option<NaiveDate>,
    load_remote_state: LoadRemoteState,
}

pub struct HolidayManager {
    property: Arc<Mutex<HolidayManagerProperty>>,
}

pub fn get_holiday_data_file_url(year: &str, language: &str, country: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/alinGmail/riqi/refs/heads/main/resources/holidays/{}/{}_{}.json",
        year,
        language,
        country
    )
}

pub fn is_need_update(modify_time: NaiveDateTime) -> bool{
    // 1. 获取当前的 UTC NaiveDateTime
    let now = Utc::now().naive_utc();
    let last_need_update_time = now - Duration::days(10);
    modify_time < last_need_update_time
}

pub async fn download_file(url: &str) -> Result<(String)> {
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        error!("Fail to download file: HTTP status {}", response.status());
        bail!("Fail to download file: HTTP status {}", response.status())
    }
    let content = response.bytes().await?;
    Ok(String::from_utf8(content.into()).expect("Found invalid utf-8"))
}
pub fn parse_holidays(json_str: &str) -> Result<HolidayOfYearList,serde_json::Error> {
    serde_json::from_str(json_str)
}

pub fn load_holidays_file(year: &str, language: &str, country: &str) -> Result<String> {
    let path = get_holiday_cache_file_path(year, language, country)
        .ok_or_eyre("get holiday cache file path failed")?;
    let content = std::fs::read_to_string(path.as_path())?;
    Ok(content)
}

impl HolidayManager {
    pub fn new() -> Self {
        Self {
            property: Arc::new(Mutex::new(HolidayManagerProperty {
                ylc_holiday_update_state: HashMap::new(),
            })),
        }
    }

    pub fn load_local_cache(
        ylc_update_state: &mut YlcHolidayUpdateState,
        year: &str,
        language: &str,
        country: &str,
    ) -> Result<Option<NaiveDateTime>> {
        if ylc_update_state.loaded_local_cache {
            return Ok(None);
        }
        let file_cache_path = get_holiday_cache_file_path(year, language, country);
        if let Some(cache_path) = file_cache_path {
            let meatdata = fs::metadata(&cache_path)?;
            // 读取文件
            let modify_time = meatdata.modified()?;
            // 1. 先转为 DateTime<Utc>
            let datetime: DateTime<Utc> = modify_time.into();
            // 2. 再提取 NaiveDateTime (不含时区信息)
            let modify_naive_time: NaiveDateTime = datetime.naive_utc();
            
            let holiday_content_str = std::fs::read_to_string(cache_path.as_path());
            // parse 文件
            let holiday_year_list = parse_holidays_of_year(&holiday_content_str?);
            // 发送事件，给main 线程处理
            ylc_update_state.loaded_local_cache = true;
            Ok(Some(modify_naive_time))
        } else {
            info!("load local cache file not exist");
            bail!("local cache file not exist")
        }
        
    }

    pub fn load_remote_file(
        ylc_update_state: &mut YlcHolidayUpdateState,
        year: &str,
        language: &str,
        country: &str,
    ) -> Result<()> {
        if !matches!(ylc_update_state.load_remote_state, LoadRemoteState::None) {
            return Ok(());
        }
        ylc_update_state.load_remote_state = LoadRemoteState::Loading;
        let url = get_holiday_data_file_url(year, language, country);
        let content = download_file(&url);

        return Ok(());
    }

    pub async fn load_ylc_holiday(&self, year: &str, language: &str, country: &str) {
        {
            let mut property = self.property.lock().await;
            let ylc_update_property = property
                .ylc_holiday_update_state
                .entry(get_ylc_code(year, language, country))
                .or_insert(YlcHolidayUpdateState {
                    loaded_local_cache: false,
                    local_cache_time: None,
                    load_remote_state: LoadRemoteState::None,
                });
            
            let load_cache_res =
                HolidayManager::load_local_cache(ylc_update_property, year, language, country);
            match load_cache_res {
                Ok(modify_time) => {
                    info!("load local cache success");
                    if let Some(modify_time) = modify_time {
                       if !is_need_update(modify_time) {
                           return;
                       } 
                    } else {
                        return;
                    }
                }
                Err(_) => {
                    info!("load local cache fail");
                    return;
                }
            }
            let load_remove_res = HolidayManager::load_remote_file(ylc_update_property, year, language, country);
            
            
        }
    }
}
