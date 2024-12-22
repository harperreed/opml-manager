use crate::error::{OPMLError, Result};
use crate::Feed;
use chrono::{DateTime, Local};
use std::collections::HashMap;
use xmlparser::{ElementEnd, Token, Tokenizer};

pub fn parse_opml(content: &str) -> Result<Vec<Feed>> {
    let mut feeds = Vec::new();
    let mut categories = Vec::new();
    let mut current_feed: Option<Feed> = None;

    let mut tokenizer = Tokenizer::from(content);
    while let Some(token) = tokenizer.next() {
        match token.map_err(|e| OPMLError::XMLParser(e.to_string()))? {
            Token::ElementStart { local, .. } => {
                if local.as_str() == "outline" {
                    current_feed = Some(Feed::new(
                        String::new(),
                        String::new(),
                        None,
                        categories.clone(),
                    ));
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
            Token::ElementEnd { end, .. } => match end {
                ElementEnd::Empty => {
                    if let Some(feed) = current_feed.take() {
                        if !feed.xml_url.is_empty() && !feed.title.is_empty() {
                            feeds.push(feed);
                        }
                    }
                }
                ElementEnd::Close(..) => {
                    if let Some(feed) = current_feed.take() {
                        if !feed.xml_url.is_empty() && !feed.title.is_empty() {
                            feeds.push(feed);
                        }
                    }
                }
                ElementEnd::Open => {}
            },
            _ => {}
        }
    }

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
