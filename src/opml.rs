use crate::error::Result;
use crate::Feed;
use chrono::{DateTime, Local};
use std::collections::HashMap;
use xmlparser::{Tokenizer, Token};

/// Parses an OPML file content into a vector of Feed structs
///
/// # Arguments
/// * `content` - The string content of the OPML file
///
/// # Returns
/// * `Result<Vec<Feed>>` - A vector of Feed structs if successful
pub fn parse_opml(content: &str) -> Result<Vec<Feed>> {
    let mut feeds = Vec::new();
    let mut categories = Vec::new();
    let mut current_feed: Option<Feed> = None;

    let mut tokenizer = Tokenizer::from(content);
    while let Some(token) = tokenizer.next() {
        match token? {
            Token::ElementStart { local, .. } => {
                if local.as_str() == "outline" {
                    current_feed = Some(Feed::new(String::new(), String::new(), None, categories.clone()));
                }
            }
            Token::Attribute { local, value, .. } => {
                if let Some(feed) = current_feed.as_mut() {
                    match local.as_str() {
                        "type" => {
                            if value.as_str() != "rss" {
                                current_feed = None;
                            }
                        }
                        "xmlUrl" => feed.xml_url = value.to_string(),
                        "text" | "title" => feed.title = value.to_string(),
                        "htmlUrl" => feed.html_url = Some(value.to_string()),
                        _ => {}
                    }
                }
            }
            Token::ElementEnd { end, .. } => {
                if end.is_empty() {
                    if let Some(feed) = current_feed.take() {
                        if !feed.xml_url.is_empty() && !feed.title.is_empty() {
                            feeds.push(feed);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(feeds)
}

/// Generates OPML content from a vector of feeds
///
/// # Arguments
/// * `feeds` - Vector of Feed structs to include in the OPML
///
/// # Returns
/// * `Result<String>` - The generated OPML content if successful
pub fn generate_opml(feeds: &[Feed]) -> Result<String> {
    let mut output = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<opml version="2.0">
  <head>
    <title>Feed List</title>
    <dateCreated>"#,
    );

    // Add timestamp
    let now: DateTime<Local> = Local::now();
    output.push_str(&now.format("%a, %d %b %Y %H:%M:%S %z").to_string());
    output.push_str("</dateCreated>\n  </head>\n  <body>\n");

    // Group feeds by category path
    let mut category_map: HashMap<String, Vec<&Feed>> = HashMap::new();
    for feed in feeds {
        let category_path = feed.category.join("/");
        category_map.entry(category_path).or_default().push(feed);
    }

    // Helper function to write feed entries
    fn write_feeds(output: &mut String, feeds: &[&Feed], indent: usize) {
        let indent_str = " ".repeat(indent);
        for feed in feeds {
            output.push_str(&format!(
                r#"{}<outline type="rss" text="{}" title="{}" xmlUrl="{}"{}/>\n"#,
                indent_str,
                feed.title,
                feed.title,
                feed.xml_url,
                feed.html_url
                    .as_ref()
                    .map(|url| format!(" htmlUrl=\"{}\"", url))
                    .unwrap_or_default()
            ));
        }
    }

    // Write uncategorized feeds first
    if let Some(uncategorized) = category_map.get("") {
        write_feeds(&mut output, uncategorized, 4);
    }

    // Write categorized feeds
    for (category_path, feeds) in category_map.iter().filter(|(k, _)| !k.is_empty()) {
        let categories: Vec<&str> = category_path.split('/').collect();
        let mut current_indent = 4;

        // Open category tags
        for category in &categories {
            output.push_str(&format!(
                "{}<outline text=\"{}\">\n",
                " ".repeat(current_indent),
                category
            ));
            current_indent += 2;
        }

        // Write feeds in this category
        write_feeds(&mut output, feeds, current_indent);

        // Close category tags
        for _ in &categories {
            current_indent -= 2;
            output.push_str(&format!("{}</outline>\n", " ".repeat(current_indent)));
        }
    }

    output.push_str("  </body>\n</opml>");
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_opml() {
        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
        <opml version="2.0">
            <head><title>Test</title></head>
            <body>
                <outline type="rss" text="Test Feed" xmlUrl="http://example.com/feed.xml"/>
            </body>
        </opml>"#;

        let feeds = parse_opml(content).unwrap();
        assert_eq!(feeds.len(), 1);
        assert_eq!(feeds[0].title, "Test Feed");
        assert_eq!(feeds[0].xml_url, "http://example.com/feed.xml");
        assert!(feeds[0].category.is_empty());
    }

    #[test]
    fn test_parse_categorized_feeds() {
        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
        <opml version="2.0">
            <head><title>Test</title></head>
            <body>
                <outline text="Category">
                    <outline type="rss" text="Test Feed" xmlUrl="http://example.com/feed.xml"/>
                </outline>
            </body>
        </opml>"#;

        let feeds = parse_opml(content).unwrap();
        assert_eq!(feeds.len(), 1);
        assert_eq!(feeds[0].category, vec!["Category"]);
    }
}
