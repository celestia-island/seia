//! `seia serve` — HTTP search service speaking the `seia_search_v1` protocol.
//!
//! Routes:
//!
//! * `POST /v1/search` — execute a caller-supplied provider `chain` with
//!   ordered failover; the first provider that returns results wins.
//! * `GET  /v1/engines` — engine roster (protocol, key env var, default base
//!   URL, capabilities) plus the default chain.
//! * `GET  /health` — unauthenticated liveness probe.
//!
//! Environment:
//!
//! * `SEIA_BIND` — listen address (default `127.0.0.1:3013`).
//! * `SEIA_TOKEN` — when non-empty, every `/v1/*` request must send
//!   `Authorization: Bearer <token>`.
//! * `SEIA_DEFAULT_CHAIN` — comma-separated provider ids used when a request
//!   omits `chain` (default `deepseek,baidu,duckduckgo,wikipedia`).
//!
//! # Security
//!
//! Caller chain entries carry a **plaintext** `api_key` (arona → seia over
//! loopback). That key is only ever handed to the engine request builder: it is
//! never logged, never returned, never included in error/attempt text, and is
//! skipped by `Debug`/`Serialize`. Attempt errors are additionally scrubbed for
//! the exact key string as defence in depth.

use std::{sync::Arc, time::Instant};

use anyhow::{Context as _, Result};
use axum::{
    Json, Router,
    extract::{State, rejection::JsonRejection},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{
    client::{EngineConfig, SearchClient, SearchOptions},
    config::EngineRegistry,
    engines::Engine,
    result::SearchItem,
    utils::truncate,
};

/// Default listen address. Port 3013 is the seia slot on node-3 (3011/3012 are
/// already taken by other demo services there); deployments may override it
/// with `SEIA_BIND` or `--bind`.
pub const DEFAULT_BIND: &str = "127.0.0.1:3013";
/// Default provider chain used when a request omits `chain`.
pub const DEFAULT_CHAIN: &[&str] = &["deepseek", "baidu", "duckduckgo", "wikipedia"];
/// Environment variable overriding [`DEFAULT_BIND`].
pub const ENV_BIND: &str = "SEIA_BIND";
/// Optional bearer token guarding `/v1/*`.
pub const ENV_TOKEN: &str = "SEIA_TOKEN";
/// Environment variable overriding [`DEFAULT_CHAIN`].
pub const ENV_DEFAULT_CHAIN: &str = "SEIA_DEFAULT_CHAIN";
/// Maximum `count` accepted by `POST /v1/search`.
pub const MAX_COUNT: usize = 50;

/// Protocols the service can dispatch, in roster order.
fn known_protocols() -> Vec<&'static str> {
    Engine::BUILTIN
        .iter()
        .map(Engine::protocol)
        .chain(std::iter::once(Engine::CUSTOM_PROTOCOL))
        .collect()
}

/// Resolve a wire protocol to the adapter that executes it.
fn engine_for_protocol(protocol: &str) -> Option<Engine> {
    Engine::BUILTIN
        .iter()
        .find(|engine| engine.protocol() == protocol)
        .cloned()
}

/// Runtime configuration for the service.
pub struct ServeConfig {
    pub client: SearchClient,
    /// When set (non-empty), `/v1/*` requires `Authorization: Bearer <token>`.
    pub token: Option<String>,
    /// Chain used when a request omits `chain`.
    pub default_chain: Vec<ChainEntry>,
}

impl ServeConfig {
    /// Build the configuration from the process environment.
    #[must_use]
    pub fn from_env(client: SearchClient) -> Self {
        let token = std::env::var(ENV_TOKEN)
            .ok()
            .map(|token| token.trim().to_string())
            .filter(|token| !token.is_empty());
        let raw = std::env::var(ENV_DEFAULT_CHAIN).unwrap_or_else(|_| DEFAULT_CHAIN.join(","));
        let default_chain = raw
            .split(',')
            .map(str::trim)
            .filter(|provider| !provider.is_empty())
            .map(|provider| ChainEntry {
                provider: provider.to_string(),
                ..ChainEntry::default()
            })
            .collect();
        Self {
            client,
            token,
            default_chain,
        }
    }
}

/// One provider attempt in a caller-supplied chain.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ChainEntry {
    /// Provider id (engine name or custom engine name).
    pub provider: String,
    /// Wire protocol; optional when `provider` identifies a built-in engine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Endpoint override (base URL, no trailing slash needed).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// Runtime-injected credential. **Never serialized or logged.**
    #[serde(default, skip_serializing)]
    pub api_key: Option<String>,
    /// Engine-specific knobs (e.g. `{"searxng_url": "..."}`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Value>,
}

