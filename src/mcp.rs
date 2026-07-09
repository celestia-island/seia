//! Standalone MCP (Model Context Protocol) server for seia.
//!
//! Exposes the multi-engine web search client over stdio so an AI coding
//! assistant can run a single query against any of the nine backends — or a
//! fallback chain — without leaving the conversation. Activate with the `mcp`
//! cargo feature and `seia mcp`.
//!
//! # Usage
//!
//! ```ignore
//! seia mcp
//! ```

#![cfg(feature = "mcp")]

use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler, ServiceExt,
    handler::server::wrapper::Parameters, model::*, service::RequestContext, tool, tool_handler,
    tool_router,
};
use schemars::JsonSchema;

use crate::{Engine, SearchClient, SearchOptions};

struct Server {
    client: SearchClient,
}

impl Server {
    fn tool_result(text: impl Into<String>) -> CallToolResult {
        CallToolResult::success(vec![Content::text(text)])
    }

    /// Build `SearchOptions` honouring the MCP-level knobs.
    fn opts(&self, limit: Option<usize>, fetch_content: bool) -> SearchOptions {
        SearchOptions {
            limit,
            fetch_content,
            searxng_url: std::env::var("SEARXNG_URL").ok().filter(|s| !s.is_empty()),
        }
    }
}

// ── Tool argument structs ────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
struct SearchArgs {
    /// The search query.
    query: String,
    /// Engine name: duckduckgo, wikipedia, searxng, tavily, bing, brave,
    /// zhipu, bocha, metaso. Defaults to duckduckgo (no key needed).
    engine: Option<String>,
    /// Maximum results to return (default 10).
    limit: Option<usize>,
    /// Fetch the full page content for each result (slower, richer).
    #[serde(rename = "fetch_content", default)]
    fetch_content: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct SearchMultiArgs {
    query: String,
    /// Engines to try, in order; returns the first that yields results.
    /// Defaults to [duckduckgo, wikipedia, bing].
    engines: Option<Vec<String>>,
    limit: Option<usize>,
    #[serde(rename = "fetch_content", default)]
    fetch_content: Option<bool>,
}

// ── Search tools ─────────────────────────────────────

#[tool_router]
impl Server {
    #[tool(
        description = "Search the web with one engine. Engines that need no key (duckduckgo, wikipedia) work out of the box; others require their *_API_KEY env var. Returns titles, URLs, snippets, and optionally full page content."
    )]
    async fn seia_search(
        &self,
        Parameters(args): Parameters<SearchArgs>,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let engine = parse_engine(args.engine.as_deref(), Engine::Duckduckgo);
        let opts = self.opts(args.limit.or(Some(10)), args.fetch_content.unwrap_or(false));
        let result = self
            .client
            .search_with_options(&args.query, engine, opts)
            .await
            .map_err(|e| McpError::internal_error(format!("{e}"), None))?;
        Ok(Self::tool_result(
            serde_json::to_string_pretty(&result).unwrap_or_default(),
        ))
    }

    #[tool(
        description = "Try several engines in order and return the first that yields results. Pass a custom engine list or accept the default chain [duckduckgo, wikipedia, bing]. Useful when you do not know which engine covers a query."
    )]
    async fn seia_search_multi(
        &self,
        Parameters(args): Parameters<SearchMultiArgs>,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let names = args
            .engines
            .unwrap_or_else(|| vec!["duckduckgo".into(), "wikipedia".into(), "bing".into()]);
        let engines: Vec<Engine> = names
            .iter()
            .map(|n| parse_engine(Some(n), Engine::Duckduckgo))
            .collect();
        let opts = self.opts(args.limit.or(Some(10)), args.fetch_content.unwrap_or(false));
        // search_fallback takes a plain-engine slice; honour limit/fetch by
        // running the first engine with options then falling back to plain.
        let result = if let Some(&first) = engines.first() {
            match self
                .client
                .search_with_options(&args.query, first, opts)
                .await
            {
                Ok(r) if !r.items.is_empty() => r,
                Ok(_) => self
                    .client
                    .search_fallback(&args.query, &engines[1..])
                    .await
                    .map_err(|e| McpError::internal_error(format!("{e}"), None))?,
                Err(_) => self
                    .client
                    .search_fallback(&args.query, &engines[1..])
                    .await
                    .map_err(|e| McpError::internal_error(format!("{e}"), None))?,
            }
        } else {
            self.client
                .search_fallback(&args.query, &[])
                .await
                .map_err(|e| McpError::internal_error(format!("{e}"), None))?
        };
        Ok(Self::tool_result(
            serde_json::to_string_pretty(&result).unwrap_or_default(),
        ))
    }

    #[tool(
        description = "List the nine search engines, their names (for the engine parameter), and the API-key env var each needs (if any)."
    )]
    async fn seia_list_engines(
        &self,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let engines = ENGINES
            .iter()
            .map(|(name, key_env)| {
                json!({
                    "name": name,
                    "api_key_env": key_env,
                    "needs_key": key_env.is_some(),
                })
            })
            .collect::<Vec<_>>();
        Ok(Self::tool_result(
            serde_json::to_string_pretty(&engines).unwrap_or_default(),
        ))
    }
}

// ── ServerHandler ────────────────────────────────────

#[tool_handler(router = Server::tool_router())]
impl ServerHandler for Server {}

// ── helpers ──────────────────────────────────────────

/// Engine name → variant, falling back to `default` on an unknown/empty name.
fn parse_engine(name: Option<&str>, default: Engine) -> Engine {
    let Some(name) = name.map(str::trim).filter(|s| !s.is_empty()) else {
        return default;
    };
    match name.to_ascii_lowercase().as_str() {
        "duckduckgo" | "ddg" => Engine::Duckduckgo,
        "tavily" => Engine::Tavily,
        "searxng" => Engine::Searxng,
        "wikipedia" | "wiki" => Engine::Wikipedia,
        "bing" => Engine::Bing,
        "brave" => Engine::Brave,
        "zhipu" => Engine::Zhipu,
        "bocha" => Engine::Bocha,
        "metaso" => Engine::Metaso,
        _ => default,
    }
}

/// Static engine roster used by `seia_list_engines` (name → API-key env, if any).
const ENGINES: &[(&str, Option<&str>)] = &[
    ("duckduckgo", None),
    ("wikipedia", None),
    ("searxng", Some("SEARXNG_URL")),
    ("tavily", Some("TAVILY_API_KEY")),
    ("bing", Some("BING_SEARCH_API_KEY")),
    ("brave", Some("BRAVE_SEARCH_API_KEY")),
    ("zhipu", Some("ZHIPU_API_KEY")),
    ("bocha", Some("BOCHA_API_KEY")),
    ("metaso", Some("METASO_API_KEY")),
];

// ── public entry point ───────────────────────────────

pub async fn run() -> Result<()> {
    // Honour HTTPS_PROXY / HTTP_PROXY via the client default; an explicit proxy
    // can be added later via a SEIA_PROXY env if needed.
    let client = match std::env::var("SEIA_PROXY") {
        Ok(p) if !p.is_empty() => SearchClient::with_proxy(&p)?,
        _ => SearchClient::new(),
    };
    let server = Server { client };
    let transport = rmcp::transport::stdio();
    let server_handle = server.serve(transport).await?;
    server_handle.waiting().await?;
    Ok(())
}
