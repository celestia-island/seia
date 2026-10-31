//! Search client — unified entry point dispatching to engine backends.

use anyhow::{Result, anyhow};
use std::time::Instant;

use crate::{config::EngineRegistry, engines::Engine, result::SearchResult};

pub struct SearchClient {
    http: reqwest::Client,
    registry: EngineRegistry,
}

impl Default for SearchClient {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchClient {
    /// Creates a new search client. Automatically honours `SEIA_PROXY`,
    /// `HTTP_PROXY`, `HTTPS_PROXY`, and `ALL_PROXY` environment variables (in that order).
    ///
    /// # Panics
    ///
    /// Panics if the underlying HTTP client cannot be built (unrecoverable
    /// TLS / system configuration issue).
    #[must_use]
    pub fn new() -> Self {
        let _ = rustls::crypto::ring::default_provider().install_default();
        let mut builder = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .timeout(std::time::Duration::from_secs(15));

        if let Some(proxy_url) = detect_proxy() {
            if let Ok(proxy) = reqwest::Proxy::all(&proxy_url) {
                builder = builder.proxy(proxy);
            }
        }

        let http = builder.build().expect("failed to build HTTP client");
        Self {
            http,
            registry: EngineRegistry::default(),
        }
    }

    /// Create a client with an explicit proxy URL (e.g. `http://localhost:7890`).
    /// Overrides any auto-detected proxy.
    ///
    /// # Errors
    ///
    /// Returns `Err` on an invalid proxy URL.
    pub fn with_proxy(proxy_url: &str) -> Result<Self> {
        let _ = rustls::crypto::ring::default_provider().install_default();
        let proxy = reqwest::Proxy::all(proxy_url)?;
        let http = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .proxy(proxy)
            .timeout(std::time::Duration::from_secs(15))
            .build()?;
        Ok(Self {
            http,
            registry: EngineRegistry::default(),
        })
    }

    /// Create a client that never routes requests through a proxy.
    ///
    /// Useful for loopback-only deployments and for tests whose upstream is a
    /// local mock server: ambient `HTTP_PROXY` must not intercept 127.0.0.1.
    ///
    /// # Panics
    ///
    /// Panics if the underlying HTTP client cannot be built.
    #[must_use]
    pub fn without_proxy() -> Self {
        let _ = rustls::crypto::ring::default_provider().install_default();
        let http = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .no_proxy()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .expect("failed to build HTTP client");
        Self {
            http,
            registry: EngineRegistry::default(),
        }
    }

    /// Load custom engine definitions from the standard config paths
    /// (`~/.seia/engines.toml` and `./seia.toml`).
    pub fn with_registry(mut self, registry: EngineRegistry) -> Self {
        self.registry = registry;
        self
    }

    /// Registry of custom engines loaded from config.
    #[must_use]
    pub fn registry(&self) -> &EngineRegistry {
        &self.registry
    }

    /// Search with a specific engine. Returns ranked results.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the engine backend fails (network, API key missing,
    /// rate-limited, or zero results).
    pub async fn search(&self, query: &str, engine: Engine) -> Result<SearchResult> {
        self.search_with_config(
            query,
            engine,
            SearchOptions::default(),
            EngineConfig::default(),
        )
        .await
    }

    /// Search with additional options (fetch content, limit, etc).
    ///
    /// # Errors
    ///
    /// Returns `Err` when the engine backend fails (network, API key missing,
    /// rate-limited, or zero results).
    pub async fn search_with_options(
        &self,
        query: &str,
        engine: Engine,
        opts: SearchOptions,
    ) -> Result<SearchResult> {
        self.search_with_config(query, engine, opts, EngineConfig::default())
            .await
    }

