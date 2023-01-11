/// This `soap` module defines a single "Soap" trait.
///
/// The trait can then be implemented for different search engines.
use std::fmt;

// Track source urls
use url::Url;

// Terminal colorization
use colored::Colorize;

// Re-export for easy use
pub use crate::args::{EngineOptions, Query};

/// Expected return format from a search query
pub struct SearchResult {
    /// The headline of the search result
    pub title: String,

    /// More verbose search result description
    pub description: String,

    /// Source URL of the search result
    pub source: Url,
}

pub type SearchResults = Vec<SearchResult>;

impl SearchResult {
    pub fn try_new<T: Into<String>>(
        title: T,
        description: T,
        source: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            title: title.into(),
            description: description.into(),
            source: Url::parse(source)?,
        })
    }
}

impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}",
            self.title.green(),
            format!("{}", self.source).italic().blue(),
            self.description.dimmed().white()
        )
    }
}

/// Core functionality that must be implemented in order to use a search engine.
pub trait Soap {
    fn search(&self, query: Query) -> Result<SearchResults, Box<dyn std::error::Error>>;
}
