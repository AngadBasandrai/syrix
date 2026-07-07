pub struct Page {
    url: String,
    html: String,
}

impl Page {
    pub fn new(url: String, html: String) -> Self {
        Self { url, html }
    }

    pub fn print_summary(&self) {
        println!("URL: {}\nHTML: {} bytes", self.url, self.html.len());
    }

    pub fn html(&self) -> &str {
        &self.html
    }
}
