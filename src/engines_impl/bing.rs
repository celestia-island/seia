//! Bing Web Search API v7.
//!
//! Microsoft Azure Cognitive Services. Free tier: 1 000 calls/month.
//! Set `BING_SEARCH_API_KEY` (an Azure subscription key, sometimes called
//! Ocp-Apim-Subscription-Key). Get one at Azure Portal → Bing Search v7.

use anyhow::{Result, anyhow};
use serde::Deserialize;

use crate::{
    client::SearchOptions,
    engines_impl::EngineOutput,
    result::{SearchItem, SearchMode},
};

/// Default Bing Search v7 endpoint. Override with `BING_SEARCH_ENDPOINT` for
/// sovereign clouds or self-hosted proxies.
const DEFAULT_ENDPOINT: &str = "https://api.bing.microsoft.com";

/// Search with the Bing Web Search API v7.
///
/// # Errors
///
/// Returns `Err` when `BING_SEARCH_API_KEY` is missing, the HTTP request
/// fails, or the API returns a non-2xx status.
pub async fn search(
    http: &reqwest::Client,
    query: &str,
    opts: &SearchOptions,
) -> Result<EngineOutput> {
    let api_key = std::env::var("BING_SEARCH_API_KEY").map_err(|_| {
        anyhow!("BING_SEARCH_API_KEY not set. Get one at the Azure Portal (Bing Search v7).")
    })?;

    let endpoint = std::env::var("BING_SEARCH_ENDPOINT")
        .unwrap_or_else(|_| DEFAULT_ENDPOINT.to_string())
        .trim_end_matches('/')
        .to_string();

    let count = opts.limit.unwrap_or(10).min(50);
    let url = format!(
        "{}/v7.0/search?q={}&count={}&responseFilter=Webpages",
        endpoint,
        urlencode(query),
        count
    );

    let resp = http
        .get(&url)
        .header("Ocp-Apim-Subscription-Key", &api_key)
        .send()
        .await?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(anyhow!(
            "Bing API error (HTTP {status}): {}",
            truncate(&body)
        ));
    }

    let parsed: BingResponse = resp.json().await?;

    let items = parsed
        .web_pages
        .map(|w| w.value)
        .unwrap_or_default()
        .into_iter()
        .map(|r| SearchItem {
            title: r.name,
            url: r.url,
            snippet: Some(r.snippet).filter(|s| !s.is_empty()),
            content: None,
        })
        .collect();

    Ok((items, SearchMode::Api))
}

#[derive(Deserialize)]
struct BingResponse {
    #[serde(default, rename = "webPages")]
    web_pages: Option<WebPages>,
}

#[derive(Deserialize)]
struct WebPages {
    #[serde(default)]
    value: Vec<BingItem>,
}

#[derive(Deserialize)]
struct BingItem {
    name: String,
    url: String,
    #[serde(default)]
    snippet: String,
}

fn truncate(s: &str) -> String {
    crate::utils::truncate(s, 400)
}

mod urlencode {
    use std::fmt::Write;

    pub fn encode(input: &str) -> String {
        let mut out = String::with_capacity(input.len() * 3);
        for byte in input.bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    out.push(byte as char);
                }
                b' ' => out.push('+'),
                _ => {
                    let _ = write!(out, "%{byte:02X}");
                }
            }
        }
        out
    }
}

use urlencode::encode as urlencode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bing_urlencode() {
        assert_eq!(urlencode("rust async"), "rust+async");
        assert_eq!(urlencode("a&b=c"), "a%26b%3Dc");
    }

    #[test]
    fn test_bing_response_parsing() {
        let raw = r#"{
            "webPages": {
                "value": [
                    {"name": "Rust Lang", "url": "https://www.rust-lang.org", "snippet": "A language empowering everyone"},
                    {"name": "Second", "url": "https://example.org/x", "snippet": ""}
                ]
            }
        }"#;
        let parsed: BingResponse = serde_json::from_str(raw).unwrap();
        let items: Vec<SearchItem> = parsed
            .web_pages
            .map(|w| w.value)
            .unwrap_or_default()
            .into_iter()
            .map(|r| SearchItem {
                title: r.name,
                url: r.url,
                snippet: Some(r.snippet).filter(|s| !s.is_empty()),
                content: None,
            })
            .collect();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].title, "Rust Lang");
        assert_eq!(
            items[0].snippet.as_deref(),
            Some("A language empowering everyone")
        );
        assert!(
            items[1].snippet.is_none(),
            "empty snippet should be dropped"
        );
    }

    #[test]
    fn test_bing_empty_response() {
        let raw = r#"{"_type": "SearchResponse"}"#;
        let parsed: BingResponse = serde_json::from_str(raw).unwrap();
        assert!(parsed.web_pages.is_none());
    }
}
