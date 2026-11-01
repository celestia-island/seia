//! Tavily search API — AI-optimized search, returns clean content.
//!
//! Free tier: 1000 queries/month. Set `TAVILY_API_KEY` env var.

use anyhow::Result;
use serde::Deserialize;

use crate::{
    client::{EngineConfig, SearchOptions},
    engines::Engine,
    engines_impl::EngineOutput,
    result::{SearchItem, SearchMode},
};

/// Search path appended to the (configurable) base URL.
pub const SEARCH_PATH: &str = "/search";
const KEY_HINT: &str = "Get one at https://tavily.com";

/// Search with the Tavily Search API.
///
/// # Errors
///
/// Returns `Err` when `TAVILY_API_KEY` is missing, the HTTP request fails, or
/// the API returns a non-OK response.
pub async fn search(
    http: &reqwest::Client,
    query: &str,
    _opts: &SearchOptions,
    cfg: &EngineConfig,
) -> Result<EngineOutput> {
    let api_key = cfg.resolve_api_key(&Engine::Tavily, KEY_HINT)?;
    let endpoint = format!("{}{SEARCH_PATH}", cfg.resolve_base_url(&Engine::Tavily)?);

    let body = serde_json::json!({
        "api_key": api_key,
        "query": query,
        "max_results": 10,
        "include_answer": true,
    });

    let resp: TavilyResponse = http
        .post(&endpoint)
        // Newer Tavily accounts authenticate with a bearer token; the body
        // `api_key` is kept for API-compatible gateways that still expect it.
        .bearer_auth(&api_key)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    let items = resp
        .results
        .into_iter()
        .map(|r| SearchItem {
            title: r.title,
            url: r.url,
            snippet: Some(r.content),
            content: None,
            ..Default::default()
        })
        .collect();

    Ok((items, SearchMode::Api))
}

#[derive(Deserialize)]
struct TavilyResponse {
    #[serde(default)]
    results: Vec<TavilyItem>,
    #[serde(default)]
    #[allow(dead_code)]
    answer: Option<String>,
}

#[derive(Deserialize)]
struct TavilyItem {
    title: String,
    url: String,
    content: String,
}
