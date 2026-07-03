//! Tavily search API — AI-optimized search, returns clean content.
//!
//! Free tier: 1000 queries/month. Set `TAVILY_API_KEY` env var.

use anyhow::{Result, anyhow};
use serde::Deserialize;

use crate::{
    client::SearchOptions,
    engines_impl::EngineOutput,
    result::{SearchItem, SearchMode},
};

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
) -> Result<EngineOutput> {
    let api_key = std::env::var("TAVILY_API_KEY")
        .map_err(|_| anyhow!("TAVILY_API_KEY not set. Get one at https://tavily.com"))?;

    let body = serde_json::json!({
        "api_key": api_key,
        "query": query,
        "max_results": 10,
        "include_answer": true,
    });

    let resp: TavilyResponse = http
        .post("https://api.tavily.com/search")
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
