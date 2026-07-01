//! Tests for seia library — CLI smoke tests, API unit tests, engine verification.

mod tests {
    use seia::{Engine, SearchClient, SearchOptions};

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
    #[tokio::test]
    async fn test_wikipedia_smoke() {
        let client = SearchClient::new();
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
    #[tokio::test]
    async fn test_wikipedia_academic() {
        let client = SearchClient::new();
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
    }

    #[test]
    fn test_engine_api_key_env() {
        assert!(Engine::Duckduckgo.api_key_env().is_none());
        assert!(Engine::Wikipedia.api_key_env().is_none());
        assert_eq!(Engine::Tavily.api_key_env(), Some("TAVILY_API_KEY"));
        assert_eq!(Engine::Bing.api_key_env(), Some("BING_SEARCH_API_KEY"));
        assert_eq!(Engine::Brave.api_key_env(), Some("BRAVE_SEARCH_API_KEY"));
    }

    #[test]
    fn test_engine_needs_key() {
        assert!(!Engine::Duckduckgo.needs_key());
        assert!(!Engine::Wikipedia.needs_key());
        assert!(Engine::Tavily.needs_key());
        assert!(Engine::Bing.needs_key());
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
    #[tokio::test]
    async fn test_search_fallback() {
        let client = SearchClient::new();
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
}
