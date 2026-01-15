mod config;

use chrono::Local;

fn main() {
    // 获取本地当前时间
    let now = Local::now();
    
    // 格式化输出 (年-月-日 时:分:秒)
    println!("当前的系统时间是: {}", now.format("%Y-%m-%d %H:%M:%S"));
    
    // 如果你只需要日期
    println!("当前日期: {}", now.format("%Y-%m-%d"));

    let (language,country_option) = config::locale::get_system_language_country();

    let country = country_option.unwrap();
    println!("当前的系统是：{}-{}", language,country);
}
