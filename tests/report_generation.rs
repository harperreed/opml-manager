use opml_manager::report::format_markdown_report;
use opml_manager::Feed;
use std::collections::{HashMap, HashSet};
use crate::common::{create_test_feed, create_test_feed_with_categories, extract_domain};

mod common;

#[test]
fn test_empty_feed_list() {
    let feeds: Vec<Feed> = vec![];
    let seen_urls = HashSet::new();
    let duplicates = vec![];
    let categories = HashSet::new();
    let domain_counter = HashMap::new();

    let report = format_markdown_report(
        &feeds,
        &seen_urls,
        &duplicates,
        &categories,
        &domain_counter,
    );

    assert!(report.contains("# OPML Analysis Report"));
    assert!(report.contains("Total Feeds: 0"));
    assert!(report.contains("No duplicate feeds found"));
    assert!(report.contains("No categories found"));
}

#[test]
fn test_special_characters() {
    let feed = create_test_feed(
        "Test & Feed with < > Special \"Chars\"",
        "http://example.com/feed.xml",
    );
    let feeds = vec![feed];
    
    let mut seen_urls = HashSet::new();
    seen_urls.insert("http://example.com/feed.xml".to_string());
    
    let duplicates = vec![];
    let categories = HashSet::new();
    let mut domain_counter = HashMap::new();
    domain_counter.insert("example.com".to_string(), 1);

    let report = format_markdown_report(
        &feeds,
        &seen_urls,
        &duplicates,
        &categories,
        &domain_counter,
    );

    assert!(report.contains("Test &amp; Feed with &lt; &gt; Special &quot;Chars&quot;"));
}

#[test]
fn test_duplicate_statistics() {
    let feed1 = create_test_feed("Feed 1", "http://example.com/feed.xml");
    let feed2 = create_test_feed("Feed 2", "http://example.com/feed.xml");
    let feeds = vec![feed1.clone(), feed2];
    
    let mut seen_urls = HashSet::new();
    seen_urls.insert("http://example.com/feed.xml".to_string());
    
    let duplicates = vec![&feed1];
    let categories = HashSet::new();
    let mut domain_counter = HashMap::new();
    domain_counter.insert("example.com".to_string(), 2);

    let report = format_markdown_report(
        &feeds,
        &seen_urls,
        &duplicates,
        &categories,
        &domain_counter,
    );

    assert!(report.contains("Duplicate Feeds Found"));
    assert!(report.contains("Feed 1"));
    assert!(report.contains("Total Feeds: 2"));
}

#[test]
fn test_category_grouping() {
    let feed1 = create_test_feed_with_categories(
        "Tech Feed",
        "http://tech.com/feed.xml",
        vec!["Technology", "News"],
    );
    let feed2 = create_test_feed_with_categories(
        "News Feed",
        "http://news.com/feed.xml",
        vec!["News"],
    );
    let feeds = vec![feed1, feed2];
    
    let mut seen_urls = HashSet::new();
    seen_urls.insert("http://tech.com/feed.xml".to_string());
    seen_urls.insert("http://news.com/feed.xml".to_string());
    
    let duplicates = vec![];
    let mut categories = HashSet::new();
    categories.insert("Technology".to_string());
    categories.insert("News".to_string());
    
    let mut domain_counter = HashMap::new();
    domain_counter.insert("tech.com".to_string(), 1);
    domain_counter.insert("news.com".to_string(), 1);

    let report = format_markdown_report(
        &feeds,
        &seen_urls,
        &duplicates,
        &categories,
        &domain_counter,
    );

    assert!(report.contains("Categories Found: 2"));
    assert!(report.contains("Technology"));
    assert!(report.contains("News"));
    assert!(report.contains("tech.com"));
    assert!(report.contains("news.com"));
}