// Manual Debug: the api key must never reach a log line, panic message or
// `{:?}` dump.
impl std::fmt::Debug for ChainEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChainEntry")
            .field("provider", &self.provider)
            .field("protocol", &self.protocol)
            .field("base_url", &self.base_url)
            .field("api_key", &self.api_key.as_ref().map(|_| "<redacted>"))
            .field("options", &self.options)
            .finish()
    }
}

/// `POST /v1/search` request body.
#[derive(Debug, Default, Deserialize)]
pub struct SearchRequestBody {
    pub query: String,
    #[serde(default)]
    pub count: Option<usize>,
    #[serde(default)]
    pub need_content: Option<bool>,
    #[serde(default)]
    pub ori_question: Option<String>,
    #[serde(default)]
    pub locale: Option<String>,
    #[serde(default)]
    pub chain: Option<Vec<ChainEntry>>,
}

/// One attempt in the failover trace returned to the caller.
#[derive(Debug, Clone, Serialize)]
pub struct Attempt {
    pub provider: String,
    pub ok: bool,
    pub took_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

struct ChainOutcome {
    provider: String,
    engine: Engine,
    protocol: String,
    items: Vec<SearchItem>,
    attempts: Vec<Attempt>,
}

struct ChainFailure {
    message: String,
    attempts: Vec<Attempt>,
}

// ── startup ──────────────────────────────────────────────────

/// Run the HTTP service until the process is stopped.
///
/// # Errors
///
/// Returns `Err` when the listen address cannot be resolved/bound or the
/// server loop fails.
pub async fn run(bind_override: Option<String>) -> Result<()> {
    let bind = resolve_bind(bind_override);
    let cfg = Arc::new(ServeConfig::from_env(load_client()));
    let listener = tokio::net::TcpListener::bind(&bind)
        .await
        .with_context(|| format!("seia serve: cannot bind {bind}"))?;
    let local = listener
        .local_addr()
        .map_or_else(|_| bind.clone(), |addr| addr.to_string());
    tracing::info!(bind = %local, "seia serve listening: POST /v1/search, GET /v1/engines, GET /health");
    axum::serve(listener, router(cfg))
        .await
        .context("seia serve failed")?;
    Ok(())
}

fn resolve_bind(override_bind: Option<String>) -> String {
    override_bind
        .map(|bind| bind.trim().to_string())
        .filter(|bind| !bind.is_empty())
        .or_else(|| {
            std::env::var(ENV_BIND)
                .ok()
                .map(|bind| bind.trim().to_string())
                .filter(|bind| !bind.is_empty())
        })
        .unwrap_or_else(|| DEFAULT_BIND.to_string())
}

/// Build the service's search client.
///
/// Custom engines are loaded from the same config paths as `seia search` and
/// `seia mcp` (`~/.seia/engines/*.toml`, `./seia.toml`), so chain entries using
/// the `seia_custom_v0` protocol resolve instead of failing as unknown.
fn load_client() -> SearchClient {
    match EngineRegistry::load() {
        Ok(registry) => build_client(registry),
        Err(err) => {
            // Never fail startup over an unreadable user config, but make the
            // degraded state visible: chain entries for custom engines will
            // otherwise fail as unknown providers.
            tracing::warn!(
                error = %err,
                "custom engine registry unavailable; serving built-in engines only"
            );
            build_client(EngineRegistry::default())
        }
    }
}

/// Build a search client over an explicit engine registry (test seam).
fn build_client(registry: EngineRegistry) -> SearchClient {
    SearchClient::new().with_registry(registry)
}

/// Build the axum router (exposed for tests).
pub fn router(cfg: Arc<ServeConfig>) -> Router {
    Router::new()
        .route("/v1/search", post(search_handler))
        .route("/v1/engines", get(engines_handler))
        .route("/health", get(health_handler))
        .with_state(cfg)
}

// ── handlers ─────────────────────────────────────────────────

async fn search_handler(
    State(cfg): State<Arc<ServeConfig>>,
    headers: HeaderMap,
    body: Result<Json<SearchRequestBody>, JsonRejection>,
) -> Response {
    if let Some(unauthorized) = check_auth(&cfg, &headers) {
        return unauthorized;
    }

    let Json(request) = match body {
        Ok(body) => body,
        Err(rejection) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "bad_request",
                rejection.body_text(),
            );
        }
    };

    let count = match validate_request(&request) {
        Ok(count) => count,
        Err(message) => return error_response(StatusCode::BAD_REQUEST, "bad_request", message),
    };

    let chain = match request.chain.as_deref() {
        Some(chain) if !chain.is_empty() => chain.to_vec(),
        _ => cfg.default_chain.clone(),
    };

    let started = Instant::now();
    match run_chain(&cfg.client, &request, count, &chain).await {
        Ok(outcome) => {
            // Served-request evidence for the caller side (arona → seia → upstream).
            // NEVER log the query text or any credential: only lengths + labels.
            let elapsed = elapsed_ms(started);
            tracing::info!(
                target: "seia::serve",
                event = "search_request",
                query_len = request.query.chars().count(),
                chain_len = chain.len(),
                provider = %outcome.provider,
                engine = %outcome.engine,
                count = outcome.items.len(),
                took_ms = elapsed,
                "seia_search served"
            );
            let body = json!({
                "query": request.query,
                "provider": outcome.provider,
                "engine": outcome.engine.as_str(),
                "protocol": outcome.protocol,
                "count": outcome.items.len(),
                "took_ms": elapsed,
                "results": outcome.items,
                "attempts": outcome.attempts,
            });
            (StatusCode::OK, Json(body)).into_response()
        }
        Err(failure) => {
            tracing::warn!(
                target: "seia::serve",
                event = "search_failed",
                query_len = request.query.chars().count(),
                chain_len = chain.len(),
                attempts = failure.attempts.len(),
                took_ms = elapsed_ms(started),
                "seia_search failed"
            );
            error_response_with_attempts(
                StatusCode::BAD_GATEWAY,
                "search_failed",
                failure.message,
                failure.attempts,
            )
        }
    }
}

