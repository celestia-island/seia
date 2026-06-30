//! Engine enumeration and dispatch.

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Engine {
    /// DuckDuckGo HTML scraping (free, no key).
    Duckduckgo,
    /// Tavily API (AI-optimized, free tier 1K/month).
    Tavily,
    /// SearXNG self-hosted meta-search.
    Searxng,
    /// Wikipedia API (free, unlimited, academic knowledge).
    Wikipedia,
    /// Bing Web Search API (paid, high quality).
    Bing,
    /// Brave Search API (free tier available).
    Brave,
}

impl Engine {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Duckduckgo => "duckduckgo",
            Self::Tavily => "tavily",
            Self::Searxng => "searxng",
            Self::Wikipedia => "wikipedia",
            Self::Bing => "bing",
            Self::Brave => "brave",
        }
    }

    /// Environment variable holding the API key (if applicable).
    pub fn api_key_env(self) -> Option<&'static str> {
        match self {
            Self::Tavily => Some("TAVILY_API_KEY"),
            Self::Bing => Some("BING_SEARCH_API_KEY"),
            Self::Brave => Some("BRAVE_SEARCH_API_KEY"),
            _ => None,
        }
    }

    /// Whether this engine needs an API key.
    pub fn needs_key(self) -> bool {
        self.api_key_env().is_some()
    }
}

impl std::fmt::Display for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
