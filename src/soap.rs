/// This `soap` module defines a single [`Soap`] trait.
///
/// The trait can then be implemented for different search engines.
///
/// Supporting types are also defined here, such as [`SearchResult`].
// Track source urls
use url::Url;

// re-export the `Query` type
pub use crate::Query;

/// Provides engine customization options
#[derive(Default, Debug)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct EngineOptions {
    /// Do not attempt to filter ads from returned responses
    ///
    /// Note that filtering ads may impact the number of results returned
    #[cfg_attr(feature = "cli", arg(long))]
    pub keep_ads: bool,

    /// Do not attempt to filter video results from returned responses
    ///
    /// Note that keeping videos may result in additional results returned
    #[cfg_attr(feature = "cli", arg(long))]
    pub keep_videos: bool,

    /// Do not attempt to filter image results from returned responses
    ///
    /// Note filtering images may impact the number of results returned
    #[cfg_attr(feature = "cli", arg(long))]
    pub keep_images: bool,
}

/// Expected return format from a search query
#[derive(Debug)]
pub struct SearchResult {
    /// The headline of the search result
    pub title: String,

    /// More verbose search result description
    pub description: String,

    /// Source URL of the search result
    pub source: Url,
}

/// Type alias for a vector of search results
pub type SearchResults = Vec<SearchResult>;

impl SearchResult {
    /// Build a new SearchResult from a title, description, and source URL
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

/// Provide fmt::Display for SearchResult
///
/// Prints the format `title (source): "description"`
impl std::fmt::Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Limit description to 40 characters
        let desc_slice = &self.description[..std::cmp::min(self.description.len(), 40)];

        write!(f, "{} ({}): \"{}...\"", self.title, self.source, desc_slice)
    }
}

/// Core functionality that must be implemented in order to use a search engine.
pub trait Soap {
    /// Set the engine options
    fn configure(&mut self, options: EngineOptions) -> Result<(), Box<dyn std::error::Error>>;

    /// Search for a query and return a vector of search results
    fn search(&self, query: Query) -> Result<SearchResults, Box<dyn std::error::Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_result_create() {
        let result = SearchResult::try_new("title", "description", "https://example.com");
        assert!(result.is_ok());
    }

    #[test]
    fn test_search_result_display() {
        let result = SearchResult::try_new("title", "description", "https://example.com").unwrap();
        assert_eq!(
            result.to_string(),
            "title (https://example.com/): \"description...\""
        );
    }

    #[test]
    fn test_search_result_display_desc_is_truncated() {
        let desc = "description description description description";
        let result = SearchResult::try_new("title", desc, "https://example.com").unwrap();
        assert_eq!(
            result.to_string(),
            format!("title (https://example.com/): \"{}...\"", &desc[..40])
        );
    }
}
