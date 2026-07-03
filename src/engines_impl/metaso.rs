//! 秘塔 (Metaso) Web Search API — domestic Chinese AI search.
//!
//! An ad-free AI search engine ("没有广告，直达结果"). Its open Search API is a
//! direct drop-in replacement for Bing Search API. Paid per query (¥0.03),
//! with a free starter grant. Supports web / academic / document / image
//! scopes, plus per-page LLM summaries.
//!
//! Set `METASO_API_KEY`. Get one at <https://metaso.cn/search-api/api-keys>.
//!
//! Request contract (recovered from the official playground at
//! <https://metaso.cn/search-api/playground>):
//!   POST <https://metaso.cn/api/v1/search>
//!   Authorization: Bearer `key`
//!   Content-Type: application/json
//!   {
//!     "mode": "search",
//!     "q": "`query`",
//!     "scope": "webpage",          // `METASO_SCOPE` override
//!     "size": "10",
//!     "includeSummary": true,
//!     "conciseSnippet": false
//!   }
//!
//! The response envelope is not formally documented, so we parse it
//! defensively: we walk the JSON for the first array of result-shaped objects
//! (anything carrying a URL), which survives either `{data:{results:[…]}}` or
//! `{data:[…]}` shapes.

use anyhow::{Result, anyhow};
use serde::Serialize;

use crate::client::SearchOptions;
use crate::engines_impl::EngineOutput;
use crate::result::{SearchItem, SearchMode};

const ENDPOINT: &str = "https://metaso.cn/api/v1/search";

/// Default search scope. Metaso accepts at least `webpage`, `image`, and
/// (per the playground UI) academic/document scopes. Override with
/// `METASO_SCOPE`.
const DEFAULT_SCOPE: &str = "webpage";

/// Search with the 秘塔 (Metaso) Web Search API.
///
/// # Errors
///
/// Returns `Err` when `METASO_API_KEY` is missing, the HTTP request fails, or
/// the API returns a non-2xx status.
pub async fn search(
    http: &reqwest::Client,
    query: &str,
    opts: &SearchOptions,
) -> Result<EngineOutput> {
    let api_key = std::env::var("METASO_API_KEY").map_err(|_| {
        anyhow!("METASO_API_KEY not set. Get one at https://metaso.cn/search-api/api-keys")
    })?;

    let scope = std::env::var("METASO_SCOPE").unwrap_or_else(|_| DEFAULT_SCOPE.to_string());
    let size = opts.limit.unwrap_or(10).clamp(1, 50);

    let body = MetasoRequest {
        mode: "search",
        q: query,
        scope: &scope,
        size,
        include_summary: true,
        concise_snippet: false,
    };

    let resp = http
        .post(ENDPOINT)
        .bearer_auth(&api_key)
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(anyhow!(
            "Metaso API error (HTTP {status}): {}",
            truncate(&body)
        ));
    }

    let value: serde_json::Value = resp.json().await?;
    let items = extract_items(&value);

    Ok((items, SearchMode::Api))
}

#[derive(Serialize)]
struct MetasoRequest<'a> {
    mode: &'a str,
    q: &'a str,
    scope: &'a str,
    size: usize,
    include_summary: bool,
    concise_snippet: bool,
}

/// Defensively walk the response JSON for the first array of result objects.
///
/// A "result object" is any JSON object that carries a URL-like field. We then
/// read title / url / snippet out of it using every field-name spelling we've
/// seen across search APIs, so this keeps working even if Metaso tweaks its
/// envelope.
fn extract_items(root: &serde_json::Value) -> Vec<SearchItem> {
    let mut items = Vec::new();
    walk_for_results(root, &mut items, 0);
    items
}

/// Depth-limited DFS. Once we've harvested a result array we stop descending
/// into it, so a nested `{webPages:{value:[…]}}` doesn't double-count.
fn walk_for_results(value: &serde_json::Value, out: &mut Vec<SearchItem>, depth: usize) {
    if out.len() >= 50 || depth > 6 {
        return;
    }

    if let serde_json::Value::Array(arr) = value {
        // Only treat this array as a "results" array if at least one element
        // parses into a real SearchItem (carries a URL). Otherwise keep
        // descending.
        let mut harvested = Vec::new();
        for el in arr {
            if let Some(item) = parse_result(el) {
                harvested.push(item);
            }
        }
        if !harvested.is_empty() {
            out.extend(harvested);
            return;
        }
        for el in arr {
            walk_for_results(el, out, depth + 1);
        }
        return;
    }

    if let serde_json::Value::Object(map) = value {
        for (_, v) in map {
            walk_for_results(v, out, depth + 1);
        }
    }
}

