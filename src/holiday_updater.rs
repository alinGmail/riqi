use crate::types::holiday_meta::HolidayMeta;
use color_eyre::eyre::Result;
use reqwest::Client;

pub async fn download_meta_file(client: &Client, url: &str) -> Result<HolidayMeta> {
    let response = client.get(url).send().await?;
    let meta = response.json::<HolidayMeta>().await?;
    Ok(meta)
}
