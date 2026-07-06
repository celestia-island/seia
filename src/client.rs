//! Search client — unified entry point dispatching to engine backends.

use anyhow::{Result, anyhow};
use std::time::Instant;

use crate::{engines::Engine, result::SearchResult};

pub struct SearchClient {
    http: reqwest::Client,
}

impl Default for SearchClient {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchClient {
    /// Creates a new search client.
    ///
    /// # Panics
    ///
    /// Panics if the underlying HTTP client cannot be built (unrecoverable
    /// TLS / system configuration issue).
    #[must_use]
    pub fn new() -> Self {
        let http = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .expect("failed to build HTTP client");
        Self { http }
    }

    /// Create a client with a proxy (e.g. `<http://localhost:7890>`).
    /// Also respects `HTTPS_PROXY` / `HTTP_PROXY` env vars automatically.
    ///
    /// # Errors
    ///
    /// Returns `Err` on an invalid proxy URL.
    pub fn with_proxy(proxy_url: &str) -> Result<Self> {
        let proxy = reqwest::Proxy::all(proxy_url)?;
        let http = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .proxy(proxy)
            .timeout(std::time::Duration::from_secs(15))
            .build()?;
        Ok(Self { http })
    }

    /// Search with a specific engine. Returns ranked results.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the engine backend fails (network, API key missing,
    /// rate-limited, or zero results).
    pub async fn search(&self, query: &str, engine: Engine) -> Result<SearchResult> {
        self.search_with_options(query, engine, SearchOptions::default())
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
        let start = Instant::now();

        let (items, mode) = match engine {
            Engine::Duckduckgo => {
                crate::engines_impl::duckduckgo::search(&self.http, query, &opts).await?
            }
            Engine::Tavily => crate::engines_impl::tavily::search(&self.http, query, &opts).await?,
            Engine::Searxng => {
                crate::engines_impl::searxng::search(&self.http, query, &opts).await?
            }
            Engine::Wikipedia => {
                crate::engines_impl::wikipedia::search(&self.http, query, &opts).await?
            }
            Engine::Bing => crate::engines_impl::bing::search(&self.http, query, &opts).await?,
            Engine::Brave => crate::engines_impl::brave::search(&self.http, query, &opts).await?,
            Engine::Zhipu => crate::engines_impl::zhipu::search(&self.http, query, &opts).await?,
            Engine::Bocha => crate::engines_impl::bocha::search(&self.http, query, &opts).await?,
            Engine::Metaso => crate::engines_impl::metaso::search(&self.http, query, &opts).await?,
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

        Ok(SearchResult {
            engine: engine.as_str().to_string(),
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
        for &engine in engines {
            match self.search(query, engine).await {
                Ok(r) if !r.items.is_empty() => return Ok(r),
                Ok(_) => {
                    tracing::debug!(engine = %engine, "empty results, trying next");
                }
                Err(e) => {
                    tracing::debug!(engine = %engine, error = %e, "failed, trying next");
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
    /// Fetch full page content for each result (slower).
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
