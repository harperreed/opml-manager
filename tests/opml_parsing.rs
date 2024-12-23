use opml_manager::error::OPMLError;
use opml_manager::opml::parse_opml;

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
fn test_empty_category_nodes() {
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
    <opml version="2.0">
        <head><title>Test</title></head>
        <body>
            <outline text="Empty Category">
                <outline text="Another Empty"/>
            </outline>
        </body>
    </opml>"#;

    let feeds = parse_opml(content).unwrap();
    assert!(feeds.is_empty());
}

#[test]
fn test_missing_required_attributes() {
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
    <opml version="2.0">
        <head><title>Test</title></head>
        <body>
            <outline type="rss" xmlUrl="http://example.com/feed.xml"/>
            <outline text="Only Text"/>
            <outline type="rss"/>
        </body>
    </opml>"#;

    let feeds = parse_opml(content).unwrap();
    assert_eq!(feeds.len(), 0); // Should ignore invalid feeds
}

#[test]
fn test_deeply_nested_categories() {
    let mut content = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
    <opml version="2.0">
        <head><title>Test</title></head>
        <body>"#,
    );

    // Create 101 levels of nesting
    for i in 0..101 {
        content.push_str(&format!("<outline text=\"Category{}\">", i));
    }
    content
        .push_str(r#"<outline type="rss" text="Deep Feed" xmlUrl="http://example.com/feed.xml"/>"#);
    for _ in 0..101 {
        content.push_str("</outline>");
    }
    content.push_str("</body></opml>");

    let result = parse_opml(&content);
    assert!(matches!(result, Err(OPMLError::CategoryNestingTooDeep(_))));
}

#[test]
fn test_malformed_category_structure() {
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
    <opml version="2.0">
        <head><title>Test</title></head>
        <body>
            <outline>
                <notanoutline type="rss" text="Feed" xmlUrl="http://example.com/feed.xml"/>
            </outline>
        </body>
    </opml>"#;

    let feeds = parse_opml(content).unwrap();
    assert_eq!(feeds.len(), 0); // Should ignore invalid elements
}

#[test]
fn test_large_opml() {
    let mut content = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
    <opml version="2.0">
        <head><title>Large Test</title></head>
        <body>"#,
    );

    // Add 10,000 feeds
    for i in 0..10_000 {
        content.push_str(&format!(
            r#"<outline type="rss" text="Feed {}" xmlUrl="http://example.com/feed{}.xml"/>"#,
            i, i
        ));
    }
    content.push_str("</body></opml>");

    let feeds = parse_opml(&content).unwrap();
    assert_eq!(feeds.len(), 10_000);
}

#[test]
fn test_invalid_utf8_sequences() {
    // Create XML with an illegal character sequence
    let content = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><opml version=\"2.0\"><head><title>Test</title></head><body><outline type=\"rss\" text=\"Bad Feed \x1B\" xmlUrl=\"http://example.com/feed.xml\"/></body></opml>";

    let result = parse_opml(content);
    assert!(result.is_err());
}

#[test]
fn test_url_normalization() {
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
    <opml version="2.0">
        <head><title>Normalization Test</title></head>
        <body>
            <outline type="rss" text="Feed 1" xmlUrl="http://example.com/feed"/>
            <outline type="rss" text="Feed 2" xmlUrl="http://example.com/feed/"/>
            <outline type="rss" text="Feed 3" xmlUrl="https://example.com/feed"/>
            <outline type="rss" text="Feed 4" xmlUrl="http://example.com/FEED"/>
        </body>
    </opml>"#;

    let feeds = parse_opml(content).unwrap();
    assert_eq!(feeds.len(), 1); // All URLs should be normalized to the same value
}

#[test]
fn test_nested_categories() {
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
    <opml version="2.0">
        <head><title>Nested Categories Test</title></head>
        <body>
            <outline text="Category 1">
                <outline text="Subcategory 1.1">
                    <outline type="rss" text="Feed 1" xmlUrl="http://example.com/feed1.xml"/>
                </outline>
                <outline text="Subcategory 1.2">
                    <outline type="rss" text="Feed 2" xmlUrl="http://example.com/feed2.xml"/>
                </outline>
            </outline>
            <outline text="Category 2">
                <outline type="rss" text="Feed 3" xmlUrl="http://example.com/feed3.xml"/>
            </outline>
        </body>
    </opml>"#;

    let feeds = parse_opml(content).unwrap();
    assert_eq!(feeds.len(), 3);

    assert_eq!(feeds[0].title, "Feed 1");
    assert_eq!(feeds[0].xml_url, "http://example.com/feed1.xml");
    assert_eq!(feeds[0].category, vec!["Category 1", "Subcategory 1.1"]);

    assert_eq!(feeds[1].title, "Feed 2");
    assert_eq!(feeds[1].xml_url, "http://example.com/feed2.xml");
    assert_eq!(feeds[1].category, vec!["Category 1", "Subcategory 1.2"]);

    assert_eq!(feeds[2].title, "Feed 3");
    assert_eq!(feeds[2].xml_url, "http://example.com/feed3.xml");
    assert_eq!(feeds[2].category, vec!["Category 2"]);
}
