#[derive(Debug, Clone)]
pub struct Feed {
    pub title: String,
    pub xml_url: String,
    pub html_url: Option<String>,
    pub category: Vec<String>,
}

impl Feed {
    pub fn new(
        title: String,
        xml_url: String,
        html_url: Option<String>,
        category: Vec<String>,
    ) -> Self {
        Feed {
            title,
            xml_url,
            html_url,
            category,
        }
    }
}
