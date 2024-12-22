use opml_manager::validation::validate_feed;
mod common;

#[test]
fn test_validate_valid_feed() {
    let rt = common::get_test_runtime();
    let mut server = mockito::Server::new();
    let mock = server
        .mock("GET", "/feed.xml")
        .with_status(200)
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
