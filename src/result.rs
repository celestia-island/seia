//! Search result types — shared across all engines.

use serde::{Deserialize, Serialize};

/// A complete search response from any engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub engine: String,
    pub query: String,
    pub mode: SearchMode,
    pub items: Vec<SearchItem>,
    pub elapsed_ms: u64,
}

/// A single search hit.
///
/// The normalized shape follows the DeepSeek Search API result fields
/// (`title` / `url` / `snippet` / `summary` / `content` / `site_name` /
/// `site_icon` / `date` / `authority` / `rerank_score`) plus the engine that
/// actually produced the item. Only `title` and `url` are required; every
/// other field is optional and omitted from JSON when absent, so the historic
/// shape (`title` + `url` + optional `snippet`/`content`) deserializes and
/// serializes unchanged.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchItem {
    pub title: String,
    pub url: String,
    /// Short snippet/abstract from the search engine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
    /// Full page content (engine-provided, or fetched when `--fetch` is used).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// LLM-oriented per-page summary, when the provider supplies one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Site display name (e.g. `小米游戏中心`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    /// Site icon URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_icon: Option<String>,
    /// Publication date, RFC 3339 or `YYYY-MM-DD HH:MM:SS` as returned upstream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Upstream authority score: `0..=4`, `-1` = unknown. Absent when the
    /// provider does not report one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<i32>,
    /// Upstream rerank score: `0..=3`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerank_score: Option<f32>,
    /// Engine that actually produced this item (`deepseek`, `baidu`, ...).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
}

/// How the search was executed.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SearchMode {
    /// Called a search provider's HTTP API.
    Api,
    /// Scraped an HTML search results page (no official API).
    Scrape,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_item_json_still_deserializes() {
        // Shape emitted by seia <= 0.1.6 and stored by downstream consumers.
        let raw = r#"{
            "title": "Rust",
            "url": "https://www.rust-lang.org",
            "snippet": "短摘要"
        }"#;
        let item: SearchItem = serde_json::from_str(raw).unwrap();
        assert_eq!(item.title, "Rust");
        assert_eq!(item.url, "https://www.rust-lang.org");
        assert_eq!(item.snippet.as_deref(), Some("短摘要"));
        assert!(item.content.is_none());
        assert!(item.summary.is_none());
        assert!(item.site_name.is_none());
        assert!(item.site_icon.is_none());
        assert!(item.date.is_none());
        assert!(item.authority.is_none());
        assert!(item.rerank_score.is_none());
        assert!(item.engine.is_none());
    }

    #[test]
    fn legacy_item_serializes_without_new_fields() {
        let item = SearchItem {
            title: "Rust".into(),
            url: "https://www.rust-lang.org".into(),
            snippet: Some("短摘要".into()),
            ..Default::default()
        };
        let json = serde_json::to_value(&item).unwrap();
        let mut keys: Vec<_> = json.as_object().unwrap().keys().cloned().collect();
        keys.sort();
        assert_eq!(keys, vec!["snippet", "title", "url"]);
    }

    #[test]
    fn normalized_item_round_trips() {
        let raw = r#"{
            "title": "翁法罗斯 - 米游社",
            "url": "https://example.org/amphoreus",
            "snippet": "短摘要",
            "summary": "更长的 LLM 摘要",
            "content": "正文 markdown",
            "site_name": "米游社",
            "site_icon": "https://example.org/icon.png",
            "date": "2026-10-01 12:00:00",
            "authority": 4,
            "rerank_score": 2.5,
            "engine": "deepseek"
        }"#;
        let item: SearchItem = serde_json::from_str(raw).unwrap();
        assert_eq!(item.summary.as_deref(), Some("更长的 LLM 摘要"));
        assert_eq!(item.content.as_deref(), Some("正文 markdown"));
        assert_eq!(item.site_name.as_deref(), Some("米游社"));
        assert_eq!(
            item.site_icon.as_deref(),
            Some("https://example.org/icon.png")
        );
        assert_eq!(item.date.as_deref(), Some("2026-10-01 12:00:00"));
        assert_eq!(item.authority, Some(4));
        assert_eq!(item.rerank_score, Some(2.5));
        assert_eq!(item.engine.as_deref(), Some("deepseek"));

        let back = serde_json::to_string(&item).unwrap();
        let reparsed: SearchItem = serde_json::from_str(&back).unwrap();
        assert_eq!(reparsed.title, item.title);
        assert_eq!(reparsed.authority, item.authority);
        assert_eq!(reparsed.engine, item.engine);
    }

    #[test]
    fn unknown_authority_marker_is_preserved() {
        // DeepSeek documents -1 as "unknown authority"; it must survive
        // normalization instead of being mistaken for a real score.
        let raw = r#"{"title":"t","url":"https://example.org","authority":-1}"#;
        let item: SearchItem = serde_json::from_str(raw).unwrap();
        assert_eq!(item.authority, Some(-1));
    }
}
