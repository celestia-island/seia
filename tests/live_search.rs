//! Smoke tests against free public search APIs (no key required).
//!
//! These tests hit real endpoints, so they are `#[ignore]` by default.
//! Run them locally with:
//!
//! ```bash
//! cargo test --test live_search -- --ignored --test-threads=1
//! ```
//!
//! The CI runs them on a daily cron schedule (see `.github/workflows/live-tests.yml`).

use seia::{Engine, SearchClient, SearchOptions};

fn client() -> SearchClient {
    SearchClient::new()
}

/// DuckDuckGo HTML scraping — free, no key. May be unreachable from some
/// regions (e.g. mainland China); the test marks itself as passed when the
/// network is unavailable (Connect / TLS errors).
#[tokio::test]
#[ignore = "requires network access"]
async fn test_duckduckgo_smoke() {
    match client()
        .search("rust programming language", Engine::Duckduckgo)
        .await
    {
        Ok(result) => {
            assert!(!result.items.is_empty(), "should return at least one result");
            for item in &result.items {
                assert!(!item.title.is_empty());
                assert!(!item.url.is_empty());
            }
        }
        Err(e) => {
            let msg = format!("{e}");
            if msg.contains("tls handshake eof")
                || msg.contains("Connect")
                || msg.contains("timeout")
                || msg.contains("dns error")
                || msg.contains("CAPTCHA")
            {
                eprintln!("[SKIP] DuckDuckGo unreachable (network restriction): {msg}");
            } else {
                panic!("DuckDuckGo search failed unexpectedly: {e}");
            }
        }
    }
}

/// Wikipedia API — free, unlimited, stable JSON response.
#[tokio::test]
#[ignore = "requires network access"]
async fn test_wikipedia_smoke() {
    let result = client()
        .search("Rust programming language", Engine::Wikipedia)
        .await
        .expect("Wikipedia search should succeed");
    assert!(!result.items.is_empty(), "should return at least one result");
    for item in &result.items {
        assert!(!item.title.is_empty());
        assert!(item.url.starts_with("https://"), "URL should be HTTPS");
    }
}

/// DuckDuckGo with content fetching enabled.
#[tokio::test]
#[ignore = "requires network access"]
async fn test_duckduckgo_fetch_content() {
    let opts = SearchOptions {
        fetch_content: true,
        limit: Some(3),
        ..Default::default()
    };
    let result = client()
        .search_with_options("hello world", Engine::Duckduckgo, opts)
        .await
        .expect("DuckDuckGo with fetch should succeed");
    assert!(!result.items.is_empty());
    // Some items may fail to fetch (robots.txt, etc.), but the search itself succeeds.
}

/// Semantic Scholar — free academic paper search.
#[tokio::test]
#[ignore = "requires network access"]
async fn test_semantic_scholar_smoke() {
    let result = client()
        .search("machine learning", Engine::SemanticScholar)
        .await
        .expect("Semantic Scholar search should succeed");
    assert!(!result.items.is_empty(), "should return at least one paper");
    for item in &result.items {
        assert!(!item.title.is_empty());
        assert!(!item.url.is_empty());
    }
}

/// OpenAlex — open scholarly index.
#[tokio::test]
#[ignore = "requires network access"]
async fn test_openalex_smoke() {
    let result = client()
        .search("deep learning", Engine::OpenAlex)
        .await
        .expect("OpenAlex search should succeed");
    assert!(!result.items.is_empty());
    for item in &result.items {
        assert!(!item.title.is_empty());
    }
}

/// arXiv — preprint server.
#[tokio::test]
#[ignore = "requires network access"]
async fn test_arxiv_smoke() {
    let result = client()
        .search("neural network", Engine::Arxiv)
        .await
        .expect("arXiv search should succeed");
    assert!(!result.items.is_empty());
    for item in &result.items {
        assert!(!item.title.is_empty());
    }
}
#[tokio::test]
#[ignore = "requires GITHUB_TOKEN in env"]
async fn test_custom_github_search() {
    use seia::config::{CustomEngineDef, EngineRegistry};
    use std::collections::HashMap;

    let def = CustomEngineDef {
        label: "GitHub Code Search".into(),
        method: "GET".into(),
        url: "https://api.github.com/search/code".into(),
        query_param: Some("q".into()),
        body_template: None,
        headers: {
            let mut h = HashMap::new();
            h.insert("Accept".into(), "application/vnd.github.v3+json".into());
            h.insert("Authorization".into(), "Bearer ${GITHUB_TOKEN}".into());
            h
        },
        result_path: Some("$.items[*]".into()),
        title_field: "name".into(),
        url_field: "html_url".into(),
        snippet_field: Some("repository.full_name".into()),
        pre_request: None,
        limit_param: None,
    };

    let mut registry = EngineRegistry::default();
    registry.engines.insert("github".into(), def);
    let client = client().with_registry(registry);

    let result = client
        .search("fn main language:rust repo:rust-lang/rust", Engine::Custom("github".into()))
        .await
        .expect("GitHub code search should succeed");
    assert!(!result.items.is_empty());
    for item in &result.items {
        assert!(!item.title.is_empty());
        assert!(!item.url.is_empty());
    }
}
