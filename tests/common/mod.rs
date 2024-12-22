use opml_manager::Feed;
use tokio::runtime::Runtime;
use std::collections::{HashMap, HashSet};

pub fn get_test_runtime() -> Runtime {
    Runtime::new().unwrap()
}

pub fn create_test_feed(title: &str, url: &str) -> Feed {
    Feed::new(title.to_string(), url.to_string(), None, vec![])
}

pub fn create_test_feed_with_categories(title: &str, url: &str, categories: Vec<&str>) -> Feed {
    Feed::new(
        title.to_string(),
        url.to_string(),
        None,
        categories.into_iter().map(String::from).collect(),
    )
}

pub fn extract_domain(url: &str) -> String {
    url.replace("http://", "")
        .replace("https://", "")
        .split('/')
        .next()
        .unwrap_or("")
        .to_string()
}
