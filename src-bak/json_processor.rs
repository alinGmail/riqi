use serde_json::Value;
use std::fs;

#[derive(serde::Deserialize)]
struct Holiday {
    name: String,
    date: Date,
    #[serde(rename = "type")]
    holiday_type: Vec<String>,
    primary_type: String,
}

#[derive(serde::Deserialize)]
struct Date {
    iso: String,
    datetime: DateTime,
}

#[derive(serde::Deserialize)]
struct DateTime {
    year: i32,
    month: i32,
    day: i32,
    #[serde(default)]
    hour: Option<i32>,
    #[serde(default)]
    minute: Option<i32>,
    #[serde(default)]
    second: Option<i32>,
}

pub fn process_holiday_json() -> Result<(), Box<dyn std::error::Error>> {
    // 读取 JSON 文件
    let content = fs::read_to_string("resources/holidays/cn_zh.json")?;
    let mut json: Value = serde_json::from_str(&content)?;

    // 获取 holidays 数组
    if let Some(holidays) = json.get_mut("holidays") {
        if let Some(holidays_array) = holidays.as_array_mut() {
            // 处理每个假期
            for holiday in holidays_array {
                if let Some(obj) = holiday.as_object_mut() {
                    obj.remove("canonical_url");
                    obj.remove("urlid");
                    obj.remove("locations");
                    obj.remove("states");
                    obj.remove("description");
                    obj.remove("country");
                }

                // 将 datetime 提升到顶层并移除 date
                if let Some(date) = holiday.get("date") {
                    if let Some(datetime) = date.get("datetime") {
                        let datetime_clone = datetime.clone();
                        if let Some(obj) = holiday.as_object_mut() {
                            obj.insert("datetime".to_string(), datetime_clone);
                        }
                    }
                }
            }
        }
    }

    // 将处理后的 JSON 写入新文件
    let output = serde_json::to_string_pretty(&json)?;
    fs::write("resources/holidays/cn_zh_processed.json", output)?;

    println!("处理完成！");
    Ok(())
}