/// Map one JSON object onto a `SearchItem`, iff it has a URL-like field.
fn parse_result(obj: &serde_json::Value) -> Option<SearchItem> {
    let serde_json::Value::Object(map) = obj else {
        return None;
    };

    let url =
        first_str(map, &["url", "link", "href", "origin"]).filter(|u| u.starts_with("http"))?;
    let title = first_str(map, &["title", "name", "headline"])
        .unwrap_or_default()
        .trim()
        .to_string();
    let snippet = first_str(
        map,
        &["summary", "content", "snippet", "description", "abstract"],
    )
    .map(|s| s.trim().to_string())
    .filter(|s| !s.is_empty());

    if title.is_empty() && snippet.is_none() {
        return None;
    }

    Some(SearchItem {
        title,
        url,
        snippet,
        content: None,
    })
}

fn first_str(map: &serde_json::Map<String, serde_json::Value>, keys: &[&str]) -> Option<String> {
    for k in keys {
        if let Some(serde_json::Value::String(s)) = map.get(*k) {
            if !s.is_empty() {
                return Some(s.clone());
            }
        }
    }
    None
}

fn truncate(s: &str) -> String {
    crate::utils::truncate(s, 400)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metaso_request_shape() {
        let req = MetasoRequest {
            mode: "search",
            q: "rust 异步",
            scope: "webpage",
            size: 10,
            include_summary: true,
            concise_snippet: false,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["mode"], "search");
        assert_eq!(json["q"], "rust 异步");
        assert_eq!(json["scope"], "webpage");
        assert_eq!(json["size"], 10);
        assert_eq!(json["include_summary"], true);
    }

    #[test]
    fn test_extract_envelope_data_results() {
        // Common shape: { data: { results: [...] } }
        let raw = r#"{
            "code": 0,
            "data": {
                "results": [
                    {"title": "Rust", "url": "https://www.rust-lang.org", "summary": "Rust 赋能每个人"},
                    {"title": "无摘要", "url": "https://example.org/x", "content": "只有 content"}
                ]
            }
        }"#;
        let v: serde_json::Value = serde_json::from_str(raw).unwrap();
        let items = extract_items(&v);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].title, "Rust");
        assert_eq!(items[0].snippet.as_deref(), Some("Rust 赋能每个人"));
        assert_eq!(items[1].snippet.as_deref(), Some("只有 content"));
    }

    #[test]
    fn test_extract_envelope_data_array() {
        // Alternative shape: { data: [...] }
        let raw = r#"{
            "data": [
                {"name": "标题用 name", "link": "https://example.org/y", "snippet": "短摘要"}
            ]
        }"#;
        let v: serde_json::Value = serde_json::from_str(raw).unwrap();
        let items = extract_items(&v);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "标题用 name");
        assert_eq!(items[0].url, "https://example.org/y");
        assert_eq!(items[0].snippet.as_deref(), Some("短摘要"));
    }

    #[test]
    fn test_extract_skips_non_url_objects() {
        // Objects without a URL must not be harvested; arrays of junk must
        // keep descending (here there's nothing useful underneath, so 0 items).
        let raw = r#"{
            "meta": [{"foo": "bar"}, {"count": 3}],
            "data": {"results": [{"title": "OK", "url": "https://ok.example", "content": "c"}]}
        }"#;
        let v: serde_json::Value = serde_json::from_str(raw).unwrap();
        let items = extract_items(&v);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].url, "https://ok.example");
    }

    #[test]
    fn test_extract_empty() {
        let raw = r#"{"code": 0, "msg": "ok"}"#;
        let v: serde_json::Value = serde_json::from_str(raw).unwrap();
        assert!(extract_items(&v).is_empty());
    }

    #[test]
    fn test_parse_result_alias_fields() {
        let obj = serde_json::json!({
            "name": "Via name",
            "href": "https://via.href/page",
            "description": "via description"
        });
        let item = parse_result(&obj).unwrap();
        assert_eq!(item.title, "Via name");
        assert_eq!(item.url, "https://via.href/page");
        assert_eq!(item.snippet.as_deref(), Some("via description"));

        // No URL → not a result.
        let no_url = serde_json::json!({"title": "x", "content": "y"});
        assert!(parse_result(&no_url).is_none());

        // Relative URL → rejected (only absolute http(s) links count).
        let rel = serde_json::json!({"title": "x", "url": "/relative/path"});
        assert!(parse_result(&rel).is_none());
    }
}
