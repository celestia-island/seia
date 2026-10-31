//! DeepSeek Search API — the standardized AI-search shape that the normalized
//! `SearchItem` model follows.
//!
//! * Endpoint: `POST {base_url}/v1/tools/search`
//! * Auth: `Authorization: Bearer <api_key>` (`DEEPSEEK_API_KEY`)
//! * Body: `{query, count, ori_question?, localization?, need_content?}`
//! * Response: `{results:[{rerank_score, title, url, snippet, summary,
//!   content, site_name, site_icon, date, authority}]}`
//!
//! `base_url` defaults to <https://api.deepseek.com> and can be overridden per
//! call (service chain entry) or via `DEEPSEEK_SEARCH_BASE_URL`.

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
pub const PROTOCOL: &str = "deepseek_search_v1";
/// Path appended to the (configurable) base URL.
const SEARCH_PATH: &str = "/v1/tools/search";
const KEY_HINT: &str = "Get one at https://platform.deepseek.com";

/// Search with the DeepSeek Search API.
///
/// # Errors
///
/// Returns `Err` when the API key is missing, the HTTP request fails, or the
/// API returns a non-2xx status. Error messages never contain the API key.
pub async fn search(
    http: &reqwest::Client,
    query: &str,
    opts: &SearchOptions,
    cfg: &EngineConfig,
) -> Result<EngineOutput> {
    let base_url = cfg.resolve_base_url(&Engine::Deepseek)?;
    let api_key = cfg.resolve_api_key(&Engine::Deepseek, KEY_HINT)?;
    let endpoint = format!("{base_url}{SEARCH_PATH}");

    let request = DeepseekRequest {
        query,
        count: opts.limit.unwrap_or(10).clamp(1, 50),
        ori_question: cfg.ori_question.as_deref(),
        localization: cfg.locale.as_deref(),
        need_content: opts.fetch_content,
    };

    let resp = http
        .post(&endpoint)
        .bearer_auth(&api_key)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .with_context(|| format!("DeepSeek Search request to {endpoint} failed"))?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(anyhow!(
            "DeepSeek Search API error (HTTP {status}): {}",
            truncate(&body, 400)
        ));
    }

    let parsed: DeepseekResponse = resp
        .json()
        .await
        .context("parsing DeepSeek Search response")?;

    let items = parsed.results.into_iter().filter_map(to_item).collect();
    Ok((items, SearchMode::Api))
}

/// Map one upstream result onto the normalized item shape, dropping entries
/// without a usable URL.
fn to_item(raw: DeepseekItem) -> Option<SearchItem> {
    let url = clean(raw.url)?;
    Some(SearchItem {
        title: clean(raw.title).unwrap_or_default(),
        url,
        snippet: clean(raw.snippet),
        content: clean(raw.content),
        summary: clean(raw.summary),
        site_name: clean(raw.site_name),
        site_icon: clean(raw.site_icon),
        date: clean(raw.date),
        authority: raw.authority.map(|a| a.round() as i32),
        rerank_score: raw.rerank_score.map(|s| s as f32),
        engine: None,
    })
}

fn clean(value: Option<String>) -> Option<String> {
    value
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[derive(Debug, Serialize)]
struct DeepseekRequest<'a> {
    query: &'a str,
    count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    ori_question: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    localization: Option<&'a str>,
    need_content: bool,
}

#[derive(Debug, Deserialize)]
struct DeepseekResponse {
    #[serde(default)]
    results: Vec<DeepseekItem>,
}

