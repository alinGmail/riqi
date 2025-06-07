pub fn load_holidays_file(lang: &str, country: &str) -> std::io::Result<String> {
    // 开发环境：从项目目录加载
    #[cfg(debug_assertions)]
    {
        let path = format!("resources/holidays/{}_{}.json", country, lang);
        std::fs::read_to_string(path)
    }

    // 生产环境：从嵌入资源或系统目录加载
    #[cfg(not(debug_assertions))]
    {
        // 方法1：嵌入资源
        match (lang, country) {
            ("zh", "cn") => Ok(include_str!("../resources/holidays/cn_zh.json").to_string()),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Holiday file not found",
            )),
        }

        // 或方法2：从系统目录加载
        // let system_path = get_system_data_path(lang, country);
        // std::fs::read_to_string(system_path)
    }
}
