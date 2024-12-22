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
