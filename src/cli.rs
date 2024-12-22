use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Analyze OPML file for duplicates and potential issues
    Analyze {
        /// Input OPML file path
        input_file: String,
    },
    /// Remove duplicate feeds while preserving categories
    Dedupe {
        /// Input OPML file path
        input_file: String,
        /// Output OPML file path
        output_file: String,
    },
    /// Validate feeds and check for issues
    Validate {
        /// Input OPML file path
        input_file: String,
        /// Timeout in seconds for feed validation
        #[arg(long, default_value = "10")]
        timeout: u64,
    },
    /// Generate a detailed report about the OPML file
    Report {
        /// Input OPML file path
        input_file: String,
        /// Output report file path
        output_file: String,
        /// Include feed validation in report
        #[arg(long)]
        validate_feeds: bool,
        /// Timeout in seconds for feed validation
        #[arg(long, default_value = "10")]
        timeout: u64,
    },
    /// Launch the interactive TUI mode
    Interactive {
        /// Input OPML file path
        #[arg(default_value = "feeds.opml")]
        input_file: String,
    },
}