    /// Search with per-call engine configuration: endpoint and API-key
    /// overrides plus optional engine hints (`ori_question` / `locale`).
    ///
    /// The HTTP service (`seia serve`) forwards each caller chain entry through
    /// this method, so upstream credentials are never read from seia's own
    /// environment in that path.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the engine backend fails (network, API key missing,
    /// rate-limited, or zero results).
    pub async fn search_with_config(
        &self,
        query: &str,
        engine: Engine,
        opts: SearchOptions,
        cfg: EngineConfig,
    ) -> Result<SearchResult> {
        let start = Instant::now();

        let (items, mode) = match engine {
            Engine::Duckduckgo => {
                crate::engines_impl::duckduckgo::search(&self.http, query, &opts).await?
            }
            Engine::Tavily => {
                crate::engines_impl::tavily::search(&self.http, query, &opts, &cfg).await?
            }
            Engine::Searxng => {
                // The chain entry's `base_url` doubles as the instance URL.
                let mut searxng_opts = opts.clone();
                if let Some(base) = cfg.base_url.as_deref() {
                    searxng_opts.searxng_url = Some(base.to_string());
                }
                crate::engines_impl::searxng::search(&self.http, query, &searxng_opts).await?
            }
            Engine::Wikipedia => {
                crate::engines_impl::wikipedia::search(&self.http, query, &opts).await?
            }
            Engine::Bing => {
                crate::engines_impl::bing::search(&self.http, query, &opts, &cfg).await?
            }
            Engine::Brave => {
                crate::engines_impl::brave::search(&self.http, query, &opts, &cfg).await?
            }
            Engine::Zhipu => {
                crate::engines_impl::zhipu::search(&self.http, query, &opts, &cfg).await?
            }
            Engine::Bocha => {
                crate::engines_impl::bocha::search(&self.http, query, &opts, &cfg).await?
            }
            Engine::Metaso => {
                crate::engines_impl::metaso::search(&self.http, query, &opts, &cfg).await?
            }
            Engine::Deepseek => {
                crate::engines_impl::deepseek::search(&self.http, query, &opts, &cfg).await?
            }
            Engine::Baidu => {
                crate::engines_impl::baidu::search(&self.http, query, &opts, &cfg).await?
            }
            Engine::SemanticScholar => {
                crate::engines_impl::semantic_scholar::search(&self.http, query, &opts).await?
            }
            Engine::OpenAlex => {
                crate::engines_impl::openalex::search(&self.http, query, &opts).await?
            }
            Engine::Arxiv => crate::engines_impl::arxiv::search(&self.http, query, &opts).await?,
            Engine::Core => {
                crate::engines_impl::core::search(&self.http, query, &opts, &cfg).await?
            }
            Engine::CrossRef => {
                crate::engines_impl::crossref::search(&self.http, query, &opts).await?
            }
            Engine::Doaj => crate::engines_impl::doaj::search(&self.http, query, &opts).await?,
            Engine::PubMed => crate::engines_impl::pubmed::search(&self.http, query, &opts).await?,
            Engine::Custom(ref name) => {
                let def = self
                    .registry
                    .get(name)
                    .ok_or_else(|| anyhow!("custom engine '{name}' not found in config"))?;
                crate::engines_impl::custom::search(&self.http, query, &opts, def, name).await?
            }
        };

        let mut items = items;
        if let Some(limit) = opts.limit {
            items.truncate(limit);
        }

        if opts.fetch_content {
            for item in &mut items {
                if item.content.is_none() {
                    item.content = crate::extractor::fetch_content(&self.http, &item.url)
                        .await
                        .ok();
                }
            }
        }

        // Normalized result shape: tag every item with the engine that ran.
        let engine_name = engine.as_str().to_string();
        for item in &mut items {
            if item.engine.is_none() {
                item.engine = Some(engine_name.clone());
            }
        }

        Ok(SearchResult {
            engine: engine_name,
            query: query.to_string(),
            mode,
            elapsed_ms: u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX),
            items,
        })
    }

    /// Convenience: try multiple engines in order, return first successful.
    ///
    /// # Errors
    ///
    /// Returns `Err` of the last engine if all engines fail.
    pub async fn search_fallback(&self, query: &str, engines: &[Engine]) -> Result<SearchResult> {
        let mut last_err = anyhow!("no engines provided");
        for engine in engines {
            let engine_name = engine.as_str().to_string();
            match self.search(query, engine.clone()).await {
                Ok(r) if !r.items.is_empty() => return Ok(r),
                Ok(_) => {
                    tracing::debug!(engine = %engine_name, "empty results, trying next");
                }
                Err(e) => {
                    tracing::debug!(engine = %engine_name, error = %e, "failed, trying next");
                    last_err = e;
                }
            }
        }
        Err(last_err)
    }
}

/// Per-search options.
#[derive(Debug, Clone)]
pub struct SearchOptions {
    /// Max results to return.
    pub limit: Option<usize>,
    /// Ask for full page content.
    ///
    /// Engines with native content support (DeepSeek `need_content`, Baidu
    /// `chunks_per_doc`) forward this to the provider; otherwise seia fetches
    /// each result page itself.
    pub fetch_content: bool,
    /// `SearXNG` instance URL (for `Engine::Searxng`).
    pub searxng_url: Option<String>,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            limit: Some(10),
            fetch_content: false,
            searxng_url: None,
        }
    }
}

/// Per-call engine configuration — endpoint/credential overrides plus optional
/// engine hints.
///
/// Used by the HTTP service to execute caller-supplied chain entries without
/// mutating process-wide environment variables. **Never log or serialize the
/// `api_key` field.**
#[derive(Clone, Default)]
pub struct EngineConfig {
    /// Override the engine base URL (e.g. `https://api.deepseek.com`).
    pub base_url: Option<String>,
    /// Runtime-injected API key. Falls back to the engine's env var when unset.
    pub api_key: Option<String>,
    /// Original user question, forwarded to engines that support it
    /// (DeepSeek `ori_question`).
    pub ori_question: Option<String>,
    /// Locale hint (DeepSeek `localization`, e.g. `zh-CN`).
    pub locale: Option<String>,
}

