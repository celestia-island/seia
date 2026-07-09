//! seia — Universal Search Engine Abstraction
//!
//! Multi-backend web search library and CLI. Provides a unified interface to
//! query diverse search backends through their official HTTP APIs (or, where no
//! official API exists, lightweight HTML scraping).

pub mod client;
pub mod engines;
pub mod engines_impl;
pub mod extractor;
#[cfg(feature = "mcp")]
pub mod mcp;
pub mod result;
pub mod utils;

pub use client::{SearchClient, SearchOptions};
pub use engines::Engine;
pub use result::{SearchItem, SearchMode, SearchResult};

pub mod prelude {
    pub use crate::{Engine, SearchClient, SearchItem, SearchMode, SearchOptions, SearchResult};
}
