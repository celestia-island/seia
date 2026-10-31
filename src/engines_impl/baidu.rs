//! Baidu AI Search (千帆 / Qianfan) — `POST {base_url}/v2/ai_search/web_search`.
//!
//! * Auth: `Authorization: Bearer <api_key>` (`BAIDU_API_KEY`)
//! * Body: `{messages:[{role,content}], search_source:"baidu_search_v2",
//!   resource_type_filter:[{type:"web", top_k:N}]}` (+ optional
//!   `chunks_per_doc` when the caller asked for page content)
//! * Response: `{request_id, references:[{id, title, url, web_anchor, website,
//!   snippet, content, date, type, ...}]}`
//!
//! `base_url` defaults to <https://qianfan.baidubce.com> and can be overridden
//! per call (service chain entry) or via `BAIDU_SEARCH_BASE_URL`.
//!
//! The upstream query field is capped at 72 characters; longer queries are
//! rejected locally with an actionable error rather than silently truncated.

use anyhow::{Context as _, Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::{
    client::{EngineConfig, SearchOptions},
    engines::Engine,
    engines_impl::EngineOutput,
    result::{SearchItem, SearchMode},
    utils::truncate,
};

/// Wire protocol identifier for the `seia_search_v1` chain protocol.
pub const PROTOCOL: &str = "baidu_ai_search_v1";
/// Path appended to the (configurable) base URL.
const SEARCH_PATH: &str = "/v2/ai_search/web_search";
const KEY_HINT: &str = "Get one at https://console.bce.baidu.com/qianfan";
/// Documented upper bound for the user message content.
pub const MAX_QUERY_CHARS: usize = 72;

/// Search with the Baidu AI Search API.
///
/// # Errors
///
/// Returns `Err` when the API key is missing, the query exceeds the documented
/// 72-character limit, the HTTP request fails, or the API reports an error.
/// Error messages never contain the API key.
pub async fn search(
    http: &reqwest::Client,
    query: &str,
    opts: &SearchOptions,
    cfg: &EngineConfig,
) -> Result<EngineOutput> {
    let query_len = query.chars().count();
    if query_len > MAX_QUERY_CHARS {
        return Err(anyhow!(
            "Baidu AI Search query is {query_len} characters; the API caps it at {MAX_QUERY_CHARS} (shorten the query or use another provider)"
        ));
    }

    let base_url = cfg.resolve_base_url(&Engine::Baidu)?;
    let api_key = cfg.resolve_api_key(&Engine::Baidu, KEY_HINT)?;
    let endpoint = format!("{base_url}{SEARCH_PATH}");

    let top_k = opts.limit.unwrap_or(10).clamp(1, 50);
    let request = BaiduRequest {
        messages: vec![BaiduMessage {
            role: "user",
            content: query,
        }],
        search_source: "baidu_search_v2",
        resource_type_filter: vec![ResourceTypeFilter {
            r#type: "web",
            top_k,
        }],
        chunks_per_doc: opts.fetch_content.then_some(3),
    };

    let resp = http
        .post(&endpoint)
        .bearer_auth(&api_key)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .with_context(|| format!("Baidu AI Search request to {endpoint} failed"))?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(anyhow!(
            "Baidu AI Search API error (HTTP {status}): {}",
            truncate(&body, 400)
        ));
    }

    let parsed: BaiduResponse = resp
        .json()
        .await
        .context("parsing Baidu AI Search response")?;

    // Qianfan reports some failures as HTTP 200 + non-zero `code`.
    if let Some(code) = parsed.code.filter(|code| *code != 0) {
        let message = parsed
            .message
            .as_deref()
            .map(|m| truncate(m, 300))
            .unwrap_or_else(|| "no message".to_string());
        return Err(anyhow!(
            "Baidu AI Search API error (code {code}): {message}"
        ));
    }

    let items = parsed.references.into_iter().filter_map(to_item).collect();
    Ok((items, SearchMode::Api))
}

/// Map one upstream reference onto the normalized item shape, dropping entries
/// without a usable URL.
fn to_item(raw: BaiduReference) -> Option<SearchItem> {
    let url = clean(raw.url)?;
    let title = clean(raw.title)
        .or_else(|| clean(raw.web_anchor))
        .unwrap_or_default();
    Some(SearchItem {
        title,
        url,
        snippet: clean(raw.snippet),
        content: clean(raw.content),
        summary: None,
        site_name: clean(raw.website),
        site_icon: None,
        date: clean(raw.date),
        authority: None,
        rerank_score: None,
        engine: None,
    })
}

fn clean(value: Option<String>) -> Option<String> {
    value
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[derive(Debug, Serialize)]
struct BaiduRequest<'a> {
    messages: Vec<BaiduMessage<'a>>,
    search_source: &'static str,
    resource_type_filter: Vec<ResourceTypeFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chunks_per_doc: Option<usize>,
}

#[derive(Debug, Serialize)]
struct BaiduMessage<'a> {
    role: &'static str,
    content: &'a str,
}

#[derive(Debug, Serialize)]
struct ResourceTypeFilter {
    #[serde(rename = "type")]
    r#type: &'static str,
    top_k: usize,
}

