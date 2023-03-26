//! Crag
//!
//! A crate for searching the web.
//!
//! # Overview
//! Crag provides a generic [`SOAP`] (search over any protocol: a wildly grandiose acronym for a trait that provides a `search` function) trait that can be implemented for various search engines,
//! and supports these engines out of the box:
//!     - Qwant
//!
//! Search engines consume a [`Query`], that specifies the search parameters,
//! and return a vector of structured [`SearchResult`]s.
//!
//! # Example
//! ```rust
//! use crag::{Query, Soap};
//! use crag::engines::Qwant;
//!
//! // Create a new query and search engine
//! let query = Query::from("rust programming language");
//! let engine = Qwant::default();
//!
//! // Perform the search using our query and engine
//! let results = engine.search(query).unwrap();
//!
//! // Print the results
//! for (idx, item) in results.iter().enumerate() {
//!    println!("\n{}. {item}\n", idx + 1);
//! }
//! ```
//!
//! # Command Line Interface
//!
//! Crag also provides a command line interface for searching the web.
//! You can install it for your platform with `cargo install crag`, and easily search the web from the command line:
//!
//! ```bash
//! $ crag -c 3 rust programming language
//!
//! 1. Rust Programming Language
//! https://www.rust-lang.org/
//! Rust Programming Language Rust A language empowering everyone to build reliable and efficient software. Get Started Version 1.68.0 Why Rust? Performance Rust is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.
//!
//! 2. Rust (programming language) - Wikipedia
//! https://en.wikipedia.org/wiki/Rust_(programming_language)
//! Rust is a multi-paradigm, high-level, general-purpose programming language.Rust emphasizes performance, type safety, and concurrency.Rust enforces memory safety—that is, that all references point to valid memory—without requiring the use of a garbage collector or reference counting present in other memory-safe languages. To simultaneously enforce memory safety and prevent concurrent data ...
//!
//! 3. The Rust Programming Language - The Rust Programming Language
//! https://doc.rust-lang.org/stable/book/
//! The Rust Programming Language by Steve Klabnik and Carol Nichols, with contributions from the Rust Community This version of the text assumes you’re using Rust 1.65 (released 2022-11-03) or later. See the “Installation” section of Chapter 1 to install or update Rust.
//! ```

#![deny(missing_docs)]
#![deny(unsafe_code)]

/// Defines the [`Soap`] trait, and supporting types
pub mod soap;
pub use soap::{EngineOptions, SearchResult, SearchResults, Soap};

/// Provides the [`Query`] type, which is used to specify search parameters
pub mod query;
pub use query::Query;

/// Implements [`Soap`] for various search providers
pub mod engines;