#[derive(Debug, Deserialize)]
struct DeepseekItem {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    snippet: Option<String>,
    #[serde(default)]
    summary: Option<String>,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    site_name: Option<String>,
    #[serde(default)]
    site_icon: Option<String>,
    #[serde(default)]
    date: Option<String>,
    /// Documented as 0..=4 with -1 for "unknown"; parsed as a float so a
    /// provider sending `4.0` still normalizes.
    #[serde(default)]
    authority: Option<f64>,
    #[serde(default)]
    rerank_score: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::EngineConfig,
        test_support::{MockServer, http_client},
    };

    const SAMPLE: &str = r#"{
        "results": [
            {
                "rerank_score": 2.5,
                "title": "翁法罗斯 - 米游社",
                "url": "https://example.org/amphoreus",
                "snippet": "短摘要",
                "summary": "更长的 LLM 摘要",
                "content": "正文 markdown",
                "site_name": "米游社",
                "site_icon": "https://example.org/icon.png",
                "date": "2026-10-01 12:00:00",
                "authority": 4
            },
            {
                "title": "只有基础字段",
                "url": "https://example.org/minimal"
            },
            {
                "title": "缺 url 的条目应被丢弃",
                "snippet": "no url"
            },
            {
                "rerank_score": 1.0,
                "title": "未知权威度",
                "url": "https://example.org/unknown",
                "authority": -1
            }
        ]
    }"#;

    fn opts() -> SearchOptions {
        SearchOptions {
            limit: Some(5),
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
            .expect("deepseek search");
        assert_eq!(mode, SearchMode::Api);

        // Request shape.
        let request = mock.last_request();
        assert_eq!(request.method, "POST");
        assert_eq!(request.path, "/v1/tools/search");
        assert_eq!(request.header("authorization"), Some("Bearer test-key"));
        let body = request.json();
        assert_eq!(body["query"], "翁法罗斯");
        assert_eq!(body["count"], 5);
        assert_eq!(body["need_content"], false);
        assert!(body.get("ori_question").is_none());
        assert!(body.get("localization").is_none());

        // Response normalization (the URL-less entry is dropped).
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].title, "翁法罗斯 - 米游社");
        assert_eq!(items[0].snippet.as_deref(), Some("短摘要"));
        assert_eq!(items[0].summary.as_deref(), Some("更长的 LLM 摘要"));
        assert_eq!(items[0].content.as_deref(), Some("正文 markdown"));
        assert_eq!(items[0].site_name.as_deref(), Some("米游社"));
        assert_eq!(
            items[0].site_icon.as_deref(),
            Some("https://example.org/icon.png")
        );
        assert_eq!(items[0].date.as_deref(), Some("2026-10-01 12:00:00"));
        assert_eq!(items[0].authority, Some(4));
        assert_eq!(items[0].rerank_score, Some(2.5));
        // Untagged item shape: only title/url survive.
        assert_eq!(items[1].title, "只有基础字段");
        assert!(items[1].snippet.is_none());
        assert!(items[1].authority.is_none());
        // -1 stays -1 (documented "unknown"), not a valid score.
        assert_eq!(items[2].authority, Some(-1));
    }

    #[tokio::test]
    async fn forwards_content_flag_and_hints() {
        let mock = MockServer::spawn_ok(r#"{"results":[]}"#).await;
        let cfg = EngineConfig {
            ori_question: Some("翁法罗斯是什么？".into()),
            locale: Some("zh-CN".into()),
            ..EngineConfig::for_test(&mock.base_url)
        };
        let mut search_opts = opts();
        search_opts.fetch_content = true;
        search_opts.limit = Some(100); // clamped to the documented 50

        let (items, _) = search(&http_client(), "翁法罗斯 资料", &search_opts, &cfg)
            .await
            .expect("deepseek search");
        assert!(items.is_empty());

        let body = mock.last_request().json();
        assert_eq!(body["count"], 50);
        assert_eq!(body["need_content"], true);
        assert_eq!(body["ori_question"], "翁法罗斯是什么？");
        assert_eq!(body["localization"], "zh-CN");
    }

    #[tokio::test]
    async fn http_error_is_reported_without_the_api_key() {
        let mock =
            MockServer::spawn(vec![(401, r#"{"error":{"message":"Invalid API key"}}"#)]).await;
        let cfg = EngineConfig {
            api_key: Some("sk-fake-secret-deepseek".into()),
            ..EngineConfig::for_test(&mock.base_url)
        };
        let err = search(&http_client(), "q", &opts(), &cfg)
            .await
            .unwrap_err()
            .to_string();
        assert!(err.contains("401"), "{err}");
        assert!(
            !err.contains("sk-fake-secret-deepseek"),
            "leaked key: {err}"
        );
    }
}
