//! Search client — unified entry point dispatching to engine backends.

use std::time::Instant;

use anyhow::{Result, anyhow};

use crate::engines::Engine;
use crate::result::{SearchItem, SearchMode, SearchResult};

pub struct SearchClient {
    http: reqwest::Client,
}

impl Default for SearchClient {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchClient {
    pub fn new() -> Self {
        let http = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .expect("failed to build HTTP client");
        Self { http }
    }

    /// Search with a specific engine. Returns ranked results.
    pub async fn search(&self, query: &str, engine: Engine) -> Result<SearchResult> {
        self.search_with_options(query, engine, SearchOptions::default()).await
    }

    /// Search with additional options (fetch content, limit, etc).
    pub async fn search_with_options(
        &self,
        query: &str,
        engine: Engine,
        opts: SearchOptions,
    ) -> Result<SearchResult> {
        let start = Instant::now();

        let (items, _mode) = match engine {
            Engine::Duckduckgo => {
                crate::engines_impl::duckduckgo::search(&self.http, query, &opts).await?
            }
            Engine::Tavily => {
                crate::engines_impl::tavily::search(&self.http, query, &opts).await?
            }
            Engine::Searxng => {
                crate::engines_impl::searxng::search(&self.http, query, &opts).await?
            }
            Engine::Wikipedia => {
                crate::engines_impl::wikipedia::search(&self.http, query, &opts).await?
            }
            Engine::Bing => Err(anyhow!("Bing engine not yet implemented. Set BING_SEARCH_API_KEY."))?,
            Engine::Brave => Err(anyhow!("Brave engine not yet implemented. Set BRAVE_SEARCH_API_KEY."))?,
        };

        let mut items = items;
        if let Some(limit) = opts.limit {
            items.truncate(limit);
        }

        if opts.fetch_content {
            for item in items.iter_mut() {
                if item.content.is_none() {
                    item.content = crate::extractor::fetch_content(
                        &self.http, &item.url,
                    ).await.ok();
                }
            }
        }

        Ok(SearchResult {
            engine: engine.as_str().to_string(),
            query: query.to_string(),
            elapsed_ms: start.elapsed().as_millis() as u64,
            items,
        })
    }

    /// Convenience: try multiple engines in order, return first successful.
    pub async fn search_fallback(
        &self,
        query: &str,
        engines: &[Engine],
    ) -> Result<SearchResult> {
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
    /// SearXNG instance URL (for Engine::Searxng).
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