async fn engines_handler(State(cfg): State<Arc<ServeConfig>>, headers: HeaderMap) -> Response {
    if let Some(unauthorized) = check_auth(&cfg, &headers) {
        return unauthorized;
    }

    let engines: Vec<Value> = Engine::BUILTIN
        .iter()
        .map(|engine| {
            json!({
                "id": engine.as_str(),
                "protocol": engine.protocol(),
                "needs_key": engine.needs_key(),
                "env_var": engine.api_key_env(),
                "default_base_url": engine.default_base_url(),
                "capabilities": engine.capabilities(),
            })
        })
        .collect();

    Json(json!({
        "engines": engines,
        // api_key is skip_serializing on ChainEntry; default entries have none.
        "default_chain": cfg.default_chain,
    }))
    .into_response()
}

async fn health_handler() -> Response {
    Json(json!({
        "ok": true,
        "service": "seia",
        "version": env!("CARGO_PKG_VERSION"),
    }))
    .into_response()
}

// ── auth ─────────────────────────────────────────────────────

fn check_auth(cfg: &ServeConfig, headers: &HeaderMap) -> Option<Response> {
    let expected = cfg
        .token
        .as_deref()
        .map(str::trim)
        .filter(|token| !token.is_empty())?;

    let presented = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    let presented = presented
        .strip_prefix("Bearer ")
        .or_else(|| presented.strip_prefix("bearer "))
        .map(str::trim)
        .unwrap_or_default();

    if constant_time_eq(presented.as_bytes(), expected.as_bytes()) {
        None
    } else {
        Some(error_response(
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "missing or invalid bearer token (Authorization: Bearer $SEIA_TOKEN)",
        ))
    }
}

/// Length-checked byte comparison; no early exit on the first differing byte.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (left, right) in a.iter().zip(b) {
        diff |= left ^ right;
    }
    diff == 0
}

// ── validation / failover ────────────────────────────────────

fn validate_request(request: &SearchRequestBody) -> Result<usize, String> {
    if request.query.trim().is_empty() {
        return Err("'query' must not be empty".to_string());
    }
    let count = request.count.unwrap_or(10);
    if !(1..=MAX_COUNT).contains(&count) {
        return Err(format!(
            "'count' must be between 1 and {MAX_COUNT} (got {count})"
        ));
    }
    Ok(count)
}

/// Resolve a chain entry to a concrete engine plus the protocol label reported
/// back to the caller.
///
/// Dispatch is **protocol-first** (the registry-declared protocol names, e.g.
/// `deepseek_search_v1` / `tavily_search_v1`). The provider id is a label and,
/// when no protocol is supplied, a fallback lookup by built-in engine name or
/// by custom-engine config name.
fn resolve_entry(entry: &ChainEntry, client: &SearchClient) -> Result<(Engine, String), String> {
    let provider = entry.provider.trim();
    if provider.is_empty() {
        return Err("chain entry is missing 'provider'".to_string());
    }
    let protocol = entry
        .protocol
        .as_deref()
        .map(str::trim)
        .filter(|protocol| !protocol.is_empty());

    let by_provider = Engine::from_name(provider).or_else(|| {
        client
            .registry()
            .get(provider)
            .map(|_| Engine::Custom(provider.to_string()))
    });

    match protocol {
        Some(protocol) => {
            let engine = if protocol == Engine::CUSTOM_PROTOCOL {
                by_provider
                    .as_ref()
                    .ok_or_else(|| {
                        format!("unknown custom engine '{provider}' (protocol '{protocol}')")
                    })?
                    .clone()
            } else {
                engine_for_protocol(protocol).ok_or_else(|| {
                    format!(
                        "unsupported_protocol: '{protocol}' (provider '{provider}'); supported protocols: {}",
                        known_protocols().join(", ")
                    )
                })?
            };
            // A provider that names a *different* built-in engine is a
            // configuration mistake worth surfacing instead of silently
            // executing the protocol.
            if let Some(by_provider) = by_provider {
                if by_provider != engine {
                    return Err(format!(
                        "protocol '{protocol}' does not match provider '{provider}' (protocol implies '{}')",
                        engine.as_str()
                    ));
                }
            }
            Ok((engine, protocol.to_string()))
        }
        None => {
            let engine = by_provider.ok_or_else(|| {
                format!(
                    "unknown provider '{provider}'; supported protocols: {}",
                    known_protocols().join(", ")
                )
            })?;
            let protocol = engine.protocol().to_string();
            Ok((engine, protocol))
        }
    }
}

