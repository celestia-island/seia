//! Endpoint invariants for the keyed engines.
//!
//! `Engine::default_base_url()` must be **host-only** and the engine's
//! `SEARCH_PATH` must carry the full resource path, so that
//! `default_base_url() + SEARCH_PATH` equals the provider-registry endpoint
//! (`search/<provider>/default.toml` → `[entrypoint.api] base_url` + `endpoint`).
//!
//! Regression guard for the served-path 404s: bases with a path prefix (e.g.
//! `https://api.bing.microsoft.com/v7.0`) used to be concatenated with the full
//! search path again, producing `/v7.0/v7.0/search`.

use seia::{Engine, engines_impl};

/// Concatenate the engine's default base URL with its search path, exactly the
/// way the adapters build the request URL when no `base_url` override is given.
fn default_endpoint(engine: &Engine, search_path: &str) -> String {
    let base = engine
        .default_base_url()
        .unwrap_or_else(|| panic!("{} has no default base URL", engine.as_str()));
    format!("{base}{search_path}")
}

#[test]
fn keyed_engine_default_endpoints_match_provider_registry() {
    // Expected values are copied verbatim from provider-registry
    // `search/<provider>/default.toml` (`base_url` + `endpoint`).
    let cases: [(&str, Engine, &str, &str); 9] = [
        (
            "deepseek",
            Engine::Deepseek,
            engines_impl::deepseek::SEARCH_PATH,
            "https://api.deepseek.com/v1/tools/search",
        ),
        (
            "baidu",
            Engine::Baidu,
            engines_impl::baidu::SEARCH_PATH,
            "https://qianfan.baidubce.com/v2/ai_search/web_search",
        ),
        (
            "tavily",
            Engine::Tavily,
            engines_impl::tavily::SEARCH_PATH,
            "https://api.tavily.com/search",
        ),
        (
            "brave",
            Engine::Brave,
            engines_impl::brave::SEARCH_PATH,
            "https://api.search.brave.com/res/v1/web/search",
        ),
        (
            "bing",
            Engine::Bing,
            engines_impl::bing::SEARCH_PATH,
            "https://api.bing.microsoft.com/v7.0/search",
        ),
        (
            "bocha",
            Engine::Bocha,
            engines_impl::bocha::SEARCH_PATH,
            "https://api.bochaai.com/v1/web-search",
        ),
        (
            "metaso",
            Engine::Metaso,
            engines_impl::metaso::SEARCH_PATH,
            "https://metaso.cn/api/v1/search",
        ),
        (
            "zhipu",
            Engine::Zhipu,
            engines_impl::zhipu::SEARCH_PATH,
            "https://open.bigmodel.cn/api/paas/v4/web_search",
        ),
        (
            "core",
            Engine::Core,
            engines_impl::core::SEARCH_PATH,
            "https://api.core.ac.uk/v3/search/works",
        ),
    ];

    for (id, engine, search_path, expected) in cases {
        assert_eq!(engine.as_str(), id);

        let base = engine
            .default_base_url()
            .unwrap_or_else(|| panic!("{id} has no default base URL"));
        // Host-only base: everything after the scheme must be a bare host.
        let host = base
            .split_once("://")
            .map(|(_, rest)| rest)
            .unwrap_or_else(|| panic!("{id} base URL is not absolute: {base}"));
        assert!(
            !host.contains('/'),
            "{id} default base URL must be host-only, got {base:?}"
        );

        let endpoint = default_endpoint(&engine, search_path);
        assert_eq!(
            endpoint, expected,
            "{id} default endpoint drifted from provider-registry"
        );
        // A doubled prefix would show the search path twice.
        assert_eq!(
            endpoint.matches(search_path).count(),
            1,
            "{id} default endpoint repeats its search path: {endpoint}"
        );
    }
}
