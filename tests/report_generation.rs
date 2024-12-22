use opml_manager::report::format_markdown_report;
use opml_manager::Feed;
use std::collections::{HashMap, HashSet};

#[test]
fn test_format_markdown_report() {
    let feed = Feed::new(
        "Test Feed".to_string(),
        "http://example.com/feed.xml".to_string(),
        None,
        vec![],
    );
    let feeds = vec![feed];

    let mut seen_urls = HashSet::new();
    seen_urls.insert("http://example.com/feed.xml".to_string());

    let duplicates = vec![];
    let mut categories = HashSet::new();
    categories.insert("Category1".to_string());

    let mut domain_counter = HashMap::new();
    domain_counter.insert("example.com".to_string(), 1);

    let report = format_markdown_report(
        &feeds,
        &seen_urls,
        &duplicates,
        &categories,
        &domain_counter,
    );

    assert!(report.contains("# OPML Analysis Report"));
    assert!(report.contains("Total Feeds: 1"));
}