async fn run_chain(
    client: &SearchClient,
    request: &SearchRequestBody,
    count: usize,
    chain: &[ChainEntry],
) -> Result<ChainOutcome, ChainFailure> {
    let need_content = request.need_content.unwrap_or(false);
    let mut attempts: Vec<Attempt> = Vec::with_capacity(chain.len());

    for entry in chain {
        let provider = entry.provider.trim().to_string();
        let started = Instant::now();

        let error: String = match resolve_entry(entry, client) {
            Ok((engine, protocol)) => {
                let opts = SearchOptions {
                    limit: Some(count),
                    fetch_content: need_content,
                    searxng_url: searxng_url(entry),
                };
                let cfg = EngineConfig {
                    base_url: entry.base_url.clone(),
                    api_key: entry.api_key.clone(),
                    ori_question: request.ori_question.clone(),
                    locale: request.locale.clone(),
                };
                match client
                    .search_with_config(&request.query, engine.clone(), opts, cfg)
                    .await
                {
                    Ok(result) if !result.items.is_empty() => {
                        attempts.push(Attempt {
                            provider: provider.clone(),
                            ok: true,
                            took_ms: elapsed_ms(started),
                            error: None,
                        });
                        return Ok(ChainOutcome {
                            provider,
                            engine,
                            protocol,
                            items: result.items,
                            attempts,
                        });
                    }
                    Ok(_) => "empty results".to_string(),
                    // `{:#}` includes the anyhow context chain (root cause),
                    // e.g. "request to <url> failed: connection refused".
                    Err(err) => format!("{err:#}"),
                }
            }
            Err(message) => message,
        };

        attempts.push(Attempt {
            provider: provider.clone(),
            ok: false,
            took_ms: elapsed_ms(started),
            error: Some(scrub(error, entry.api_key.as_deref())),
        });
        tracing::warn!(
            provider = %provider,
            error = %attempts.last().and_then(|attempt| attempt.error.as_deref()).unwrap_or("unknown"),
            "search provider attempt failed"
        );
    }

    let last_error = attempts
        .last()
        .and_then(|attempt| attempt.error.clone())
        .unwrap_or_else(|| "no providers configured".to_string());
    Err(ChainFailure {
        message: format!("all providers failed; last error: {last_error}"),
        attempts,
    })
}

fn searxng_url(entry: &ChainEntry) -> Option<String> {
    entry
        .options
        .as_ref()
        .and_then(|options| options.get("searxng_url"))
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| {
            std::env::var("SEARXNG_URL")
                .ok()
                .filter(|url| !url.is_empty())
        })
}

/// Defence in depth: strip the exact credential from any error text.
fn scrub(message: impl Into<String>, secret: Option<&str>) -> String {
    let mut message = truncate(&message.into(), 600);
    if let Some(secret) = secret.map(str::trim).filter(|secret| !secret.is_empty()) {
        if message.contains(secret) {
            message = message.replace(secret, "<redacted>");
        }
    }
    message
}

fn elapsed_ms(started: Instant) -> u64 {
    u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX)
}

// ── responses ────────────────────────────────────────────────

fn error_body(code: &str, message: impl Into<String>, attempts: Vec<Attempt>) -> Value {
    let mut error = serde_json::Map::new();
    error.insert("code".to_string(), json!(code));
    error.insert("message".to_string(), json!(message.into()));
    if !attempts.is_empty() {
        error.insert("attempts".to_string(), json!(attempts));
    }
    json!({ "error": error })
}

fn error_response(status: StatusCode, code: &str, message: impl Into<String>) -> Response {
    (status, Json(error_body(code, message, Vec::new()))).into_response()
}

