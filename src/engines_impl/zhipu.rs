//! 智谱 (Zhipu / `BigModel`) Web Search API.
//!
//! A search engine built for LLMs, with intent recognition on top of
//! traditional web crawling/ranking. Returns titles, URLs, snippets, media
//! names and icons. Supports multiple backing engines: 智谱基础版 (`search_std`),
//! 智谱高阶版 (`search_pro`), 搜狗 (`search_pro_sogou`), 夸克 (`search_pro_quark`).
//!
//! Set `ZHIPU_API_KEY`. Get one at <https://bigmodel.cn/usercenter/proj-mgmt/apikeys>.
//!
//! API reference: <https://docs.bigmodel.cn/cn/guide/tools/web-search>

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::{
    client::SearchOptions,
    engines_impl::EngineOutput,
    result::{SearchItem, SearchMode},
};

const ENDPOINT: &str = "https://open.bigmodel.cn/api/paas/v4/web_search";

/// Which backing search engine Zhipu should route the query to.
///
/// Override with the `ZHIPU_SEARCH_ENGINE` env var. Defaults to `search_std`
/// (智谱基础版) — the cheapest tier that still returns full title/url/snippet.
#[derive(Debug, Clone, Copy, Default)]
enum ZhipuSearchEngine {
    /// 智谱基础版搜索引擎 (default, cheapest).
    #[default]
    Std,
    /// 智谱高阶版搜索引擎 (higher quality).
    Pro,
    /// 搜狗 (Sogou).
    Sogou,
    /// 夸克搜索 (Quark).
    Quark,
}

impl ZhipuSearchEngine {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Std => "search_std",
            Self::Pro => "search_pro",
            Self::Sogou => "search_pro_sogou",
            Self::Quark => "search_pro_quark",
        }
    }
}

/// Search using the 智谱 (Zhipu) Web Search API.
///
/// # Errors
///
/// Returns `Err` when `ZHIPU_API_KEY` is missing, the HTTP request fails, or
/// the API returns a non-2xx status.
pub async fn search(
    http: &reqwest::Client,
    query: &str,
    opts: &SearchOptions,
) -> Result<EngineOutput> {
    let api_key = std::env::var("ZHIPU_API_KEY")
        .map_err(|_| anyhow!("ZHIPU_API_KEY not set. Get one at https://bigmodel.cn/"))?;

    let engine = match std::env::var("ZHIPU_SEARCH_ENGINE").ok().as_deref() {
        Some("search_pro") => ZhipuSearchEngine::Pro,
        Some("search_pro_sogou" | "sogou") => ZhipuSearchEngine::Sogou,
        Some("search_pro_quark" | "quark") => ZhipuSearchEngine::Quark,
        _ => ZhipuSearchEngine::Std,
    };

    let count = opts.limit.unwrap_or(10).clamp(1, 50);

    let body = WebSearchRequest {
        search_query: query.to_string(),
        search_engine: engine.as_str(),
        search_intent: false,
        count,
        content_size: "medium",
    };

    let resp = http
        .post(ENDPOINT)
        .bearer_auth(&api_key)
        .json(&body)
        .send()
        .await?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(anyhow!(
            "Zhipu API error (HTTP {status}): {}",
            truncate(&body)
        ));
    }

    let parsed: WebSearchResponse = resp.json().await?;

    let items = parsed
        .search_result
        .unwrap_or_default()
        .into_iter()
        .map(|r| SearchItem {
            title: r.title,
            url: r.link,
            snippet: Some(r.content).filter(|s| !s.is_empty()),
            content: None,
        })
        .collect();

    Ok((items, SearchMode::Api))
}

#[derive(Serialize)]
struct WebSearchRequest<'a> {
    search_query: String,
    search_engine: &'a str,
    search_intent: bool,
    count: usize,
    content_size: &'a str,
}

#[derive(Deserialize)]
struct WebSearchResponse {
    #[serde(default)]
    search_result: Option<Vec<WebSearchItem>>,
}

#[derive(Deserialize)]
struct WebSearchItem {
    #[serde(default)]
    title: String,
    #[serde(default)]
    link: String,
    #[serde(default)]
    content: String,
}

fn truncate(s: &str) -> String {
    crate::utils::truncate(s, 400)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zhipu_engine_routing() {
        // Smoke: the default config + override strings map to the right codes.
        let cases = [
            (None, "search_std"),
            (Some("search_pro"), "search_pro"),
            (Some("search_pro_sogou"), "search_pro_sogou"),
            (Some("sogou"), "search_pro_sogou"),
            (Some("quark"), "search_pro_quark"),
        ];
        for (raw, expected) in cases {
            let engine = match raw {
                None => ZhipuSearchEngine::Std,
                Some("search_pro") => ZhipuSearchEngine::Pro,
                Some("search_pro_sogou" | "sogou") => ZhipuSearchEngine::Sogou,
                Some("search_pro_quark" | "quark") => ZhipuSearchEngine::Quark,
                Some(_) => ZhipuSearchEngine::Std,
            };
            assert_eq!(engine.as_str(), expected);
        }
    }

    #[test]
    fn test_zhipu_response_parsing() {
        let raw = r#"{
            "id": "abc",
            "created": 1700000000,
            "search_result": [
                {"title": "Rust 语言", "link": "https://www.rust-lang.org", "content": "Rust 赋能每个人"},
                {"title": "空摘要", "link": "https://example.org/x", "content": ""}
            ]
        }"#;
        let parsed: WebSearchResponse = serde_json::from_str(raw).unwrap();
        let items: Vec<SearchItem> = parsed
            .search_result
            .unwrap_or_default()
            .into_iter()
            .map(|r| SearchItem {
                title: r.title,
                url: r.link,
                snippet: Some(r.content).filter(|s| !s.is_empty()),
                content: None,
            })
            .collect();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].snippet.as_deref(), Some("Rust 赋能每个人"));
        assert!(items[1].snippet.is_none());
    }

    #[test]
    fn test_zhipu_empty_response() {
        let raw = r#"{"id": "x", "created": 0}"#;
        let parsed: WebSearchResponse = serde_json::from_str(raw).unwrap();
        assert!(parsed.search_result.is_none());
    }
}
