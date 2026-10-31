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
    /// DeepSeek Search API (`POST /v1/tools/search`) — the standardized
    /// AI-search shape the normalized result model follows.
    Deepseek,
    /// Baidu AI Search (Qianfan, `POST /v2/ai_search/web_search`).
    Baidu,
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

/// Capability flags advertised by the `seia_search_v1` protocol
/// (`GET /v1/engines` on the HTTP service).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct EngineCapabilities {
    /// Engine can return full page content (not just a snippet).
    pub content: bool,
    /// Engine returns snippets/abstracts.
    pub snippets: bool,
    /// Engine exposes a freshness/recency filter.
    pub freshness: bool,
    /// Engine exposes a site/topical filter.
    pub site_filter: bool,
}

impl Engine {
    /// Protocol label for config-defined (custom) engines.
    pub const CUSTOM_PROTOCOL: &'static str = "seia_custom_v0";

    /// All built-in (non-custom) engines, in roster order.
    pub const BUILTIN: &'static [Self] = &[
        Self::Duckduckgo,
        Self::Wikipedia,
        Self::Searxng,
        Self::Tavily,
        Self::Bing,
        Self::Brave,
        Self::Zhipu,
        Self::Bocha,
        Self::Metaso,
        Self::Deepseek,
        Self::Baidu,
        Self::SemanticScholar,
        Self::OpenAlex,
        Self::Arxiv,
        Self::Core,
        Self::CrossRef,
        Self::Doaj,
        Self::PubMed,
    ];

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
            Self::Deepseek => "deepseek",
            Self::Baidu => "baidu",
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

    /// Wire protocol identifier used by the search service chain entries.
    ///
    /// `deepseek_search_v1` and `baidu_ai_search_v1` are the two standardized
    /// providers; everything else executes through seia's native adapters.
    #[must_use]
    pub const fn protocol(&self) -> &'static str {
        match self {
            Self::Duckduckgo => "duckduckgo_search_v1",
            Self::Wikipedia => "wikipedia_search_v1",
            Self::Searxng => "searxng_search_v1",
            Self::Tavily => "tavily_search_v1",
            Self::Bing => "bing_search_v1",
            Self::Brave => "brave_search_v1",
            Self::Zhipu => "zhipu_search_v1",
            Self::Bocha => "bocha_search_v1",
            Self::Metaso => "metaso_search_v1",
            Self::Deepseek => "deepseek_search_v1",
            Self::Baidu => "baidu_ai_search_v1",
            Self::SemanticScholar => "semantic_scholar_search_v1",
            Self::OpenAlex => "openalex_search_v1",
            Self::Arxiv => "arxiv_search_v1",
            Self::Core => "core_search_v1",
            Self::CrossRef => "crossref_search_v1",
            Self::Doaj => "doaj_search_v1",
            Self::PubMed => "pubmed_search_v1",
            Self::Custom(_) => Self::CUSTOM_PROTOCOL,
        }
    }

    /// Public default base URL of the engine's HTTP API (no trailing slash).
    /// `None` when the host is only known at runtime (e.g. SearXNG, custom).
    #[must_use]
    pub const fn default_base_url(&self) -> Option<&'static str> {
        match self {
            Self::Deepseek => Some("https://api.deepseek.com"),
            Self::Baidu => Some("https://qianfan.baidubce.com"),
            Self::Duckduckgo => Some("https://html.duckduckgo.com"),
            Self::Wikipedia => Some("https://en.wikipedia.org"),
            Self::Tavily => Some("https://api.tavily.com"),
            Self::Bing => Some("https://api.bing.microsoft.com/v7.0"),
            Self::Brave => Some("https://api.search.brave.com/res/v1"),
            Self::Zhipu => Some("https://open.bigmodel.cn/api/paas/v4"),
            Self::Bocha => Some("https://api.bochaai.com"),
            Self::Metaso => Some("https://metaso.cn/api/v1"),
            Self::SemanticScholar => Some("https://api.semanticscholar.org"),
            Self::OpenAlex => Some("https://api.openalex.org"),
            Self::Arxiv => Some("http://export.arxiv.org"),
            Self::Core => Some("https://api.core.ac.uk"),
            Self::CrossRef => Some("https://api.crossref.org"),
            Self::Doaj => Some("https://doaj.org"),
            Self::PubMed => Some("https://eutils.ncbi.nlm.nih.gov"),
            Self::Searxng | Self::Custom(_) => None,
        }
    }

    /// Advertised capabilities for the search protocol.
    #[must_use]
    pub const fn capabilities(&self) -> EngineCapabilities {
        match self {
            // DeepSeek supports `need_content`, `site:`/`-site:`/`lang:`/
            // `after:`/`before:` query operators.
            Self::Deepseek => EngineCapabilities {
                content: true,
                snippets: true,
                freshness: true,
                site_filter: true,
            },
            // Baidu returns per-reference `content` but we do not expose its
            // recency/site filters through protocol v0.
            Self::Baidu => EngineCapabilities {
                content: true,
                snippets: true,
                freshness: false,
                site_filter: false,
            },
            _ => EngineCapabilities {
                content: false,
                snippets: true,
                freshness: false,
                site_filter: false,
            },
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
            Self::Deepseek => Some("DEEPSEEK_API_KEY"),
            Self::Baidu => Some("BAIDU_API_KEY"),
            Self::Core => Some("CORE_API_KEY"),
            Self::CrossRef => None,
            Self::Doaj => None,
            Self::PubMed => None,
            Self::Custom(_) => None,
            _ => None,
        }
    }

    /// Environment variable overriding the engine base URL (if applicable).
    #[must_use]
    pub const fn base_url_env(&self) -> Option<&'static str> {
        match self {
            Self::Deepseek => Some("DEEPSEEK_SEARCH_BASE_URL"),
            Self::Baidu => Some("BAIDU_SEARCH_BASE_URL"),
            Self::Searxng => Some("SEARXNG_URL"),
            Self::Bing => Some("BING_SEARCH_ENDPOINT"),
            _ => None,
        }
    }

    /// Whether this engine needs an API key.
    #[must_use]
    pub const fn needs_key(&self) -> bool {
        self.api_key_env().is_some()
    }

    /// Parse engine name to variant. Returns `None` for unknown names
    /// (use `Engine::Custom(name)` instead).
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        let lower = name.to_ascii_lowercase();
        match lower.as_str() {
            "duckduckgo" | "ddg" => Some(Self::Duckduckgo),
            "tavily" => Some(Self::Tavily),
            "searxng" => Some(Self::Searxng),
            "wikipedia" | "wiki" => Some(Self::Wikipedia),
            "bing" => Some(Self::Bing),
            "brave" => Some(Self::Brave),
            "zhipu" => Some(Self::Zhipu),
            "bocha" => Some(Self::Bocha),
            "metaso" => Some(Self::Metaso),
            "deepseek" => Some(Self::Deepseek),
            "baidu" | "qianfan" => Some(Self::Baidu),
            "semantic-scholar" | "semanticscholar" | "s2" => Some(Self::SemanticScholar),
            "openalex" | "oa" => Some(Self::OpenAlex),
            "arxiv" => Some(Self::Arxiv),
            "core" => Some(Self::Core),
            "crossref" => Some(Self::CrossRef),
            "doaj" => Some(Self::Doaj),
            "pubmed" | "pm" => Some(Self::PubMed),
            _ => None,
        }
    }
}

