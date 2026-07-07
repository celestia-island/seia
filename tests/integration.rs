//! Tests for seia library — CLI smoke tests, API unit tests, engine verification.

mod tests {

    use seia::{Engine, SearchClient, SearchOptions};

    /// Build a client for the live-network tests.
    ///
    /// Route traffic through a local proxy (e.g. a clash/v2ray on 7890) by
    /// setting `SEIA_TEST_PROXY=http://localhost:7890`. With it unset the
    /// client connects directly — which is what CI does (GitHub runners have
    /// direct internet). `SearchClient::new` also still honors the standard
    /// `HTTPS_PROXY` env var via reqwest.
    fn test_client() -> SearchClient {
        if let Ok(p) = std::env::var("SEIA_TEST_PROXY") {
            if !p.is_empty() {
                return SearchClient::with_proxy(&p)
                    .unwrap_or_else(|e| panic!("invalid SEIA_TEST_PROXY={p:?}: {e}"));
            }
        }
        SearchClient::new()
    }

    /// Smoke test: DuckDuckGo returns results for a simple query.
    #[tokio::test]
    #[ignore = "DuckDuckGo may rate-limit; run manually with --ignored"]
    async fn test_duckduckgo_smoke() {
        let client = SearchClient::new();
        let result = client
            .search("hello world", Engine::Duckduckgo)
            .await
            .expect("DuckDuckGo search should succeed");

        assert_eq!(result.engine, "duckduckgo");
        assert!(!result.items.is_empty(), "should return at least 1 result");
        assert!(result.elapsed_ms > 0, "elapsed should be positive");

        for item in &result.items {
            assert!(!item.title.is_empty(), "title should not be empty");
            assert!(!item.url.is_empty(), "url should not be empty");
        }
    }

    /// Smoke test: Wikipedia API returns results.
    #[ignore = "requires network access; run manually with --ignored"]
    #[tokio::test]
    async fn test_wikipedia_smoke() {
        let client = test_client();
        let result = client
            .search("Klein bottle", Engine::Wikipedia)
            .await
            .expect("Wikipedia search should succeed");

        assert_eq!(result.engine, "wikipedia");
        assert!(!result.items.is_empty(), "should return results");
        assert!(
            result.items[0].title.contains("Klein") || result.items[0].url.contains("Klein"),
            "first result should be about Klein bottle"
        );
    }

    /// Wikipedia: academic query returns relevant results.
    #[ignore = "requires network access; run manually with --ignored"]
    #[tokio::test]
    async fn test_wikipedia_academic() {
        let client = test_client();
        let result = client
            .search("fundamental group of torus", Engine::Wikipedia)
            .await
            .expect("Wikipedia should work");

        assert!(!result.items.is_empty());
        let has_relevant = result.items.iter().any(|item| {
            let text =
                format!("{} {}", item.title, item.snippet.as_deref().unwrap_or("")).to_lowercase();
            text.contains("torus") || text.contains("fundamental")
        });
        assert!(
            has_relevant,
            "should have results about torus or fundamental group"
        );
    }

    /// Engine enumeration and properties.
    #[test]
    fn test_engine_properties() {
        assert_eq!(Engine::Duckduckgo.as_str(), "duckduckgo");
        assert_eq!(Engine::Tavily.as_str(), "tavily");
        assert_eq!(Engine::Wikipedia.as_str(), "wikipedia");
        assert_eq!(Engine::Searxng.as_str(), "searxng");
        assert_eq!(Engine::Bing.as_str(), "bing");
        assert_eq!(Engine::Brave.as_str(), "brave");
        assert_eq!(Engine::Zhipu.as_str(), "zhipu");
        assert_eq!(Engine::Bocha.as_str(), "bocha");
        assert_eq!(Engine::Metaso.as_str(), "metaso");
    }

