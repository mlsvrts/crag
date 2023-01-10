/// This `soap` module defines a single "Soap" trait.
///
/// The trait can then be implemented for different search engines.

use dyn_clone::DynClone;

/// Re-export for easy use
pub use crate::args::Query;

/// Expected return format from a search query
pub type SearchResults = String;

/// Core functionality that must be implemented in order to use a search engine.
pub trait Soap: DynClone + Send + Sync{
    fn search(&self, query: Query) -> Result<SearchResults, Box<dyn std::error::Error>>;
}
