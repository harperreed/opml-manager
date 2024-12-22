//! OPML Manager is a tool for managing OPML feed lists
//!
//! This library provides functionality for:
//! - Parsing and generating OPML files
//! - Analyzing feed lists for duplicates
//! - Validating feeds
//! - Generating reports about feed lists
//!
//! # Examples
//!
//! ```no_run
//! use opml_manager::{parse_opml, validate_feed};
//! use std::fs;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Read and parse OPML file
//! let content = fs::read_to_string("feeds.opml")?;
//! let feeds = parse_opml(&content)?;
//!
//! // Print feed information
//! for feed in feeds {
//!     println!("Feed: {} ({})", feed.title, feed.xml_url);
//! }
//! # Ok(())
//! # }
//! ```

pub mod cli;
pub mod error;
pub mod feed;
pub mod opml;
pub mod report;
pub mod validation;

pub use error::{OPMLError, Result};
pub use feed::Feed;
pub use opml::{generate_opml, parse_opml};
pub use validation::{validate_feed, ValidationResult};
