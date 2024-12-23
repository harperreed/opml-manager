use opml_manager::validation::validate_feed;
mod common;
use std::time::Duration;

#[test]
fn test_validate_valid_feed() {
    let rt = common::get_test_runtime();
    let mut server = mockito::Server::new();
    let mock = server
        .mock("GET", "/feed.xml")
        .with_status(200)
        .with_header("content-type", "application/rss+xml")
        .with_body(r#"<?xml version="1.0"?><rss version="2.0"><channel><title>Test</title></channel></rss>"#)
        .create();

    let feed = common::create_test_feed("Test Feed", &format!("{}/feed.xml", server.url()));
    let client = reqwest::Client::new();

    let result = rt
        .block_on(async { validate_feed(&feed, &client).await })
        .unwrap();

    assert_eq!(result.status, "valid");
    assert!(result.error.is_empty());
    mock.assert();
}

#[test]
fn test_malformed_xml() {
    let rt = common::get_test_runtime();
    let mut server = mockito::Server::new();
    let mock = server
        .mock("GET", "/feed.xml")
        .with_status(200)
        .with_body("<?xml version='1.0'?><rss><broken>")
        .create();

    let feed = common::create_test_feed("Malformed Feed", &format!("{}/feed.xml", server.url()));
    let client = reqwest::Client::new();

    let result = rt
        .block_on(async { validate_feed(&feed, &client).await })
        .unwrap();

    assert_eq!(result.status, "invalid");
    assert!(!result.error.is_empty());
    mock.assert();
}

#[test]
fn test_valid_xml_invalid_feed() {
    let rt = common::get_test_runtime();
    let mut server = mockito::Server::new();
    let mock = server
        .mock("GET", "/feed.xml")
        .with_status(200)
        .with_body(r#"<?xml version="1.0"?><note><body>Not a feed</body></note>"#)
        .create();

    let feed = common::create_test_feed("Invalid Feed", &format!("{}/feed.xml", server.url()));
    let client = reqwest::Client::new();

    let result = rt
        .block_on(async { validate_feed(&feed, &client).await })
        .unwrap();

    assert_eq!(result.status, "invalid");
    assert!(!result.error.is_empty());
    mock.assert();
}

#[test]
fn test_redirect() {
    let rt = common::get_test_runtime();
    let mut server = mockito::Server::new();

    // Set up redirect
    let mock_redirect = server
        .mock("GET", "/old-feed.xml")
        .with_status(301)
        .with_header("Location", "/new-feed.xml")
        .create();

    // Set up final destination
    let mock_final = server
        .mock("GET", "/new-feed.xml")
        .with_status(200)
        .with_body(r#"<?xml version="1.0"?><rss version="2.0"><channel><title>Test</title></channel></rss>"#)
        .create();

    let feed = common::create_test_feed("Redirect Feed", &format!("{}/old-feed.xml", server.url()));
    let client = reqwest::Client::new();

    let result = rt
        .block_on(async { validate_feed(&feed, &client).await })
        .unwrap();

    assert_eq!(result.status, "valid");
    assert!(result.error.is_empty());
    mock_redirect.assert();
    mock_final.assert();
}

#[test]
fn test_different_encoding() {
    let rt = common::get_test_runtime();
    let mut server = mockito::Server::new();
    let mock = server
        .mock("GET", "/feed.xml")
        .with_status(200)
        .with_header("Content-Type", "application/xml; charset=ISO-8859-1")
        .with_body(vec![0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20]) // Basic XML header in ISO-8859-1
        .create();

    let feed = common::create_test_feed("Encoded Feed", &format!("{}/feed.xml", server.url()));
    let client = reqwest::Client::new();

    let result = rt
        .block_on(async { validate_feed(&feed, &client).await })
        .unwrap();

    assert_eq!(result.status, "invalid");
    mock.assert();
}

#[test]
fn test_rate_limit() {
    let rt = common::get_test_runtime();
    let mut server = mockito::Server::new();
    let mock = server
        .mock("GET", "/feed.xml")
        .with_status(429)
        .with_header("Retry-After", "60")
        .create();

    let feed = common::create_test_feed("Rate Limited Feed", &format!("{}/feed.xml", server.url()));
    let client = reqwest::Client::new();

    let result = rt
        .block_on(async { validate_feed(&feed, &client).await })
        .unwrap();

    assert_eq!(result.status, "error");
    assert!(result.error.contains("429"));
    mock.assert();
}

#[test]
fn test_compressed_response() {
    let rt = common::get_test_runtime();
    let mut server = mockito::Server::new();
    let mock = server
        .mock("GET", "/feed.xml")
        .with_status(200)
        .with_header("Content-Encoding", "gzip")
        .with_body([0x1f, 0x8b].to_vec()) // Basic gzip header
        .create();

    let feed = common::create_test_feed("Compressed Feed", &format!("{}/feed.xml", server.url()));
    let client = reqwest::Client::new();

    let result = rt
        .block_on(async { validate_feed(&feed, &client).await })
        .unwrap();

    assert_eq!(result.status, "invalid");
    mock.assert();
}

#[test]
fn test_validate_atom_feed() {
    let rt = common::get_test_runtime();
    let mut server = mockito::Server::new();
    let mock = server
        .mock("GET", "/feed.atom")
        .with_status(200)
        .with_header("content-type", "application/atom+xml")
        .with_body(r#"<?xml version="1.0"?><feed xmlns="http://www.w3.org/2005/Atom"><title>Test</title></feed>"#)
        .create();

    let feed = common::create_test_feed("Test Atom Feed", &format!("{}/feed.atom", server.url()));
    let client = reqwest::Client::new();

    let result = rt
        .block_on(async { validate_feed(&feed, &client).await })
        .unwrap();

    assert_eq!(result.status, "valid");
    assert!(result.error.is_empty());
    mock.assert();
}

#[test]
fn test_invalid_feed_format() {
    let rt = common::get_test_runtime();
    let mut server = mockito::Server::new();
    let mock = server
        .mock("GET", "/feed.xml")
        .with_status(200)
        .with_body(r#"<?xml version="1.0"?><html><body>Not a feed</body></html>"#)
        .create();

    let feed = common::create_test_feed("Invalid Feed Format", &format!("{}/feed.xml", server.url()));
    let client = reqwest::Client::new();

    let result = rt
        .block_on(async { validate_feed(&feed, &client).await })
        .unwrap();

    assert_eq!(result.status, "invalid");
    assert_eq!(result.error, "Document is not a valid RSS or Atom feed");
    mock.assert();
}
