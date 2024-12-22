use opml_manager::opml::generate_opml;

#[test]
fn test_generate_empty_opml() {
    let feeds = vec![];
    let output = generate_opml(&feeds).unwrap();
    assert!(output.contains("<opml version=\"2.0\">"));
    assert!(output.contains("<body>"));
    assert!(output.contains("</body>"));
}