#[derive(Debug, Deserialize)]
struct BaiduResponse {
    #[serde(default)]
    #[allow(dead_code)]
    request_id: Option<String>,
    #[serde(default)]
    references: Vec<BaiduReference>,
    #[serde(default)]
    code: Option<i64>,
    #[serde(default)]
    message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BaiduReference {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    web_anchor: Option<String>,
    #[serde(default)]
    website: Option<String>,
    #[serde(default)]
    snippet: Option<String>,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    date: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::EngineConfig,
        test_support::{MockServer, http_client},
    };

    const SAMPLE: &str = r#"{
        "request_id": "req-123",
        "references": [
            {
                "id": 1,
                "title": "翁法罗斯 - 百度百科",
                "url": "https://example.org/baike/amphoreus",
                "web_anchor": "翁法罗斯吧",
                "website": "百度百科",
                "snippet": "网页摘要",
                "content": "网页正文内容",
                "date": "2026-09-30",
                "type": "web"
            },
            {
                "id": 2,
                "url": "https://example.org/only-url",
                "web_anchor": "只有锚文本"
            },
            {
                "id": 3,
                "title": "缺 url 的条目应被丢弃",
                "snippet": "no url"
            }
        ]
    }"#;

    fn opts() -> SearchOptions {
        SearchOptions {
            limit: Some(3),
            fetch_content: false,
            searxng_url: None,
        }
    }

    #[tokio::test]
    async fn builds_request_and_normalizes_response() {
        let mock = MockServer::spawn_ok(SAMPLE).await;
        let cfg = EngineConfig::for_test(&mock.base_url);

        let (items, mode) = search(&http_client(), "翁法罗斯", &opts(), &cfg)
            .await
            .expect("baidu search");
        assert_eq!(mode, SearchMode::Api);

        let request = mock.last_request();
        assert_eq!(request.method, "POST");
        assert_eq!(request.path, "/v2/ai_search/web_search");
        assert_eq!(request.header("authorization"), Some("Bearer test-key"));
        let body = request.json();
        assert_eq!(body["search_source"], "baidu_search_v2");
        assert_eq!(body["messages"][0]["role"], "user");
        assert_eq!(body["messages"][0]["content"], "翁法罗斯");
        assert_eq!(body["resource_type_filter"][0]["type"], "web");
        assert_eq!(body["resource_type_filter"][0]["top_k"], 3);
        assert!(body.get("chunks_per_doc").is_none());

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].title, "翁法罗斯 - 百度百科");
        assert_eq!(items[0].snippet.as_deref(), Some("网页摘要"));
        assert_eq!(items[0].content.as_deref(), Some("网页正文内容"));
        assert_eq!(items[0].site_name.as_deref(), Some("百度百科"));
        assert_eq!(items[0].date.as_deref(), Some("2026-09-30"));
        assert!(items[0].authority.is_none());
        // web_anchor is the title fallback when `title` is absent.
        assert_eq!(items[1].title, "只有锚文本");
    }

    #[tokio::test]
    async fn requests_content_chunks_when_content_is_wanted() {
        let mock = MockServer::spawn_ok(r#"{"references":[]}"#).await;
        let cfg = EngineConfig::for_test(&mock.base_url);
        let mut search_opts = opts();
        search_opts.fetch_content = true;

        search(&http_client(), "q", &search_opts, &cfg)
            .await
            .expect("baidu search");
        let body = mock.last_request().json();
        assert_eq!(body["chunks_per_doc"], 3);
    }

    #[tokio::test]
    async fn rejects_query_longer_than_upstream_limit() {
        let mock = MockServer::spawn_ok(SAMPLE).await;
        let cfg = EngineConfig::for_test(&mock.base_url);
        let query = "翁".repeat(MAX_QUERY_CHARS + 1);
        let err = search(&http_client(), &query, &opts(), &cfg)
            .await
            .unwrap_err()
            .to_string();
        assert!(err.contains("72"), "{err}");
        assert!(
            mock.requests().is_empty(),
            "no HTTP call on oversized query"
        );
    }

    #[tokio::test]
    async fn http_200_with_error_code_is_surfaced() {
        let mock = MockServer::spawn_ok(r#"{"code":216100,"message":"invalid parameter"}"#).await;
        let cfg = EngineConfig::for_test(&mock.base_url);
        let err = search(&http_client(), "q", &opts(), &cfg)
            .await
            .unwrap_err()
            .to_string();
        assert!(err.contains("216100"), "{err}");
        assert!(err.contains("invalid parameter"), "{err}");
    }

    #[tokio::test]
    async fn http_error_is_reported_without_the_api_key() {
        let mock = MockServer::spawn(vec![(500, r#"{"message":"upstream exploded"}"#)]).await;
        let cfg = EngineConfig {
            api_key: Some("bce-fake-secret-baidu".into()),
            ..EngineConfig::for_test(&mock.base_url)
        };
        let err = search(&http_client(), "q", &opts(), &cfg)
            .await
            .unwrap_err()
            .to_string();
        assert!(err.contains("500"), "{err}");
        assert!(!err.contains("bce-fake-secret-baidu"), "leaked key: {err}");
    }
}
