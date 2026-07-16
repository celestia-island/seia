//! Engine enumeration and dispatch.

use serde::{Deserialize, Serialize};

use clap::ValueEnum;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Engine {
    Duckduckgo,
    Tavily,
    Searxng,
    Wikipedia,
    Bing,
    Brave,
    Zhipu,
    Bocha,
    Metaso,
    SemanticScholar,
    OpenAlex,
    Arxiv,
    Core,
    CrossRef,
    Doaj,
    PubMed,
    /// User-defined custom engine (name from config).
    #[value(skip)]
    Custom(String),
}

impl Engine {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Duckduckgo => "duckduckgo",
            Self::Tavily => "tavily",
            Self::Searxng => "searxng",
            Self::Wikipedia => "wikipedia",
            Self::Bing => "bing",
            Self::Brave => "brave",
            Self::Zhipu => "zhipu",
            Self::Bocha => "bocha",
            Self::Metaso => "metaso",
            Self::SemanticScholar => "semantic-scholar",
            Self::OpenAlex => "openalex",
            Self::Arxiv => "arxiv",
            Self::Core => "core",
            Self::CrossRef => "crossref",
            Self::Doaj => "doaj",
            Self::PubMed => "pubmed",
            Self::Custom(name) => name.as_str(),
        }
    }

    /// Environment variable holding the API key (if applicable).
    #[must_use]
    pub const fn api_key_env(&self) -> Option<&'static str> {
        match self {
            Self::Tavily => Some("TAVILY_API_KEY"),
            Self::Bing => Some("BING_SEARCH_API_KEY"),
            Self::Brave => Some("BRAVE_SEARCH_API_KEY"),
            Self::Zhipu => Some("ZHIPU_API_KEY"),
            Self::Bocha => Some("BOCHA_API_KEY"),
            Self::Metaso => Some("METASO_API_KEY"),
            Self::Core => Some("CORE_API_KEY"),
            Self::CrossRef => None,
            Self::Doaj => None,
            Self::PubMed => None,
            Self::Custom(_) => None,
            _ => None,
        }
    }

    /// Whether this engine needs an API key.
    #[must_use]
    pub const fn needs_key(&self) -> bool {
        self.api_key_env().is_some()
    }
}

impl std::fmt::Display for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
