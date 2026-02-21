use crate::config::model::Source;
use crate::events::AppEvent;
use crate::holiday::modal::{parse_holidays_of_year, HolidayOfYearList};
use crate::holiday::utils::{get_holiday_cache_file_path, get_ylc_code};
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use color_eyre::eyre::{bail, OptionExt};
use color_eyre::Result;
use log::{error, info};
use std::io::Bytes;
use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::{collections::HashMap, sync::Arc};
use tokio::fs;
use tokio::io::AsyncWriteExt;
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
    tx: Sender<AppEvent>,
}

pub fn get_holiday_data_file_url(
    year: &str,
    language: &str,
    country: &str,
    source: &Source,
) -> String {
    match source {
        Source::Github => format!(
            "https://raw.githubusercontent.com/alinGmail/riqi/refs/heads/main/resources/holidays/{}/{}_{}.json",
            year,
            language,
            country
        ),
        Source::Gitee => format!(
            "https://gitee.com/zhaixiaolin/riqi/raw/main/resources/holidays/{}/{}_{}.json",
            year,
            language,
            country
        ),
    }
}

pub fn is_need_update(modify_time: NaiveDateTime) -> bool {
    // 1. 获取当前的 UTC NaiveDateTime
    let now = Utc::now().naive_utc();
    let last_need_update_time = now - Duration::days(10);
    modify_time < last_need_update_time
}

pub async fn save_holidays_file(
    year: &str,
    language: &str,
    country: &str,
    content: &[u8],
) -> Result<()> {
    let path = get_holiday_cache_file_path(year, language, country);
    if path == None {
        bail!("get holiday cache file path failed")
    }
    let path_unwrap = path.unwrap();
    // 2. 确保目录存在
    if let Some(parent) = path_unwrap.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).await?;
        }
    }
    // 3. 写入文件
    let mut file = fs::File::create(&path_unwrap).await?;
    file.write_all(content).await?;
    info!(
        "Successfully downloaded and saved file to {}",
        path_unwrap.display()
    );
    Ok(())
}

pub async fn download_file(url: &str) -> Result<String> {
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        error!("Fail to download file: HTTP status {}", response.status());
        bail!("Fail to download file: HTTP status {}", response.status())
    }
    let content = response.bytes().await?;
    Ok(String::from_utf8(content.into()).expect("Found invalid utf-8"))
}
pub fn parse_holidays(json_str: &str) -> Result<HolidayOfYearList, serde_json::Error> {
    serde_json::from_str(json_str)
}

impl HolidayManager {
    pub fn new(tx: Sender<AppEvent>) -> Self {
        Self {
            property: Arc::new(Mutex::new(HolidayManagerProperty {
                ylc_holiday_update_state: HashMap::new(),
            })),
            tx,
        }
    }

    pub fn load_local_cache(
        ylc_update_state: &mut YlcHolidayUpdateState,
        year: &str,
        language: &str,
        country: &str,
        tx_sender: Sender<AppEvent>,
    ) -> Result<(Option<NaiveDateTime>, i32)> {
        if ylc_update_state.loaded_local_cache {
            return Ok((None, 0));
        }
        let file_cache_path = get_holiday_cache_file_path(year, language, country);
        if let Some(cache_path) = file_cache_path {
            let meatdata = std::fs::metadata(&cache_path)?;
            // 读取文件
            let modify_time = meatdata.modified()?;
            // 1. 先转为 DateTime<Utc>
            let datetime: DateTime<Utc> = modify_time.into();
            // 2. 再提取 NaiveDateTime (不含时区信息)
            let modify_naive_time: NaiveDateTime = datetime.naive_utc();

            let holiday_content_str = std::fs::read_to_string(cache_path.as_path());
            // parse 文件
            let holiday_year_list = parse_holidays_of_year(&holiday_content_str?);
            let holiday_year_list_un_wrap = holiday_year_list?;
            let version = holiday_year_list_un_wrap.version;

            tx_sender.send(AppEvent::UpdateHoliday(
                get_ylc_code(year, language, country),
                holiday_year_list_un_wrap,
            ))?;

            // 发送事件，给main 线程处理
            ylc_update_state.loaded_local_cache = true;
            Ok((Some(modify_naive_time), version))
        } else {
            info!("load local cache file not exist");
            bail!("local cache file not exist")
        }
    }

    pub async fn load_remote_file(
        property: Arc<Mutex<HolidayManagerProperty>>,
        year: &str,
        language: &str,
        country: &str,
        source: Source,
        tx: Sender<AppEvent>,
        old_version: Option<i32>,
    ) -> Result<()> {
        {
            let mut property = property.lock().await;
            let ylc_update_state = property
                .ylc_holiday_update_state
                .get_mut(&get_ylc_code(year, language, country))
                .unwrap();
            if !matches!(ylc_update_state.load_remote_state, LoadRemoteState::None) {
                return Ok(());
            }
            info!("start to load remote file");
            ylc_update_state.load_remote_state = LoadRemoteState::Loading;
        }

        let url = get_holiday_data_file_url(year, language, country, &source);
        info!("remote url is {}", &url);
        let content = download_file(&url).await;
        if let Ok(content_str) = content {
            let holiday_of_ylc = parse_holidays(&content_str)?;
            // save to local file
            if let Some(old_version) = old_version {
                if (old_version < holiday_of_ylc.version) {
                    // save file
                    let save_res =
                        save_holidays_file(year, language, country, content_str.as_bytes()).await;
                }
            } else {
                let save_res =
                    save_holidays_file(year, language, country, content_str.as_bytes()).await;
            }

            tx.send(AppEvent::UpdateHoliday(
                get_ylc_code(year, language, country),
                holiday_of_ylc,
            ))?;
            {
                let mut property = property.lock().await;
                let ylc_update_state = property
                    .ylc_holiday_update_state
                    .get_mut(&get_ylc_code(year, language, country))
                    .unwrap();
                ylc_update_state.load_remote_state = LoadRemoteState::Finish;
            }
        }
        return Ok(());
    }

    pub async fn load_ylc_holiday(&self, year: &str, language: &str, country: &str, source: Source) {
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

            let load_cache_res = HolidayManager::load_local_cache(
                ylc_update_property,
                year,
                language,
                country,
                self.tx.clone(),
            );

            let mut old_version: Option<i32> = None;
            match load_cache_res {
                Ok((modify_time, version)) => {
                    old_version = Some(version);
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
                }
            }
            let property_clone = self.property.clone();
            let tx_clone = self.tx.clone();

            let year_owned = year.to_string();
            let language_owned = language.to_string();
            let country_owned = country.to_string();
            let source_owned = source.clone();

            tokio::spawn(async move {
                let load_remote_res = HolidayManager::load_remote_file(
                    property_clone,
                    &year_owned,
                    &language_owned,
                    &country_owned,
                    source_owned,
                    tx_clone,
                    old_version,
                )
                .await;
            });
        }
    }
}
