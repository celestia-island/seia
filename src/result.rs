//! Search result types — shared across all engines.

use serde::{Deserialize, Serialize};

/// A complete search response from any engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub engine: String,
    pub query: String,
    pub mode: SearchMode,
    pub items: Vec<SearchItem>,
    pub elapsed_ms: u64,
}

/// A single search hit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchItem {
    pub title: String,
    pub url: String,
    /// Short snippet/abstract from the search engine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
    /// Full page content (only when `--fetch` is used).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// How the search was executed.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SearchMode {
    /// Called a search provider's HTTP API.
    Api,
    /// Scraped an HTML search results page (no official API).
    Scrape,
}
