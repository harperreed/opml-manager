use crate::error::OPMLError;
use crate::{Feed, Result};
use reqwest::Client;
use serde::Serialize;
use tokio::time::{sleep, Duration};
use std::time::Instant;

#[derive(Debug, Serialize)]
pub struct ValidationResult {
    pub feed: String,
    pub url: String,
    pub status: String,
    pub error: String,
    pub categories: Vec<String>,
}

pub async fn validate_feed(feed: &Feed, client: &Client) -> Result<ValidationResult> {
    let mut attempts = 0;
    let max_attempts = 5;
    let mut backoff = Duration::from_secs(1);

    loop {
        attempts += 1;
        let start = Instant::now();

        let response = match client.get(&feed.xml_url).send().await {
            Ok(resp) => resp,
            Err(e) => {
                if e.is_timeout() {
                    return Err(OPMLError::Http(e));
                }

                if attempts < max_attempts && (e.is_connect() || e.is_request()) {
                    let elapsed = start.elapsed();
                    if elapsed < backoff {
                        sleep(backoff - elapsed).await;
                    }
                    backoff *= 2;
                    continue;
                }

                return Err(OPMLError::Http(e));
            }
        };

        if response.status().is_success() {
            let text = response.text().await
                .map_err(|e| OPMLError::ResponseParsingError(e.to_string()))?;

            match roxmltree::Document::parse(&text) {
                Ok(doc) => {
                    let root = doc.root_element();
                    let is_rss = root.children()
                        .find(|n| n.has_tag_name("rss") || n.has_tag_name("channel"))
                        .is_some();
                    let is_atom = root.has_tag_name("feed");
                    
                    if is_rss || is_atom {
                        Ok(ValidationResult {
                            feed: feed.title.clone(),
                            url: feed.xml_url.clone(),
                            status: "valid".to_string(),
                            error: String::new(),
                            categories: feed.category.clone(),
                        })
                    } else {
                        Err(OPMLError::FeedParsingError(
                            "Document is not a valid RSS or Atom feed".to_string()
                        ))
                    }
                },
                Err(e) => Err(OPMLError::XMLParsing(e)),
            }?
        } else if response.status().is_server_error() && attempts < max_attempts {
            let elapsed = start.elapsed();
            if elapsed < backoff {
                sleep(backoff - elapsed).await;
            }
            backoff *= 2;
            continue;
        } else {
            return Err(OPMLError::Http(
                reqwest::Error::from(reqwest::StatusError::new(response))
            ));
        }

        if attempts >= max_attempts {
            return Err(OPMLError::ValidationError(
                "Max retry attempts reached".to_string()
            ));
        }
    }
}