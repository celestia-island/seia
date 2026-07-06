//! `SearXNG` — self-hosted meta-search engine.
//!
//! No API key needed. Set `SEARXNG_URL` env var or pass via `SearchOptions`.

use anyhow::{Result, anyhow};
use serde::Deserialize;

use crate::{
    client::SearchOptions,
    engines_impl::EngineOutput,
    result::{SearchItem, SearchMode},
};

/// Search using a self-hosted `SearXNG` instance.
///
/// # Errors
///
/// Returns `Err` when `SEARXNG_URL` is missing, the HTTP request fails, or
/// the instance returns a non-OK response.
pub async fn search(
    http: &reqwest::Client,
    query: &str,
    opts: &SearchOptions,
) -> Result<EngineOutput> {
    let base_url = opts
        .searxng_url
        .clone()
        .or_else(|| std::env::var("SEARXNG_URL").ok())
        .ok_or_else(|| anyhow!("SEARXNG_URL not set. E.g. http://localhost:8080"))?;

    let url = format!(
        "{}/search?q={}&format=json",
        base_url.trim_end_matches('/'),
        urlencoding::encode(query)
    );

    let resp: SearxngResponse = http.get(&url).send().await?.json().await?;

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
struct SearxngResponse {
    results: Vec<SearxngItem>,
}

#[derive(Deserialize)]
struct SearxngItem {
    title: String,
    url: String,
    content: String,
}

mod urlencoding {
    pub fn encode(input: &str) -> String {
        crate::utils::urlencode_query(input)
    }
}