    #[test]
    fn test_engine_api_key_env() {
        assert!(Engine::Duckduckgo.api_key_env().is_none());
        assert!(Engine::Wikipedia.api_key_env().is_none());
        assert_eq!(Engine::Tavily.api_key_env(), Some("TAVILY_API_KEY"));
        assert_eq!(Engine::Bing.api_key_env(), Some("BING_SEARCH_API_KEY"));
        assert_eq!(Engine::Brave.api_key_env(), Some("BRAVE_SEARCH_API_KEY"));
        assert_eq!(Engine::Zhipu.api_key_env(), Some("ZHIPU_API_KEY"));
        assert_eq!(Engine::Bocha.api_key_env(), Some("BOCHA_API_KEY"));
        assert_eq!(Engine::Metaso.api_key_env(), Some("METASO_API_KEY"));
    }

    #[test]
    fn test_engine_needs_key() {
        assert!(!Engine::Duckduckgo.needs_key());
        assert!(!Engine::Wikipedia.needs_key());
        assert!(Engine::Tavily.needs_key());
        assert!(Engine::Bing.needs_key());
        assert!(Engine::Brave.needs_key());
        assert!(Engine::Zhipu.needs_key());
        assert!(Engine::Bocha.needs_key());
        assert!(Engine::Metaso.needs_key());
    }

    /// SearchOptions defaults.
    #[test]
    fn test_search_options_default() {
        let opts = SearchOptions::default();
        assert_eq!(opts.limit, Some(10));
        assert!(!opts.fetch_content);
    }

    /// Result serialization (for JSON output).
    #[test]
    fn test_result_serialization() {
        let result = seia::SearchResult {
            engine: "test".to_string(),
            query: "hello".to_string(),
            mode: seia::SearchMode::Api,
            items: vec![seia::SearchItem {
                title: "Test".to_string(),
                url: "https://example.com".to_string(),
                snippet: Some("A test snippet".to_string()),
                content: None,
            }],
            elapsed_ms: 42,
        };

        let json = serde_json::to_string(&result).expect("should serialize");
        assert!(json.contains("test"));
        assert!(json.contains("example.com"));
        assert!(json.contains("A test snippet"));
    }

    /// Fallback search: tries multiple engines.
    #[ignore = "requires network access; run manually with --ignored"]
    #[tokio::test]
    async fn test_search_fallback() {
        let client = test_client();
        let result = client
            .search_fallback("mathematics", &[Engine::Duckduckgo, Engine::Wikipedia])
            .await
            .expect("at least one engine should work");

        assert!(!result.items.is_empty());
    }

    /// Tavily engine: gracefully handles missing API key.
    #[tokio::test]
    async fn test_tavily_no_key() {
        let client = SearchClient::new();
        let result = client.search("test", Engine::Tavily).await;
        assert!(result.is_err(), "should fail without API key");
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("TAVILY_API_KEY"),
            "error should mention the key name"
        );
    }

    /// SearXNG engine: gracefully handles missing URL.
    #[tokio::test]
    async fn test_searxng_no_url() {
        let client = SearchClient::new();
        let result = client.search("test", Engine::Searxng).await;
        assert!(result.is_err(), "should fail without SEARXNG_URL");
    }

    /// Keyed engines: each refuses to run without its API key set. We don't
    /// assert the exact env-var name per engine here (that's covered by the
    /// unit tests) — only that a missing key surfaces as an error rather than
    /// a panic or an empty result.
    #[tokio::test]
    async fn test_keyed_engines_fail_without_key() {
        // Make sure none of these are set in the test environment.
        let keys = [
            "BING_SEARCH_API_KEY",
            "BRAVE_SEARCH_API_KEY",
            "ZHIPU_API_KEY",
            "BOCHA_API_KEY",
            "METASO_API_KEY",
        ];
        let client = SearchClient::new();
        for &engine in &[
            Engine::Bing,
            Engine::Brave,
            Engine::Zhipu,
            Engine::Bocha,
            Engine::Metaso,
        ] {
            // If the developer happens to have a key set locally, skip — we
            // can only assert the missing-key path when the key is actually
            // absent.
            let has_key = engine
                .api_key_env()
                .map(|k| keys.contains(&k) && std::env::var(k).is_ok())
                .unwrap_or(false);
            if has_key {
                continue;
            }
            let result = client.search("test", engine).await;
            assert!(result.is_err(), "{engine} should fail without its API key");
        }
    }
}
