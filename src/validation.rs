use crate::{Feed, Result};
use reqwest::Client;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ValidationResult {
    pub feed: String,
    pub url: String,
    pub status: String,
    pub error: String,
    pub categories: Vec<String>,
}

/// Validates a feed by attempting to fetch and parse it
///
/// # Arguments
/// * `feed` - The feed to validate
/// * `client` - HTTP client to use for the request
///
/// # Returns
/// A Result containing the validation status and any error information
pub async fn validate_feed(feed: &Feed, client: &Client) -> Result<ValidationResult> {
    let response = client.get(&feed.xml_url).send().await?;

    let result = if response.status().is_success() {
        let text = response.text().await?;
        match roxmltree::Document::parse(&text) {
            Ok(doc) => {
                // Check for RSS or Atom feed markers
                let root = doc.root_element();
                let is_rss = root.children()
                    .find(|n| n.has_tag_name("rss") || n.has_tag_name("channel"))
                    .is_some();
                let is_atom = root.has_tag_name("feed");
                
                if is_rss || is_atom {
                    ValidationResult {
                feed: feed.title.clone(),
                url: feed.xml_url.clone(),
                        status: "valid".to_string(),
                        error: String::new(),
                        categories: feed.category.clone(),
                    }
                } else {
                    ValidationResult {
                        feed: feed.title.clone(),
                        url: feed.xml_url.clone(),
                        status: "invalid".to_string(),
                        error: "Document is not a valid RSS or Atom feed".to_string(),
                        categories: feed.category.clone(),
                    }
                }
            },
            Err(e) => ValidationResult {
                feed: feed.title.clone(),
                url: feed.xml_url.clone(),
                status: "invalid".to_string(),
                error: e.to_string(),
                categories: feed.category.clone(),
            },
        }
    } else {
        ValidationResult {
            feed: feed.title.clone(),
            url: feed.xml_url.clone(),
            status: "error".to_string(),
            error: format!("HTTP {}", response.status()),
            categories: feed.category.clone(),
        }
    };

    Ok(result)
}
