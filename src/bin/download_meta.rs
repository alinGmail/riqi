use reqwest::Client;
use riqi::holiday_updater::download_meta_file;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let meate_url = "https://raw.githubusercontent.com/alinGmail/riqi/refs/heads/main/resources/holidays/meta.json";
    let meta = download_meta_file(&client, meate_url).await;
    println!("{:?}", meta.unwrap())
}
