use opml_manager::Feed;
use tokio::runtime::Runtime;

pub fn get_test_runtime() -> Runtime {
    Runtime::new().unwrap()
}

pub fn create_test_feed(title: &str, url: &str) -> Feed {
    Feed::new(title.to_string(), url.to_string(), None, vec![])
}
