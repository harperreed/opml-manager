use opml_manager::{
    feed::Feed,
    opml::{generate_opml, parse_opml},
    report::{format_markdown_report, generate_summary},
    validation::validate_feed,
};
use mockito; // Import mockito directly
use std::collections::{HashMap, HashSet};

mod opml_parsing {
    use super::*;

    #[test]
    fn test_parse_empty_opml() {
        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
        <opml version="2.0">
            <head><title>Empty Test</title></head>
            <body></body>
        </opml>"#;

        let feeds = parse_opml(content).unwrap();
        assert!(feeds.is_empty());
    }

    #[test]
    fn test_parse_malformed_xml() {
        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
        <opml version="2.0">
            <head><title>Malformed Test</title></head>
            <body>
                <outline type="rss" text="Test Feed" xmlUrl="http://example.com/feed.xml"
            </body>
        </opml>"#;

        assert!(parse_opml(content).is_err());
    }

    #[test]
    fn test_parse_nested_categories() {
        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
        <opml version="2.0">
            <head><title>Test</title></head>
            <body>
                <outline text="Category1">
                    <outline text="Category2">
                        <outline type="rss" text="Test Feed" xmlUrl="http://example.com/feed.xml"/>
                    </outline>
                </outline>
            </body>
        </opml>"#;

        let feeds = parse_opml(content).unwrap();
        assert_eq!(feeds.len(), 1);
        assert_eq!(feeds[0].category, vec!["Category1", "Category2"]);
    }

    #[test]
    fn test_parse_mixed_feeds_and_categories() {
        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
        <opml version="2.0">
            <head><title>Test</title></head>
            <body>
                <outline type="rss" text="Uncategorized Feed" xmlUrl="http://example.com/feed1.xml"/>
                <outline text="Category1">
                    <outline type="rss" text="Categorized Feed" xmlUrl="http://example.com/feed2.xml"/>
                    <outline text="Subcategory">
                        <outline type="rss" text="Nested Feed" xmlUrl="http://example.com/feed3.xml"/>
                    </outline>
                </outline>
            </body>
        </opml>"#;

        let feeds = parse_opml(content).unwrap();
        assert_eq!(feeds.len(), 3);
        
        // Check uncategorized feed
        let uncategorized = feeds.iter().find(|f| f.xml_url.ends_with("feed1.xml")).unwrap();
        assert!(uncategorized.category.is_empty());
        
        // Check categorized feed
        let categorized = feeds.iter().find(|f| f.xml_url.ends_with("feed2.xml")).unwrap();
        assert_eq!(categorized.category, vec!["Category1"]);
        
        // Check nested feed
        let nested = feeds.iter().find(|f| f.xml_url.ends_with("feed3.xml")).unwrap();
        assert_eq!(nested.category, vec!["Category1", "Subcategory"]);
    }
}

mod opml_generation {
    use super::*;

    #[test]
    fn test_generate_empty_opml() {
        let feeds = vec![];
        let output = generate_opml(&feeds).unwrap();
        assert!(output.contains("<opml version=\"2.0\">"));
        assert!(output.contains("<body>"));
        assert!(output.contains("</body>"));
    }

