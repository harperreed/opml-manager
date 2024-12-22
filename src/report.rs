use crate::Feed;
use chrono::Local;
use std::collections::{HashMap, HashSet};
use url::Url;

fn escape_special_chars(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
}

pub fn generate_summary(
    feeds: &[Feed],
) -> (
    HashSet<String>,
    Vec<&Feed>,
    HashSet<String>,
    HashMap<String, usize>,
) {
    let mut seen_urls = HashSet::new();
    let mut duplicates = Vec::new();
    let mut categories = HashSet::new();
    let mut domain_counter = HashMap::new();

    for feed in feeds {
        if !seen_urls.insert(feed.xml_url.to_lowercase()) {
            duplicates.push(feed);
        }

        if let Ok(url) = Url::parse(&feed.xml_url) {
            let domain = url.host_str().unwrap_or("unknown").to_string();
            *domain_counter.entry(domain).or_insert(0) += 1;
        }

        categories.extend(feed.category.iter().cloned());
    }

    (seen_urls, duplicates, categories, domain_counter)
}

// In src/report.rs, modify format_markdown_report:

pub fn format_markdown_report(
    feeds: &[Feed],
    seen_urls: &HashSet<String>,
    duplicates: &[&Feed],
    categories: &HashSet<String>,
    domain_counter: &HashMap<String, usize>,
) -> String {
    let mut report = String::new();
    report.push_str("# OPML Analysis Report\n\n");
    report.push_str(&format!(
        "Generated on: {}\n\n",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    ));

    // Summary section
    report.push_str("## Summary\n\n");
    report.push_str(&format!("Total Feeds: {}\n", feeds.len()));
    report.push_str(&format!("Unique Feeds: {}\n", seen_urls.len()));
    report.push_str(&format!("Categories Found: {}\n", categories.len()));
    report.push_str(&format!("Unique Domains: {}\n\n", domain_counter.len()));

    // Category breakdown
    if categories.is_empty() {
        report.push_str("No categories found\n\n");
    } else {
        report.push_str("## Categories\n\n");
        // Count how many feeds are in each category
        let mut category_counter: HashMap<String, usize> = HashMap::new();
        for feed in feeds {
            for category in &feed.category {
                *category_counter.entry(category.clone()).or_insert(0) += 1;
            }
        }

        report.push_str("| Category | Feed Count |\n");
        report.push_str("|----------|------------|\n");

        let mut sorted_categories: Vec<_> = category_counter.iter().collect();
        sorted_categories.sort_by(|a, b| b.1.cmp(a.1));

        for (category, count) in sorted_categories {
            report.push_str(&format!(
                "| {} | {} |\n",
                escape_special_chars(category),
                count
            ));
        }
        report.push_str("\n");
    }

    // Top domains
    report.push_str("## Top Domains\n\n");
    report.push_str("| Domain | Feed Count |\n");
    report.push_str("|--------|------------|\n");

    let mut sorted_domains: Vec<_> = domain_counter.iter().collect();
    sorted_domains.sort_by(|a, b| b.1.cmp(a.1));

    for (domain, count) in sorted_domains.iter().take(10) {
        report.push_str(&format!(
            "| {} | {} |\n",
            escape_special_chars(domain),
            count
        ));
    }
    report.push_str("\n");

    // Duplicate feeds
    if duplicates.is_empty() {
        report.push_str("No duplicate feeds found\n\n");
    } else {
        report.push_str("## Duplicate Feeds Found\n\n");
        for feed in duplicates {
            report.push_str(&format!("### {}\n\n", escape_special_chars(&feed.title)));
            report.push_str(&format!("- URL: {}\n", escape_special_chars(&feed.xml_url)));
            if !feed.category.is_empty() {
                let escaped_categories: Vec<_> = feed
                    .category
                    .iter()
                    .map(|c| escape_special_chars(c))
                    .collect();
                report.push_str(&format!(
                    "- Categories: {}\n",
                    escaped_categories.join(" > ")
                ));
            }
            report.push_str("\n");
        }
    }

    // List of all feeds
    report.push_str("## All Feeds\n\n");
    for feed in feeds {
        report.push_str(&format!("- {}\n", escape_special_chars(&feed.title)));
    }
    report.push_str("\n");

    report
}
