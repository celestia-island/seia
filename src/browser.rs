//! Browser-mode search via tairitsu HTTP debug API.
//!
//! Connects to a running tairitsu debug server to drive a headless browser
//! for search engines that block non-browser requests (Google, Baidu, etc.).

use anyhow::{Result, anyhow};
use std::time::Duration;

use crate::profiles::SearchProfile;
use crate::result::{SearchItem, SearchResult};

/// Client for a running tairitsu debug server.
pub struct BrowserClient {
    http: reqwest::Client,
    endpoint: String,
}

impl BrowserClient {
    /// Connect to a tairitsu debug server (e.g. "http://127.0.0.1:3001").
    pub fn new(endpoint: &str) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(35))
            .build()
            .expect("failed to build HTTP client");
        Self {
            http,
            endpoint: endpoint.trim_end_matches('/').to_string(),
        }
    }

    /// Check if the browser is connected and ready.
    pub async fn health(&self) -> Result<bool> {
        let resp: serde_json::Value = self
            .http
            .get(format!("{}/health", self.endpoint))
            .send()
            .await?
            .json()
            .await?;

        Ok(resp.get("ok").and_then(|v| v.as_bool()).unwrap_or(false))
    }

    /// Search via browser: navigate → wait → extract results.
    pub async fn search(&self, query: &str, profile: &SearchProfile) -> Result<SearchResult> {
        let start = std::time::Instant::now();

        // Step 1: Navigate to search page
        let url = (profile.search_url)(query);
        self.navigate(&url).await?;

        // Step 2: Wait for results to render
        self.wait_for_selector(profile.wait_selector, 10_000)
            .await?;

        // Step 3: Extract results via JS
        let items = self.extract_results(profile).await?;

        Ok(SearchResult {
            engine: profile.name.to_string(),
            query: query.to_string(),
            items,
            elapsed_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Navigate to a URL.
    async fn navigate(&self, url: &str) -> Result<()> {
        let resp = self
            .http
            .post(format!("{}/navigate", self.endpoint))
            .json(&serde_json::json!({ "url": url }))
            .send()
            .await?;

        let body: serde_json::Value = resp.json().await?;
        if body.get("ok").and_then(|v| v.as_bool()) == Some(true) {
            Ok(())
        } else {
            Err(anyhow!(
                "navigate failed: {}",
                body.get("error")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
            ))
        }
    }

    /// Wait for a CSS selector to appear in the DOM.
    async fn wait_for_selector(&self, selector: &str, timeout_ms: u64) -> Result<bool> {
        let resp = self
            .http
            .post(format!("{}/wait-for-selector", self.endpoint))
            .json(&serde_json::json!({
                "selector": selector,
                "timeout_ms": timeout_ms
            }))
            .send()
            .await?;

        let body: serde_json::Value = resp.json().await?;
        let found = body
            .get("data")
            .and_then(|d| d.get("found"))
            .and_then(|f| f.as_bool())
            .unwrap_or(false);

        Ok(found)
    }

    /// Execute JS to extract search results from the rendered page.
    async fn extract_results(&self, profile: &SearchProfile) -> Result<Vec<SearchItem>> {
        let snippet_sel = profile.snippet_selector.unwrap_or("");
        let js = format!(
            r#"
            (() => {{
                const containers = document.querySelectorAll('{results}');
                const items = [];
                for (const c of containers) {{
                    const titleEl = c.querySelector('{title}');
                    const linkEl = c.querySelector('{link}');
                    if (!titleEl || !linkEl) continue;
                    const title = titleEl.textContent.trim();
                    let url = linkEl.href || linkEl.getAttribute('href') || '';
                    if (!title || !url) continue;
                    if (!url.startsWith('http')) continue;
                    let snippet = '';
                    const snippetEl = c.querySelector('{snippet}');
                    if (snippetEl) snippet = snippetEl.textContent.trim();
                    items.push({{ title, url, snippet }});
                    if (items.length >= 20) break;
                }}
                return items;
            }})()
            "#,
            results = profile.result_selector,
            title = profile.title_selector,
            link = profile.link_selector,
            snippet = snippet_sel,
        );

        let resp = self
            .http
            .post(format!("{}/evaluate", self.endpoint))
            .json(&serde_json::json!({ "expression": js }))
            .send()
            .await?;

        let body: serde_json::Value = resp.json().await?;

        let raw_items = body
            .get("data")
            .and_then(|d| d.get("result"))
            .and_then(|r| r.as_array())
            .ok_or_else(|| anyhow!("no results in evaluate response"))?;

        let items = raw_items
            .iter()
            .map(|item| SearchItem {
                title: item
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                url: item
                    .get("url")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                snippet: item
                    .get("snippet")
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.is_empty())
                    .map(String::from),
                content: None,
            })
            .filter(|item| !item.title.is_empty() && !item.url.is_empty())
            .collect();

        Ok(items)
    }
}
