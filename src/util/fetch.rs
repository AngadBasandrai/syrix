pub fn fetch(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(reqwest::blocking::get(url)?.text()?)
}