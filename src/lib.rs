//! seia — Universal Search Engine Abstraction
//!
//! Multi-backend web search library and CLI. Supports both API-based and
//! browser-scraping search modes through a unified interface.

pub mod engines;
pub mod engines_impl;
pub mod client;
pub mod extractor;
pub mod result;

pub use client::SearchClient;
pub use result::{SearchResult, SearchItem, SearchMode};
pub use engines::Engine;

pub mod prelude {
    pub use crate::{SearchClient, Engine, SearchResult, SearchItem, SearchMode};
}
