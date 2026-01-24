use crate::holiday::utils::{get_holiday_cache_file_path, get_ylc_code};
use chrono::NaiveDate;
use color_eyre::eyre::OptionExt;
use color_eyre::Result;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

enum LoadRemoteState {
    None,
    Loading,
    Finish,
    Failed,
}

struct HolidayManagerProperty {
    ylc_holiday_update_state: HashMap<String, YlcHolidayUpdateState>,
}

struct YlcHolidayUpdateState {
    loaded_local_cache: bool,
    local_cache_time: Option<NaiveDate>,
    load_remote_state: LoadRemoteState,
}

struct HolidayManager {
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
            // 先加载本地缓存，然后看是否需要更新到最新的

            if !ylc_update_property.loaded_local_cache {
                let file_cache_path = get_holiday_cache_file_path(year, language, country);
                if let Some(cache_path) = file_cache_path {
                    
                    // 读取文件
                    
                }

                ylc_update_property.loaded_local_cache = true;
            }
        }
    }
}
