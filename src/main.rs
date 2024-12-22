use clap::Parser;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use futures::future::join_all;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use reqwest::Client;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use opml_manager::cli::{Cli, Commands};
use opml_manager::opml::{generate_opml, parse_opml};
use opml_manager::report::{format_markdown_report, generate_summary};
use opml_manager::tui::events::Event as AppEvent;
use opml_manager::tui::{draw_ui, handle_events, TuiApp};
use opml_manager::validation::validate_feed;

struct Events {
    rx: mpsc::Receiver<AppEvent<KeyCode>>,
}

impl Events {
    fn new(tick_rate: Duration) -> Events {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || loop {
            if event::poll(tick_rate).unwrap() {
                if let Ok(evt) = event::read() {
                    if let Event::Key(key) = evt {
                        let _ = tx.send(AppEvent::Input(key.code));
                    }
                }
            }
        });

        Events { rx }
    }

    fn next(&self) -> Result<AppEvent<KeyCode>, mpsc::RecvError> {
        self.rx.recv()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { input_file } => {
            let content = fs::read_to_string(&input_file)?;
            let feeds = parse_opml(&content)?;

            let (seen_urls, duplicates, categories, _) = generate_summary(&feeds);

            println!("\n📊 OPML Analysis Report");
            println!("Total Feeds: {}", feeds.len());
            println!("Unique Feeds: {}", seen_urls.len());
            println!("Duplicates: {}", duplicates.len());
            println!("Total Categories: {}", categories.len());

            if !duplicates.is_empty() {
                println!("\n🔄 Duplicate Feeds:");
                for feed in duplicates {
                    println!("  - {} ({})", feed.title, feed.xml_url);
                    if !feed.category.is_empty() {
                        println!("    Categories: {}", feed.category.join(" > "));
                    }
                }
            }
        }

        Commands::Dedupe {
            input_file,
            output_file,
        } => {
            let content = fs::read_to_string(&input_file)?;
            let feeds = parse_opml(&content)?;

            // Store the original length
            let original_len = feeds.len();

            let mut unique_feeds = Vec::new();
            let mut seen = std::collections::HashSet::new();

            for feed in feeds {
                if seen.insert(feed.xml_url.to_lowercase()) {
                    unique_feeds.push(feed);
                }
            }

            let opml_content = generate_opml(&unique_feeds)?;
            fs::write(&output_file, opml_content)?;

            println!(
                "✅ Removed {} duplicates",
                original_len - unique_feeds.len()
            );
        }

        Commands::Validate {
            input_file,
            timeout,
        } => {
            let content = fs::read_to_string(&input_file)?;
            let feeds = parse_opml(&content)?;

            let client = Client::builder()
                .timeout(Duration::from_secs(timeout))
                .build()?;

            let mut tasks = Vec::new();
            for feed in &feeds {
                let feed_clone = feed.clone();
                let client_clone = client.clone();
                tasks.push(tokio::spawn(async move {
                    validate_feed(&feed_clone, &client_clone).await
                }));
            }

            let results = join_all(tasks).await;
            let mut validation_results = Vec::new();

            for result in results {
                if let Ok(Ok(validation)) = result {
                    validation_results.push(validation);
                }
            }

            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
            let report_path = Path::new(&input_file)
                .with_file_name(format!("validation_report_{}.md", timestamp));

            let mut report = String::new();
            report.push_str("# Feed Validation Report\n\n");
            report.push_str(&format!(
                "Generated on: {}\n\n",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            ));
            report.push_str(&format!("Source OPML: {}\n\n", input_file));

            let mut status_counts = std::collections::HashMap::new();
            for result in &validation_results {
                *status_counts.entry(&result.status).or_insert(0) += 1;
            }

            report.push_str("## Summary\n\n");
            report.push_str(&format!(
                "- Total feeds checked: {}\n",
                validation_results.len()
            ));
            for (status, count) in &status_counts {
                report.push_str(&format!("- {}: {}\n", status, count));
            }
            report.push_str("\n");

            for status in &["valid", "invalid", "error"] {
                let status_results: Vec<_> = validation_results
                    .iter()
                    .filter(|r| r.status == *status)
                    .collect();

                if !status_results.is_empty() {
                    let status_capitalized = status[0..1].to_uppercase() + &status[1..];
                    report.push_str(&format!("## {} Feeds\n\n", status_capitalized));
                    report.push_str("| Feed | URL | Error | Categories |\n");
                    report.push_str("|------|-----|-------|------------|\n");

                    for result in status_results {
                        let categories = result.categories.join(" > ");
                        let error = result.error.replace("|", "\\|");
                        report.push_str(&format!(
                            "| {} | {} | {} | {} |\n",
                            result.feed, result.url, error, categories
                        ));
                    }
                    report.push_str("\n");
                }
            }

            fs::write(&report_path, report)?;
            println!("\n✅ Validation report saved: {}", report_path.display());
        }

        Commands::Report {
            input_file,
            output_file,
            validate_feeds,
            timeout,
        } => {
            let content = fs::read_to_string(&input_file)?;
            let feeds = parse_opml(&content)?;

            let (seen_urls, duplicates, categories, domain_counter) = generate_summary(&feeds);

            let mut report = format_markdown_report(
                &feeds,
                &seen_urls,
                &duplicates,
                &categories,
                &domain_counter,
            );

            if validate_feeds {
                let client = Client::builder()
                    .timeout(Duration::from_secs(timeout))
                    .build()?;

                report.push_str("## Feed Validation Results\n\n");
                report.push_str("| Feed | Status | Error |\n");
                report.push_str("|------|--------|-------|\n");

                let mut tasks = Vec::new();
                for feed in &feeds {
                    let feed_clone = feed.clone();
                    let client_clone = client.clone();
                    tasks.push(tokio::spawn(async move {
                        validate_feed(&feed_clone, &client_clone).await
                    }));
                }

                let results = join_all(tasks).await;
                for result in results {
                    if let Ok(Ok(validation)) = result {
                        let error = validation.error.replace("|", "\\|");
                        report.push_str(&format!(
                            "| {} | {} | {} |\n",
                            validation.feed, validation.status, error
                        ));
                    }
                }
                report.push_str("\n");
            }

            fs::write(&output_file, report)?;
            println!("✅ Report generated: {}", output_file);
        }

        Commands::Interactive { input_file } => {
            enable_raw_mode()?;
            let stdout = std::io::stdout();
            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend)?;

            let mut app = TuiApp::new();
            app.load_feeds(parse_opml(&fs::read_to_string(input_file)?)?);

            let events = Events::new(Duration::from_millis(200));

            loop {
                terminal.draw(|f| draw_ui(f, &app))?;

                if let Ok(event) = events.next() {
                    if let AppEvent::Input(KeyCode::Char('q')) = event {
                        break;
                    }
                    handle_events(&mut app, event);
                }
            }

            disable_raw_mode()?;
            terminal.show_cursor()?;
        }
    }

    Ok(())
}
