pub fn load_holidays_file(lang: &str, region: &str) -> std::io::Result<String> {
    // 开发环境：从项目目录加载
    #[cfg(debug_assertions)]
    {
        let path = format!("resources/holidays/{}_{}.json", region, lang);
        return std::fs::read_to_string(path);
    }

    // 生产环境：从嵌入资源或系统目录加载
    #[cfg(not(debug_assertions))]
    {
        // 方法1：嵌入资源
        match (lang, region) {
            ("zh", "cn") => Ok(include_str!("../resources/holidays/cn_zh.json").to_string()),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Holiday file not found",
            )),
        }

        // 或方法2：从系统目录加载
        // let system_path = get_system_data_path(lang, region);
        // std::fs::read_to_string(system_path)
    }
}
