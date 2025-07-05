pub fn get_lc_code(language: &str, country: &str) -> String {
    return format!("{}_{}", language, country);
}

pub fn get_ylc_code(year: &str, language: &str, country: &str) -> String {
    return format!("{}_{}_{}", year, language, country);
}
