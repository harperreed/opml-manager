use thiserror::Error;

#[derive(Error, Debug)]
pub enum OPMLError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("XML parsing error: {0}")]
    XMLParsing(#[from] roxmltree::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("No body tag found in OPML")]
    NoBodyTag,

    #[error("Invalid OPML structure: {0}")]
    InvalidStructure(String),

    #[error("Feed validation error: {0}")]
    ValidationError(String),

    #[error("URL parsing error: {0}")]
    UrlParsing(#[from] url::ParseError),

    #[error("Category nesting too deep: maximum depth is {0} levels")]
    CategoryNestingTooDeep(usize),

    #[error("Feed attribute error: {0}")]
    FeedAttributeError(String),
    
    #[error("Feed parsing error: {0}")]
    FeedParsingError(String),
    
    #[error("Response parsing error: {0}")]
    ResponseParsingError(String),
}

pub type Result<T> = std::result::Result<T, OPMLError>;
