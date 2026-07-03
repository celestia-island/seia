//! 博查 (Bocha) Web Search API — domestic Chinese web search for AI.
//!
//! An LLM-oriented search engine returning clean web results with summaries.
//! Covers web pages, news, encyclopedia, video. Free tier available.
//!
//! Set `BOCHA_API_KEY`. Get one at <https://open.bochaai.com/>.
//!
//! API reference: <https://open.bochaai.com/docs> (POST /v1/web-search)

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::{
    client::SearchOptions,
    engines_impl::EngineOutput,
    result::{SearchItem, SearchMode},
};

const ENDPOINT: &str = "https://api.bochaai.com/v1/web-search";

/// Search with the 博查 (Bocha) Web Search API.
///
/// # Errors
///
/// Returns `Err` when `BOCHA_API_KEY` is missing, the HTTP request fails, or
/// the API returns a non-2xx status.
pub async fn search(
    http: &reqwest::Client,
    query: &str,
    opts: &SearchOptions,
) -> Result<EngineOutput> {
    let api_key = std::env::var("BOCHA_API_KEY")
        .map_err(|_| anyhow!("BOCHA_API_KEY not set. Get one at https://open.bochaai.com/"))?;

    let count = opts.limit.unwrap_or(10).clamp(1, 50);

    let body = BochaRequest {
        query: query.to_string(),
        count,
        // Ask Bocha to include LLM-generated per-page summaries; we surface them
        // as the snippet. Cheap and dramatically improves result usefulness.
        summary: true,
    };

    let resp = http
        .post(ENDPOINT)
        .bearer_auth(&api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(anyhow!(
            "Bocha API error (HTTP {status}): {}",
            truncate(&body)
        ));
    }

    let parsed: BochaResponse = resp.json().await?;

    let items = parsed
        .data
        .and_then(|d| d.web_pages)
        .map(|w| w.value)
        .unwrap_or_default()
        .into_iter()
        .map(|r| {
            // Bocha can return either `snippet` or an LLM `summary`; prefer the
            // longer of the two so callers always get the most informative text.
            let snippet = match (r.snippet.as_deref(), r.summary.as_deref()) {
                (Some(s), Some(u)) if u.len() > s.len() => Some(u.to_string()),
                (Some(s), _) => Some(s.to_string()),
                (None, Some(u)) => Some(u.to_string()),
                (None, None) => None,
            };
            SearchItem {
                title: r.name,
                url: r.url,
                snippet,
                content: None,
            }
        })
        .collect();

    Ok((items, SearchMode::Api))
}

#[derive(Serialize)]
struct BochaRequest {
    query: String,
    count: usize,
    summary: bool,
}

#[derive(Deserialize)]
struct BochaResponse {
    #[serde(default)]
    data: Option<BochaData>,
}

#[derive(Deserialize)]
struct BochaData {
    #[serde(default, rename = "webPages")]
    web_pages: Option<BochaWebPages>,
}

#[derive(Deserialize)]
struct BochaWebPages {
    #[serde(default)]
    value: Vec<BochaItem>,
}

#[derive(Deserialize)]
struct BochaItem {
    name: String,
    url: String,
    #[serde(default)]
    snippet: Option<String>,
    #[serde(default)]
    summary: Option<String>,
}

fn truncate(s: &str) -> String {
    crate::utils::truncate(s, 400)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bocha_response_parsing() {
        let raw = r#"{
            "data": {
                "webPages": {
                    "value": [
                        {"name": "Rust", "url": "https://www.rust-lang.org", "snippet": "短摘要", "summary": "更长的 LLM 摘要内容"},
                        {"name": "只有snippet", "url": "https://example.org/x", "snippet": "只有短摘要"},
                        {"name": "都没有", "url": "https://example.org/y"}
                    ]
                }
            }
        }"#;
        let parsed: BochaResponse = serde_json::from_str(raw).unwrap();
        let items: Vec<SearchItem> = parsed
            .data
            .and_then(|d| d.web_pages)
            .map(|w| w.value)
            .unwrap_or_default()
            .into_iter()
            .map(|r| {
                let snippet = match (r.snippet.as_deref(), r.summary.as_deref()) {
                    (Some(s), Some(u)) if u.len() > s.len() => Some(u.to_string()),
                    (Some(s), _) => Some(s.to_string()),
                    (None, Some(u)) => Some(u.to_string()),
                    (None, None) => None,
                };
                SearchItem {
                    title: r.name,
                    url: r.url,
                    snippet,
                    content: None,
                }
            })
            .collect();
        assert_eq!(items.len(), 3);
        // First: summary longer than snippet → picks summary.
        assert_eq!(items[0].snippet.as_deref(), Some("更长的 LLM 摘要内容"));
        assert_eq!(items[1].snippet.as_deref(), Some("只有短摘要"));
        assert!(items[2].snippet.is_none());
    }

    #[test]
    fn test_bocha_empty_response() {
        let raw = r#"{"code": 200}"#;
        let parsed: BochaResponse = serde_json::from_str(raw).unwrap();
        assert!(parsed.data.is_none());
    }
}
