use crate::error::Result;
use crate::Feed;
use chrono::{DateTime, Local};
use roxmltree::Node;
use std::collections::HashMap;

/// Parses an OPML file content into a vector of Feed structs
///
/// # Arguments
/// * `content` - The string content of the OPML file
///
/// # Returns
/// * `Result<Vec<Feed>>` - A vector of Feed structs if successful
pub fn parse_opml(content: &str) -> Result<Vec<Feed>> {
    let doc = roxmltree::Document::parse(content)?;
    let mut feeds = Vec::new();

    // Recursively process outline nodes
    const MAX_CATEGORY_DEPTH: usize = 100;

    fn process_outline(
        node: Node,
        current_categories: &[String],
        feeds: &mut Vec<Feed>,
    ) -> Result<()> {
        if current_categories.len() >= MAX_CATEGORY_DEPTH {
            return Err(crate::error::OPMLError::CategoryNestingTooDeep(
                MAX_CATEGORY_DEPTH,
            ));
        }
        for child in node.children() {
            if child.has_tag_name("outline") {
                let mut categories = current_categories.to_vec();

                match (
                    child.attribute("type"),
                    child.attribute("xmlUrl"),
                    child.attribute("text").or(child.attribute("title")),
                ) {
                    // Category node (no type or xmlUrl, but has text/title)
                    (None, None, Some(title)) => {
                        categories.push(title.to_string());
                        process_outline(child, &categories, feeds)?;
                    }
                    // Feed node (has xmlUrl or type="rss")
                    (type_attr, Some(xml_url), Some(title))
                        if type_attr.is_none() || type_attr == Some("rss") =>
                    {
                        // Normalize URL: lowercase, remove trailing slash, standardize to https
                        let mut normalized_url = xml_url.to_lowercase();
                        if normalized_url.ends_with('/') {
                            normalized_url.pop();
                        }
                        if normalized_url.starts_with("http://") {
                            normalized_url = normalized_url.replacen("http://", "https://", 1);
                        }

                        // Check if we've already seen this normalized URL
                        if !feeds.iter().any(|f| {
                            let mut existing = f.xml_url.to_lowercase();
                            if existing.ends_with('/') {
                                existing.pop();
                            }
                            if existing.starts_with("http://") {
                                existing = existing.replacen("http://", "https://", 1);
                            }
                            existing == normalized_url
                        }) {
                            feeds.push(Feed::new(
                                title.to_string(),
                                xml_url.to_string(),
                                child.attribute("htmlUrl").map(String::from),
                                categories.clone(),
                            ));
                        }
                    }
                    // Invalid or ignored node
                    _ => continue,
                }
            }
        }
        Ok(())
    }

    // Find and process the body tag
    let body = doc
        .root()
        .descendants()
        .find(|n| n.has_tag_name("body"))
        .ok_or(crate::error::OPMLError::NoBodyTag)?;

    process_outline(body, &[], &mut feeds)?;
    Ok(feeds)
}

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