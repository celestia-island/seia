//! Brave Search API — independent web index, no Big Tech dependency.
//!
//! Free tier: 2 000 queries/month (Web Search, base plan).
//! Set `BRAVE_SEARCH_API_KEY`. Get one at <https://brave.com/search/api/>.
//!
//! API reference: <https://api-dashboard.search.brave.com/app/documentation/web-search/get-started>

use anyhow::{Result, anyhow};
use serde::Deserialize;

use crate::{
    client::SearchOptions,
    engines_impl::EngineOutput,
    result::{SearchItem, SearchMode},
};

const ENDPOINT: &str = "https://api.search.brave.com/res/v1/web/search";

/// Search with the Brave Search API.
///
/// # Errors
///
/// Returns `Err` when `BRAVE_SEARCH_API_KEY` is missing, the HTTP request
/// fails, or the API returns a non-2xx status.
pub async fn search(
    http: &reqwest::Client,
    query: &str,
    opts: &SearchOptions,
) -> Result<EngineOutput> {
    let api_key = std::env::var("BRAVE_SEARCH_API_KEY").map_err(|_| {
        anyhow!("BRAVE_SEARCH_API_KEY not set. Get one at https://brave.com/search/api/")
    })?;

    let count = opts.limit.unwrap_or(10).min(20);

    let resp = http
        .get(ENDPOINT)
        .header("X-Subscription-Token", &api_key)
        .header("Accept", "application/json")
        .header("Accept-Encoding", "gzip")
        .query(&[
            ("q", query),
            ("count", &count.to_string()),
            // Use the free base plan: no AI snippets, no summarizer.
            ("result_filter", "web"),
        ])
        .send()
        .await?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(anyhow!(
            "Brave API error (HTTP {status}): {}",
            truncate(&body)
        ));
    }

    let parsed: BraveResponse = resp.json().await?;

    let items = parsed
        .web
        .and_then(|w| w.results)
        .unwrap_or_default()
        .into_iter()
        .map(|r| SearchItem {
            title: r.title,
            url: r.url,
            snippet: r.description.filter(|s| !s.is_empty()),
            content: None,
        })
        .collect();

    Ok((items, SearchMode::Api))
}

#[derive(Deserialize)]
struct BraveResponse {
    #[serde(default)]
    web: Option<BraveWeb>,
}

#[derive(Deserialize)]
struct BraveWeb {
    #[serde(default)]
    results: Option<Vec<BraveItem>>,
}

#[derive(Deserialize)]
struct BraveItem {
    title: String,
    url: String,
    #[serde(default)]
    description: Option<String>,
}

fn truncate(s: &str) -> String {
    crate::utils::truncate(s, 400)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brave_response_parsing() {
        let raw = r#"{
            "web": {
                "results": [
                    {"title": "Rust", "url": "https://www.rust-lang.org", "description": "Empower everyone"},
                    {"title": "No desc", "url": "https://example.org/x"}
                ]
            }
        }"#;
        let parsed: BraveResponse = serde_json::from_str(raw).unwrap();
        let items: Vec<SearchItem> = parsed
            .web
            .and_then(|w| w.results)
            .unwrap_or_default()
            .into_iter()
            .map(|r| SearchItem {
                title: r.title,
                url: r.url,
                snippet: r.description.filter(|s| !s.is_empty()),
                content: None,
            })
            .collect();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].snippet.as_deref(), Some("Empower everyone"));
        assert!(items[1].snippet.is_none());
    }

    #[test]
    fn test_brave_empty_response() {
        let raw = r#"{"type": "search"}"#;
        let parsed: BraveResponse = serde_json::from_str(raw).unwrap();
        assert!(parsed.web.is_none());
    }
}