    #[test]
    fn test_generate_with_categories() {
        let feeds = vec![
            Feed::new(
                "Test Feed 1".to_string(),
                "http://example.com/feed1.xml".to_string(),
                None,
                vec!["Category1".to_string()],
            ),
            Feed::new(
                "Test Feed 2".to_string(),
                "http://example.com/feed2.xml".to_string(),
                Some("http://example.com".to_string()),
                vec!["Category1".to_string(), "Category2".to_string()],
            ),
        ];

        let output = generate_opml(&feeds).unwrap();
        
        // Check basic structure
        assert!(output.contains("<opml version=\"2.0\">"));
        
        // Check category structure
        assert!(output.contains(r#"<outline text="Category1">"#));
        assert!(output.contains(r#"<outline text="Category2">"#));
        
        // Check feed entries
        assert!(output.contains(r#"type="rss" text="Test Feed 1""#));
        assert!(output.contains(r#"type="rss" text="Test Feed 2""#));
        assert!(output.contains(r#"xmlUrl="http://example.com/feed1.xml""#));
        assert!(output.contains(r#"htmlUrl="http://example.com""#));
    }
}

mod feed_validation {
    use super::*;
    use mockito::Server;
    use std::sync::Once;

    // Initialize the runtime once
    static INIT: Once = Once::new();
    static mut RUNTIME: Option<tokio::runtime::Runtime> = None;

    // Helper function to get/create runtime
    fn get_runtime() -> &'static tokio::runtime::Runtime {
        unsafe {
            INIT.call_once(|| {
                RUNTIME = Some(
                    tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .unwrap(),
                );
            });
            RUNTIME.as_ref().unwrap()
        }
    }

    #[test]
    fn test_validate_valid_feed() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/feed.xml")
            .with_status(200)
            .with_body(r#"<?xml version="1.0"?><rss version="2.0"><channel><title>Test</title></channel></rss>"#)
            .create();

        let feed = Feed::new(
            "Test Feed".to_string(),
            format!("{}/feed.xml", server.url()),
            None,
            vec![],
        );

        let client = reqwest::Client::new();
        let result = get_runtime().block_on(async {
            validate_feed(&feed, &client).await
        }).unwrap();
        
        assert_eq!(result.status, "valid");
        assert!(result.error.is_empty());
        mock.assert();
    }

    #[test]
    fn test_validate_invalid_feed() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/feed.xml")
            .with_status(200)
            .with_body("Not XML content")
            .create();

        let feed = Feed::new(
            "Test Feed".to_string(),
            format!("{}/feed.xml", server.url()),
            None,
            vec![],
        );

        let client = reqwest::Client::new();
        let result = get_runtime().block_on(async {
            validate_feed(&feed, &client).await
        }).unwrap();
        
        assert_eq!(result.status, "invalid");
        assert!(!result.error.is_empty());
        mock.assert();
    }

    #[test]
    fn test_validate_unreachable_feed() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/feed.xml")
            .with_status(404)
            .create();

        let feed = Feed::new(
            "Test Feed".to_string(),
            format!("{}/feed.xml", server.url()),
            None,
            vec![],
        );

        let client = reqwest::Client::new();
        let result = get_runtime().block_on(async {
            validate_feed(&feed, &client).await
        }).unwrap();
        
        assert_eq!(result.status, "error");
        assert!(result.error.contains("404"));
        mock.assert();
    }
}

mod report_generation {
    use super::*;

    #[test]
    fn test_generate_summary() {
        let feeds = vec![
            Feed::new(
                "Test Feed 1".to_string(),
                "http://example.com/feed1.xml".to_string(),
                None,
                vec!["Category1".to_string()],
            ),
            Feed::new(
                "Test Feed 2".to_string(),
                "http://example.com/feed1.xml".to_string(), // Duplicate URL
                None,
                vec!["Category2".to_string()],
            ),
        ];

        let (seen_urls, duplicates, categories, domain_counter) = generate_summary(&feeds);
        
        assert_eq!(seen_urls.len(), 1); // Only one unique URL
        assert_eq!(duplicates.len(), 1); // One duplicate
        assert_eq!(categories.len(), 2); // Two unique categories
        assert_eq!(domain_counter.get("example.com"), Some(&2)); // Two feeds from example.com
    }

    #[test]
    fn test_format_markdown_report() {
        let feeds = vec![
            Feed::new(
                "Test Feed".to_string(),
                "http://example.com/feed.xml".to_string(),
                None,
                vec!["Category1".to_string()],
            ),
        ];

        let mut seen_urls = HashSet::new();
        seen_urls.insert("http://example.com/feed.xml".to_string());

        let duplicates = vec![];
        
        let mut categories = HashSet::new();
        categories.insert("Category1".to_string());

        let mut domain_counter = HashMap::new();
        domain_counter.insert("example.com".to_string(), 1);

        let report = format_markdown_report(&feeds, &seen_urls, &duplicates, &categories, &domain_counter);
        
        assert!(report.contains("# OPML Analysis Report"));
        assert!(report.contains("Total Feeds: 1"));
        assert!(report.contains("Unique Feeds: 1"));
        assert!(report.contains("Category1"));
        assert!(report.contains("example.com"));
    }
}
