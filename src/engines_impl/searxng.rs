//! SearXNG — self-hosted meta-search engine.
//!
//! No API key needed. Set SEARXNG_URL env var or pass via SearchOptions.

use anyhow::{Result, anyhow};
use serde::Deserialize;

use crate::client::SearchOptions;
use crate::engines_impl::EngineOutput;
use crate::result::{SearchItem, SearchMode};

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
        let mut out = String::with_capacity(input.len() * 3);
        for byte in input.bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    out.push(byte as char);
                }
                b' ' => out.push_str("%20"),
                _ => out.push_str(&format!("%{:02X}", byte)),
            }
        }
        out
    }
}