impl std::fmt::Display for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_deepseek_and_baidu_aliases() {
        assert_eq!(Engine::from_name("DeepSeek"), Some(Engine::Deepseek));
        assert_eq!(Engine::from_name("baidu"), Some(Engine::Baidu));
        assert_eq!(Engine::from_name("qianfan"), Some(Engine::Baidu));
        assert_eq!(Engine::Deepseek.as_str(), "deepseek");
        assert_eq!(Engine::Baidu.as_str(), "baidu");
    }

    #[test]
    fn builtin_roster_is_self_consistent() {
        // The /v1/engines roster is derived from BUILTIN; make sure every
        // entry has a distinct name and round-trips through from_name().
        let mut seen = Vec::new();
        for engine in Engine::BUILTIN {
            let name = engine.as_str();
            assert!(
                Engine::from_name(name).is_some(),
                "engine {name:?} cannot be parsed back from its name"
            );
            assert!(
                !seen.contains(&name),
                "duplicate engine in roster: {name:?}"
            );
            seen.push(name);
            assert!(!name.is_empty());
        }
        assert!(seen.contains(&"deepseek"));
        assert!(seen.contains(&"baidu"));
    }

    #[test]
    fn new_engines_declare_protocol_and_key_env() {
        assert_eq!(Engine::Deepseek.protocol(), "deepseek_search_v1");
        assert_eq!(Engine::Baidu.protocol(), "baidu_ai_search_v1");
        assert_eq!(Engine::Tavily.protocol(), "tavily_search_v1");
        assert_eq!(Engine::Duckduckgo.protocol(), "duckduckgo_search_v1");
        assert_eq!(Engine::Deepseek.api_key_env(), Some("DEEPSEEK_API_KEY"));
        assert_eq!(Engine::Baidu.api_key_env(), Some("BAIDU_API_KEY"));
        assert_eq!(
            Engine::Deepseek.default_base_url(),
            Some("https://api.deepseek.com")
        );
        assert_eq!(
            Engine::Baidu.default_base_url(),
            Some("https://qianfan.baidubce.com")
        );
        assert!(Engine::Deepseek.capabilities().content);
        assert!(Engine::Deepseek.capabilities().site_filter);
        assert!(!Engine::Duckduckgo.capabilities().content);
    }
}