fn error_response_with_attempts(
    status: StatusCode,
    code: &str,
    message: impl Into<String>,
    attempts: Vec<Attempt>,
) -> Response {
    (status, Json(error_body(code, message, attempts))).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::SearchClient,
        test_support::{MockServer, http_client},
    };

    const DEEPSEEK_OK: &str = r#"{
        "results": [
            {
                "rerank_score": 2.0,
                "title": "翁法罗斯 - 米游社",
                "url": "https://example.org/amphoreus",
                "snippet": "短摘要",
                "summary": "摘要",
                "site_name": "米游社",
                "date": "2026-10-01"
            }
        ]
    }"#;

    /// Spawn the router on an ephemeral loopback port and return its base URL.
    async fn start(cfg: ServeConfig) -> String {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind test server");
        let addr = listener.local_addr().expect("test server addr");
        let app = router(Arc::new(cfg));
        tokio::spawn(async move {
            axum::serve(listener, app).await.expect("test server");
        });
        format!("http://{addr}")
    }

    fn test_config(token: Option<&str>, default_chain: Vec<ChainEntry>) -> ServeConfig {
        ServeConfig {
            client: SearchClient::without_proxy(),
            token: token.map(str::to_string),
            default_chain,
        }
    }

    fn entry(provider: &str, base_url: &str, api_key: Option<&str>) -> ChainEntry {
        ChainEntry {
            provider: provider.to_string(),
            base_url: Some(base_url.to_string()),
            api_key: api_key.map(str::to_string),
            ..ChainEntry::default()
        }
    }

    async fn post_search(base_url: &str, token: Option<&str>, body: Value) -> (u16, Value) {
        let mut request = http_client()
            .post(format!("{base_url}/v1/search"))
            .json(&body);
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        let response = request.send().await.expect("search request");
        let status = response.status().as_u16();
        let body: Value = response.json().await.expect("search response json");
        (status, body)
    }

    async fn get_engines(base_url: &str, token: Option<&str>) -> (u16, Value) {
        let mut request = http_client().get(format!("{base_url}/v1/engines"));
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        let response = request.send().await.expect("engines request");
        let status = response.status().as_u16();
        let body: Value = response.json().await.expect("engines response json");
        (status, body)
    }

    #[tokio::test]
    async fn fails_over_in_chain_order() {
        let failing = MockServer::spawn(vec![(500, r#"{"error":"upstream down"}"#)]).await;
        let healthy = MockServer::spawn_ok(DEEPSEEK_OK).await;
        let cfg = test_config(
            None,
            vec![
                entry("deepseek", &failing.base_url, Some("sk-fake-chain-key")),
                entry("deepseek", &healthy.base_url, Some("sk-fake-chain-key")),
            ],
        );
        let base_url = start(cfg).await;

        let (status, body) =
            post_search(&base_url, None, json!({"query": "翁法罗斯", "count": 3})).await;

        assert_eq!(status, 200, "{body}");
        assert_eq!(body["query"], "翁法罗斯");
        assert_eq!(body["provider"], "deepseek");
        assert_eq!(body["engine"], "deepseek");
        assert_eq!(body["protocol"], "deepseek_search_v1");
        assert_eq!(body["count"], 1);
        assert_eq!(body["results"][0]["title"], "翁法罗斯 - 米游社");
        assert_eq!(body["results"][0]["engine"], "deepseek");
        let attempts = body["attempts"].as_array().expect("attempts array");
        assert_eq!(attempts.len(), 2);
        assert_eq!(attempts[0]["provider"], "deepseek");
        assert_eq!(attempts[0]["ok"], false);
        assert!(attempts[1]["ok"].as_bool().unwrap());
        assert!(attempts[1].get("error").is_none());

        // The first provider really was tried first, with the requested count.
        assert_eq!(failing.last_request().json()["count"], 3);
        assert_eq!(healthy.last_request().json()["count"], 3);

        // Lead requirement: the plaintext chain key must not surface anywhere in
        // the response (attempts, error text, or otherwise).
        let serialized = body.to_string();
        assert!(
            !serialized.contains("sk-fake-chain-key"),
            "api key leaked into response: {serialized}"
        );
    }

    #[tokio::test]
    async fn bearer_auth_guards_v1_routes_when_configured() {
        let healthy = MockServer::spawn_ok(DEEPSEEK_OK).await;
        let cfg = test_config(
            Some("test-service-token"),
            vec![entry("deepseek", &healthy.base_url, Some("sk-fake"))],
        );
        let base_url = start(cfg).await;

        // No token → 401 on both /v1 routes.
        let (status, body) = post_search(&base_url, None, json!({"query": "q"})).await;
        assert_eq!(status, 401);
        assert_eq!(body["error"]["code"], "unauthorized");
        let (status, _) = get_engines(&base_url, None).await;
        assert_eq!(status, 401);

        // Wrong token → 401.
        let (status, _) = post_search(&base_url, Some("wrong"), json!({"query": "q"})).await;
        assert_eq!(status, 401);
        let (status, _) = get_engines(&base_url, Some("wrong")).await;
        assert_eq!(status, 401);

        // Correct token → 200.
        let (status, body) =
            post_search(&base_url, Some("test-service-token"), json!({"query": "q"})).await;
        assert_eq!(status, 200, "{body}");
        let (status, body) = get_engines(&base_url, Some("test-service-token")).await;
        assert_eq!(status, 200, "{body}");
    }

    #[tokio::test]
    async fn bad_requests_are_400() {
        let base_url = start(test_config(None, Vec::new())).await;

        let (status, body) = post_search(&base_url, None, json!({"query": "   "})).await;
        assert_eq!(status, 400);
        assert_eq!(body["error"]["code"], "bad_request");

        let (status, body) = post_search(&base_url, None, json!({"query": "q", "count": 0})).await;
        assert_eq!(status, 400);
        assert_eq!(body["error"]["code"], "bad_request");

        let (status, _body) =
            post_search(&base_url, None, json!({"query": "q", "count": 51})).await;
        assert_eq!(status, 400);

        // Malformed JSON body.
        let response = http_client()
            .post(format!("{base_url}/v1/search"))
            .header("content-type", "application/json")
            .body("{not json")
            .send()
            .await
            .expect("malformed request");
        assert_eq!(response.status().as_u16(), 400);
        let body: Value = response.json().await.expect("error json");
        assert_eq!(body["error"]["code"], "bad_request");
    }

    #[tokio::test]
    async fn unsupported_protocol_fails_the_attempt_and_continues() {
        let healthy = MockServer::spawn_ok(DEEPSEEK_OK).await;
        let base_url = start(test_config(None, Vec::new())).await;

        let (status, body) = post_search(
            &base_url,
            None,
            json!({
                "query": "翁法罗斯",
                "chain": [
                    {"provider": "future", "protocol": "future_search_v1", "base_url": "http://127.0.0.1:1"},
                    {"provider": "deepseek", "protocol": "deepseek_search_v1",
                     "base_url": healthy.base_url, "api_key": "sk-fake"}
                ]
            }),
        )
        .await;

        assert_eq!(status, 200, "{body}");
        assert_eq!(body["provider"], "deepseek");
        assert_eq!(body["attempts"].as_array().unwrap().len(), 2);
        let first_error = body["attempts"][0]["error"].as_str().expect("error");
        assert!(
            first_error.starts_with("unsupported_protocol"),
            "{first_error}"
        );
        assert_eq!(body["attempts"][1]["ok"], true);
    }

    #[tokio::test]
    async fn chain_keys_are_injected_with_each_providers_auth_style() {
        // Registry auth styles: tavily = Bearer, brave = X-Subscription-Token,
        // bing = Ocp-Apim-Subscription-Key.
        let tavily = MockServer::spawn_ok(
            r#"{"results":[{"title":"T","url":"https://example.org/t","content":"c"}]}"#,
        )
        .await;
        let brave = MockServer::spawn_ok(
            r#"{"web":{"results":[{"title":"B","url":"https://example.org/b","description":"d"}]}}"#,
        )
        .await;
        let bing = MockServer::spawn_ok(
            r#"{"webPages":{"value":[{"name":"Bing","url":"https://example.org/bing","snippet":"s"}]}}"#,
        )
        .await;

        let cases = [
            (
                "tavily",
                "tavily_search_v1",
                &tavily,
                "authorization",
                "Bearer sk-fake-tavily",
            ),
            (
                "brave",
                "brave_search_v1",
                &brave,
                "x-subscription-token",
                "sk-fake-brave",
            ),
            (
                "bing",
                "bing_search_v1",
                &bing,
                "ocp-apim-subscription-key",
                "sk-fake-bing",
            ),
        ];
        for (provider, protocol, mock, header, expected) in cases {
            let key = expected.rsplit(' ').next().unwrap_or(expected);
            let base_url = start(test_config(None, Vec::new())).await;
            let (status, body) = post_search(
                &base_url,
                None,
                json!({
                    "query": "翁法罗斯",
                    "chain": [{
                        "provider": provider,
                        "protocol": protocol,
                        "base_url": mock.base_url.as_str(),
                        "api_key": key,
                    }]
                }),
            )
            .await;
            assert_eq!(status, 200, "{provider}: {body}");
            assert_eq!(body["engine"], provider);
            assert_eq!(body["protocol"], protocol);
            assert_eq!(
                mock.last_request().header(header),
                Some(expected),
                "{provider} did not receive its registry auth style"
            );
        }
    }

    #[tokio::test]
    async fn all_providers_failing_is_502_with_attempts() {
        let failing = MockServer::spawn(vec![(500, r#"{"error":"down"}"#)]).await;
        let cfg = test_config(
            None,
            vec![entry(
                "deepseek",
                &failing.base_url,
                Some("sk-fake-chain-key"),
            )],
        );
        let base_url = start(cfg).await;

        let (status, body) = post_search(&base_url, None, json!({"query": "q"})).await;
        assert_eq!(status, 502);
        assert_eq!(body["error"]["code"], "search_failed");
        let message = body["error"]["message"].as_str().expect("message");
        assert!(message.contains("all providers failed"), "{message}");
        let attempts = body["error"]["attempts"].as_array().expect("attempts");
        assert_eq!(attempts.len(), 1);
        assert_eq!(attempts[0]["provider"], "deepseek");
        assert_eq!(attempts[0]["ok"], false);
        assert!(!body.to_string().contains("sk-fake-chain-key"));
    }

    #[tokio::test]
    async fn default_chain_is_used_when_request_omits_it() {
        let healthy = MockServer::spawn_ok(DEEPSEEK_OK).await;
        let cfg = test_config(
            None,
            vec![entry("deepseek", &healthy.base_url, Some("sk-fake"))],
        );
        let base_url = start(cfg).await;

        let (status, body) = post_search(&base_url, None, json!({"query": "翁法罗斯"})).await;
        assert_eq!(status, 200, "{body}");
        assert_eq!(body["provider"], "deepseek");
    }

    #[tokio::test]
    async fn engines_endpoint_reports_the_roster() {
        let base_url = start(test_config(None, Vec::new())).await;
        let (status, body) = get_engines(&base_url, None).await;
        assert_eq!(status, 200);

        let engines = body["engines"].as_array().expect("engines array");
        let by_id = |id: &str| -> Value {
            engines
                .iter()
                .find(|engine| engine["id"] == id)
                .unwrap_or_else(|| panic!("engine '{id}' missing from roster"))
                .clone()
        };

        let deepseek = by_id("deepseek");
        assert_eq!(deepseek["protocol"], "deepseek_search_v1");
        assert_eq!(deepseek["needs_key"], true);
        assert_eq!(deepseek["env_var"], "DEEPSEEK_API_KEY");
        assert_eq!(deepseek["default_base_url"], "https://api.deepseek.com");
        assert_eq!(deepseek["capabilities"]["content"], true);

        let baidu = by_id("baidu");
        assert_eq!(baidu["protocol"], "baidu_ai_search_v1");
        assert_eq!(baidu["needs_key"], true);
        assert_eq!(baidu["env_var"], "BAIDU_API_KEY");
        assert_eq!(baidu["default_base_url"], "https://qianfan.baidubce.com");

        // Key-less engines stay key-less and advertise their protocol.
        let wikipedia = by_id("wikipedia");
        assert_eq!(wikipedia["protocol"], "wikipedia_search_v1");
        assert_eq!(wikipedia["needs_key"], false);
        assert!(wikipedia["env_var"].is_null());

        // Default base URLs are host-only: the endpoint path lives in the
        // protocol constant (regression guard for the doubled-prefix 404s).
        assert_eq!(
            by_id("bing")["default_base_url"],
            "https://api.bing.microsoft.com"
        );
        assert_eq!(
            by_id("brave")["default_base_url"],
            "https://api.search.brave.com"
        );
        assert_eq!(
            by_id("zhipu")["default_base_url"],
            "https://open.bigmodel.cn"
        );
        assert_eq!(by_id("metaso")["default_base_url"], "https://metaso.cn");
        assert_eq!(
            by_id("tavily")["default_base_url"],
            "https://api.tavily.com"
        );
        assert_eq!(
            by_id("bocha")["default_base_url"],
            "https://api.bochaai.com"
        );

        // Registry-declared protocol names are used verbatim.
        assert_eq!(by_id("tavily")["protocol"], "tavily_search_v1");
        assert_eq!(by_id("brave")["protocol"], "brave_search_v1");
        assert_eq!(by_id("bing")["protocol"], "bing_search_v1");
        assert_eq!(by_id("bocha")["protocol"], "bocha_search_v1");
        assert_eq!(by_id("metaso")["protocol"], "metaso_search_v1");
        assert_eq!(by_id("zhipu")["protocol"], "zhipu_search_v1");
        assert_eq!(by_id("searxng")["protocol"], "searxng_search_v1");
        assert_eq!(by_id("duckduckgo")["protocol"], "duckduckgo_search_v1");
    }

    #[tokio::test]
    async fn unknown_provider_is_diagnosed_in_attempts() {
        let base_url = start(test_config(None, Vec::new())).await;
        let (status, body) = post_search(
            &base_url,
            None,
            json!({"query": "q", "chain": [{"provider": "nope", "base_url": "http://127.0.0.1:1"}]}),
        )
        .await;
        assert_eq!(status, 502);
        let error = body["error"]["attempts"][0]["error"]
            .as_str()
            .expect("attempt error");
        assert!(error.contains("unknown provider"), "{error}");
        assert!(error.contains("deepseek_search_v1"), "{error}");
    }

    #[tokio::test]
    async fn health_is_open_even_when_a_token_is_set() {
        let base_url = start(test_config(Some("test-service-token"), Vec::new())).await;
        let response = http_client()
            .get(format!("{base_url}/health"))
            .send()
            .await
            .expect("health request");
        assert_eq!(response.status().as_u16(), 200);
        let body: Value = response.json().await.expect("health json");
        assert_eq!(body["ok"], true);
        assert_eq!(body["service"], "seia");
    }

    // ── unit tests: credential hygiene & protocol resolution ──

    #[test]
    fn serve_client_loads_the_custom_engine_registry() {
        // The service path must not silently run on an empty registry: custom
        // engines (`seia_custom_v0`) have to resolve like they do for the CLI
        // and MCP paths.
        let mut registry = EngineRegistry::default();
        registry.engines.insert(
            "custom-test".to_string(),
            crate::config::CustomEngineDef {
                label: "Custom Test".to_string(),
                method: "GET".to_string(),
                url: "https://example.org/search".to_string(),
                ..crate::config::CustomEngineDef::default()
            },
        );

        let client = build_client(registry);
        assert_eq!(client.registry().names(), vec!["custom-test"]);

        let entry = ChainEntry {
            provider: "custom-test".to_string(),
            protocol: Some(Engine::CUSTOM_PROTOCOL.to_string()),
            ..ChainEntry::default()
        };
        let (engine, protocol) =
            resolve_entry(&entry, &client).expect("custom engine must resolve in serve");
        assert_eq!(engine, Engine::Custom("custom-test".to_string()));
        assert_eq!(protocol, "seia_custom_v0");

        // `load_client()` is the wiring used by `run()`; it must be the same
        // code path (build_client over EngineRegistry::load()).
        assert_eq!(
            load_client().registry().names().len(),
            EngineRegistry::load().unwrap_or_default().names().len()
        );
    }

    #[test]
    fn chain_entry_debug_and_json_redact_api_key() {
        let chain_entry = entry(
            "deepseek",
            "https://api.deepseek.com",
            Some("sk-very-secret"),
        );
        let debug = format!("{chain_entry:?}");
        assert!(
            !debug.contains("sk-very-secret"),
            "Debug leaked key: {debug}"
        );
        assert!(debug.contains("<redacted>"));
        let json = serde_json::to_string(&chain_entry).expect("serialize chain entry");
        assert!(
            !json.contains("sk-very-secret"),
            "Serialize leaked key: {json}"
        );
        assert!(json.contains("deepseek"));
    }

    #[test]
    fn protocol_resolution_maps_providers_and_rejects_mismatches() {
        let client = SearchClient::without_proxy();
        let cases: [(&str, Option<&str>, &str); 5] = [
            ("deepseek", None, "deepseek_search_v1"),
            ("baidu", Some("baidu_ai_search_v1"), "baidu_ai_search_v1"),
            ("wikipedia", None, "wikipedia_search_v1"),
            ("tavily", Some("tavily_search_v1"), "tavily_search_v1"),
            ("duckduckgo", None, "duckduckgo_search_v1"),
        ];
        for (provider, protocol, expected) in cases {
            let chain_entry = ChainEntry {
                provider: provider.to_string(),
                protocol: protocol.map(str::to_string),
                ..ChainEntry::default()
            };
            let (engine, resolved) =
                resolve_entry(&chain_entry, &client).expect("protocol resolution");
            assert_eq!(engine.as_str(), provider);
            assert_eq!(resolved, expected);
        }

        // baidu provider + deepseek protocol → configuration error.
        let mismatched = ChainEntry {
            provider: "baidu".to_string(),
            protocol: Some(crate::engines_impl::deepseek::PROTOCOL.to_string()),
            ..ChainEntry::default()
        };
        let error = resolve_entry(&mismatched, &client).unwrap_err();
        assert!(error.contains("does not match provider"), "{error}");

        // Protocol names are dispatched verbatim: an unknown one is reported as
        // `unsupported_protocol` (never a panic or a 500).
        let unknown_protocol = ChainEntry {
            provider: "deepseek".to_string(),
            protocol: Some("made_up_v9".to_string()),
            ..ChainEntry::default()
        };
        let error = resolve_entry(&unknown_protocol, &client).unwrap_err();
        assert!(error.starts_with("unsupported_protocol"), "{error}");
    }

    #[test]
    fn scrub_removes_the_credential_from_error_text() {
        let scrubbed = scrub("boom: sk-secret-value leaked", Some("sk-secret-value"));
        assert!(!scrubbed.contains("sk-secret-value"), "{scrubbed}");
        assert!(scrubbed.contains("<redacted>"));
        // No secret → text is only truncated.
        assert_eq!(scrub("plain error", None), "plain error");
    }

    #[test]
    fn default_bind_is_the_node3_seia_slot() {
        assert_eq!(DEFAULT_BIND, "127.0.0.1:3013");
        assert_eq!(DEFAULT_CHAIN[0], "deepseek");
    }
}
