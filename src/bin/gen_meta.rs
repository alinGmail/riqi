use chrono::{DateTime, TimeZone, Utc};
use serde_json::json;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn get_year_dirs(base: &Path) -> Vec<PathBuf> {
    fs::read_dir(base)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.is_dir())
                .filter(|p| {
                    p.file_name()
                        .and_then(|n| n.to_str())
                        .map_or(false, |n| n.chars().all(|c| c.is_ascii_digit()))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn get_json_files(year_dir: &Path) -> Vec<(String, PathBuf)> {
    let year = year_dir.file_name().and_then(|n| n.to_str()).unwrap_or("");
    fs::read_dir(year_dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().map_or(false, |ext| ext == "json"))
                .filter_map(|p| {
                    let file_stem = p.file_stem().and_then(|stem| stem.to_str())?;
                    Some((format!("{}_{}", year, file_stem), p.clone()))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn get_last_modified(path: &Path) -> String {
    fs::metadata(path)
        .and_then(|meta| meta.modified())
        .ok()
        .and_then(|mtime| mtime.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| chrono::Utc.timestamp(d.as_secs() as i64, 0).to_rfc3339())
        .unwrap_or_else(|| Utc::now().to_rfc3339())
}

fn main() -> std::io::Result<()> {
    let holidays_dir = Path::new("resources/holidays");
    let meta_path = holidays_dir.join("meta.json");
    let mut files_map = BTreeMap::new();

    for year_dir in get_year_dirs(holidays_dir) {
        for (key, file_path) in get_json_files(&year_dir) {
            let last_modified = get_last_modified(&file_path);
            files_map.insert(key, json!({ "last_modified": last_modified }));
        }
    }

    let now = Utc::now().to_rfc3339();
    let meta_json = json!({
        "last_updated": now,
        "files": files_map
    });

    let mut file = File::create(meta_path)?;
    file.write_all(serde_json::to_string_pretty(&meta_json)?.as_bytes())?;
    println!("meta.json 已生成");
    Ok(())
}