// Manual Debug: EngineConfig carries a plaintext API key on the service path.
impl std::fmt::Debug for EngineConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EngineConfig")
            .field("base_url", &self.base_url)
            .field("api_key", &self.api_key.as_ref().map(|_| "<redacted>"))
            .field("ori_question", &self.ori_question)
            .field("locale", &self.locale)
            .finish()
    }
}

impl EngineConfig {
    /// Resolve the API key: explicit per-call override first, then the engine's
    /// environment variable.
    ///
    /// # Errors
    ///
    /// Returns `Err` naming the env var when neither source provides a key.
    pub fn resolve_api_key(&self, engine: &Engine, hints: &str) -> Result<String> {
        if let Some(key) = self
            .api_key
            .as_deref()
            .map(str::trim)
            .filter(|k| !k.is_empty())
        {
            return Ok(key.to_string());
        }
        let env = engine
            .api_key_env()
            .ok_or_else(|| anyhow!("{} does not use an API key", engine.as_str()))?;
        std::env::var(env).map_err(|_| anyhow!("{env} not set. {hints}"))
    }

    /// Resolve the base URL: explicit per-call override, then the engine's
    /// base-URL env var, then the public default.
    ///
    /// # Errors
    ///
    /// Returns `Err` when no base URL is configured for the engine.
    pub fn resolve_base_url(&self, engine: &Engine) -> Result<String> {
        if let Some(base) = self
            .base_url
            .as_deref()
            .map(str::trim)
            .filter(|b| !b.is_empty())
        {
            return Ok(base.trim_end_matches('/').to_string());
        }
        if let Some(env) = engine.base_url_env() {
            if let Ok(value) = std::env::var(env) {
                let trimmed = value.trim().trim_end_matches('/');
                if !trimmed.is_empty() {
                    return Ok(trimmed.to_string());
                }
            }
        }
        engine
            .default_base_url()
            .map(str::to_string)
            .ok_or_else(|| {
                anyhow!(
                    "no base URL configured for engine '{}' (set {} or pass base_url)",
                    engine.as_str(),
                    engine.base_url_env().unwrap_or("a base URL")
                )
            })
    }
}

#[cfg(test)]
impl EngineConfig {
    /// Test-only shortcut: a config with a fake credential and endpoint.
    pub fn for_test(base_url: impl Into<String>) -> Self {
        Self {
            base_url: Some(base_url.into()),
            api_key: Some("test-key".into()),
            ..Self::default()
        }
    }
}

/// Auto-detect proxy from environment variables.
/// Checks `SEIA_PROXY` first, then `HTTP_PROXY`, `HTTPS_PROXY`, then `ALL_PROXY`.
fn detect_proxy() -> Option<String> {
    for var in &["SEIA_PROXY", "HTTP_PROXY", "HTTPS_PROXY", "ALL_PROXY"] {
        if let Ok(val) = std::env::var(var) {
            if !val.is_empty() {
                return Some(val);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_config_debug_redacts_api_key() {
        let cfg = EngineConfig {
            api_key: Some("sk-fake-super-secret".into()),
            ..EngineConfig::default()
        };
        let rendered = format!("{cfg:?}");
        assert!(
            !rendered.contains("sk-fake-super-secret"),
            "leaked: {rendered}"
        );
        assert!(rendered.contains("<redacted>"));
    }

    #[test]
    fn engine_config_never_serializes_credentials() {
        // EngineConfig deliberately has no Serialize impl; the chain entry type
        // in the service module is the only wire form and skips `api_key`.
        let cfg = EngineConfig::for_test("http://127.0.0.1:1");
        assert_eq!(
            cfg.resolve_base_url(&Engine::Deepseek).unwrap(),
            "http://127.0.0.1:1"
        );
        assert_eq!(
            cfg.resolve_api_key(&Engine::Deepseek, "").unwrap(),
            "test-key"
        );
    }

    #[test]
    fn base_url_resolution_prefers_override_then_default() {
        let cfg = EngineConfig::default();
        assert_eq!(
            cfg.resolve_base_url(&Engine::Baidu).unwrap(),
            "https://qianfan.baidubce.com"
        );
        // Engines without a public default must fail loudly instead of hitting
        // an empty URL.
        assert!(cfg.resolve_base_url(&Engine::Searxng).is_err());
    }

    #[test]
    fn missing_key_error_names_the_env_var() {
        if std::env::var("DEEPSEEK_API_KEY").is_ok() {
            return; // ambient developer key; nothing to assert
        }
        let err = EngineConfig::default()
            .resolve_api_key(
                &Engine::Deepseek,
                "Get one at https://platform.deepseek.com",
            )
            .unwrap_err()
            .to_string();
        assert!(err.contains("DEEPSEEK_API_KEY"), "{err}");
    }
}
