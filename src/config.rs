//! Custom engine configuration — TOML-based engine definitions.
//!
//! Users define custom search engines in `~/.seia/engines.toml` (or
//! `./seia.toml` for project-local). Each entry describes an HTTP endpoint,
//! request construction rules, and response mapping via JSONPath.
//!
//! # Example
//!
//! ```toml
//! [engines.github]
//! label = "GitHub Code Search"
//! method = "GET"
//! url = "https://api.github.com/search/code"
//! query_param = "q"
//! headers = { Authorization = "Bearer ${GITHUB_TOKEN}", Accept = "application/vnd.github.v3+json" }
//! result_path = "$.items[*]"
//! title_field = "name"
//! url_field = "html_url"
//! snippet_field = "repository.full_name"
//! ```

use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};

const BUILTIN_TOML: &str = include_str!("builtin_engines.toml");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    #[serde(default)]
    pub engines: HashMap<String, CustomEngineDef>,
}

/// A user-defined search engine specification.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomEngineDef {
    /// Human-readable display name.
    pub label: String,

    /// HTTP method: "GET" or "POST". Defaults to "GET".
    #[serde(default = "default_method")]
    pub method: String,

    /// The endpoint URL. Supports template variables:
    /// - `{{query}}` — URL-encoded search query
    /// - `{{limit}}` — max results requested
    /// - `${ENV_VAR}` — environment variable substitution
    ///
    /// Defaults to empty for Rust-backed engines (whose TOML entry is metadata
    /// only and carries no endpoint), so one missing `url` can no longer make
    /// the whole embedded roster fail to load.
    #[serde(default)]
    pub url: String,

    /// For GET requests: the query-string parameter name for the search term.
    /// When set, `?<param>=<query>` is appended to the URL.
    #[serde(default)]
    pub query_param: Option<String>,

    /// For POST requests: JSON body template. Supports the same template
    /// variables as `url`.
    #[serde(default)]
    pub body_template: Option<String>,

    /// Extra HTTP headers. Values support `${ENV_VAR}` substitution.
    #[serde(default)]
    pub headers: HashMap<String, String>,

    /// JSONPath expression pointing to the array of result items in the
    /// API response (e.g. `$.data.items[*]` or `$.results`). When absent,
    /// the root object is treated as the result list — useful when the
    /// response is already a JSON array.
    #[serde(default)]
    pub result_path: Option<String>,

    /// Field name (or JSONPath relative to each result item) for the title.
    /// Simple dot-notation is supported (e.g. `repository.full_name`).
    #[serde(default = "default_title_field")]
    pub title_field: String,

    /// Field name for the URL.
    #[serde(default = "default_url_field")]
    pub url_field: String,

    /// Optional field name for the snippet/description.
    #[serde(default)]
    pub snippet_field: Option<String>,

    /// Pre-request JavaScript snippet (requires `pre-request-script` feature).
    /// The script runs before the HTTP request is sent and can mutate:
    /// - `req.url` (string)
    /// - `req.method` (string)
    /// - `req.headers` (object)
    /// - `req.body` (string, for POST)
    #[serde(default)]
    pub pre_request: Option<String>,

    /// Optional per-page limit parameter name (e.g. "page_size"). When
    /// provided, `?<param>=<limit>` is appended to GET requests.
    #[serde(default)]
    pub limit_param: Option<String>,

    /// Whether this is a built-in engine (metadata only, for display).
    #[serde(default)]
    pub builtin: bool,

    /// One-line help text shown in `seia engines`.
    #[serde(default)]
    pub help: Option<String>,
}

fn default_method() -> String {
    "GET".into()
}

fn default_title_field() -> String {
    "title".into()
}

fn default_url_field() -> String {
    "url".into()
}

impl CustomEngineDef {
    /// Render a template string by substituting well-known variables.
    ///
    /// Supported placeholders:
    /// - `{{query}}` → the URL-encoded search term
    /// - `{{limit}}` → the max-results count
    /// - `${NAME}`  → value of environment variable `NAME`
    pub fn render(&self, tmpl: &str, query: &str, limit: usize) -> String {
        let mut out = tmpl.to_string();

        out = out.replace("{{query}}", &crate::utils::urlencode_query(query));
        out = out.replace("{{limit}}", &limit.to_string());

        let mut start = 0;
        while let Some(dollar) = out[start..].find("${") {
            let abs = start + dollar;
            if let Some(end) = out[abs + 2..].find('}') {
                let var_name = &out[abs + 2..abs + 2 + end];
                let val = std::env::var(var_name).unwrap_or_default();
                out.replace_range(abs..=abs + 2 + end, &val);
                start = abs + val.len();
            } else {
                break;
            }
        }

        out
    }
}

/// Resolved engine registry loaded from config files.
#[derive(Debug, Clone, Default)]
pub struct EngineRegistry {
    pub engines: HashMap<String, CustomEngineDef>,
}

impl EngineRegistry {
    /// Load configs in priority order (lowest to highest):
    /// 1. Embedded built-in definitions
    /// 2. `~/.seia/engines/*.toml` (user overrides)
    /// 3. `./seia.toml` (project-local)
    pub fn load() -> Result<Self> {
        let mut registry = Self::default();

        let builtin: ConfigFile =
            toml::from_str(BUILTIN_TOML).context("parsing embedded builtin_engines.toml")?;
        registry.merge(builtin);

        if let Some(home) = dirs::home_dir() {
            let user_dir = home.join(".seia").join("engines");
            if user_dir.is_dir() {
                if let Ok(entries) = std::fs::read_dir(&user_dir) {
                    for entry in entries.filter_map(|e| e.ok()) {
                        let path = entry.path();
                        if path.extension().is_some_and(|e| e == "toml") {
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                if let Ok(cfg) = toml::from_str::<ConfigFile>(&content) {
                                    registry.merge(cfg);
                                }
                            }
                        }
                    }
                }
            }
        }

        let local_config = PathBuf::from("seia.toml");
        if local_config.is_file() {
            let content = std::fs::read_to_string(&local_config).context("reading ./seia.toml")?;
            let cfg: ConfigFile = toml::from_str(&content).context("parsing ./seia.toml")?;
            registry.merge(cfg);
        }

        Ok(registry)
    }

    /// Write built-in engine definitions to `~/.seia/engines/`.
    /// Does not overwrite existing files unless `force` is true.
    pub fn install(force: bool) -> Result<PathBuf> {
        let dir = dirs::home_dir()
            .map(|h| h.join(".seia").join("engines"))
            .ok_or_else(|| anyhow::anyhow!("cannot determine home directory"))?;
        std::fs::create_dir_all(&dir)?;

        let builtin: ConfigFile =
            toml::from_str(BUILTIN_TOML).context("parsing embedded builtin_engines.toml")?;

        for (name, def) in &builtin.engines {
            let path = dir.join(format!("builtin.{name}.toml"));
            if path.exists() && !force {
                continue;
            }
            let single = ConfigFile {
                engines: {
                    let mut m = HashMap::new();
                    m.insert(name.clone(), def.clone());
                    m
                },
            };
            std::fs::write(&path, toml::to_string_pretty(&single)?)?;
        }

        Ok(dir)
    }

    /// Re-install built-in engine configs, overwriting any user changes.
    pub fn reset() -> Result<PathBuf> {
        Self::install(true)
    }

    fn merge(&mut self, cfg: ConfigFile) {
        for (name, def) in cfg.engines {
            self.engines.insert(name, def);
        }
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<&CustomEngineDef> {
        self.engines.get(name)
    }

    #[must_use]
    pub fn names(&self) -> Vec<&str> {
        self.engines.keys().map(|k| k.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_roster_loads_completely() {
        // Rust-backed engines carry metadata-only TOML entries without a `url`;
        // one missing required field must not fail the whole embedded roster
        // (that used to silently empty `seia engines` and disable custom
        // engines, because load() returned Err).
        let builtin: ConfigFile =
            toml::from_str(BUILTIN_TOML).expect("embedded builtin_engines.toml must parse");
        assert!(
            builtin.engines.len() >= 18,
            "unexpected roster size: {}",
            builtin.engines.len()
        );
        for name in ["deepseek", "baidu", "tavily", "duckduckgo", "pubmed"] {
            assert!(builtin.engines.contains_key(name), "missing engine {name}");
        }
        assert!(builtin.engines["deepseek"].builtin);
        assert!(builtin.engines["deepseek"].help.is_some());
    }

    #[test]
    fn parse_minimal_engine_def() {
        let toml = r#"
[engines.github]
label = "GitHub Code Search"
method = "GET"
url = "https://api.github.com/search/code"
query_param = "q"
headers = { Authorization = "Bearer ${GITHUB_TOKEN}" }
result_path = "$.items[*]"
title_field = "name"
url_field = "html_url"
snippet_field = "repository.full_name"
"#;
        let cfg: ConfigFile = toml::from_str(toml).unwrap();
        let def = cfg.engines.get("github").unwrap();
        assert_eq!(def.label, "GitHub Code Search");
        assert_eq!(def.method, "GET");
        assert_eq!(def.query_param.as_deref(), Some("q"));
        assert_eq!(def.result_path.as_deref(), Some("$.items[*]"));
        assert_eq!(def.title_field, "name");
        assert_eq!(def.url_field, "html_url");
    }

    #[test]
    fn parse_post_engine_with_body_template() {
        let toml = r#"
[engines.test]
label = "Test"
method = "POST"
url = "https://example.com/api"
body_template = '{"q": "{{query}}", "n": {{limit}}}'
"#;
        let cfg: ConfigFile = toml::from_str(toml).unwrap();
        let def = cfg.engines.get("test").unwrap();
        assert_eq!(def.method, "POST");
        assert!(def.body_template.is_some());
    }

    #[test]
    fn defaults() {
        let toml = r#"
[engines.minimal]
label = "Minimal"
url = "https://example.com/search"
"#;
        let cfg: ConfigFile = toml::from_str(toml).unwrap();
        let def = cfg.engines.get("minimal").unwrap();
        assert_eq!(def.method, "GET");
        assert_eq!(def.title_field, "title");
        assert_eq!(def.url_field, "url");
        assert!(def.query_param.is_none());
        assert!(def.body_template.is_none());
        assert!(def.headers.is_empty());
        assert!(def.result_path.is_none());
    }

    #[test]
    fn render_templates() {
        unsafe { std::env::set_var("TOKEN", "secret123") };
        let def = CustomEngineDef {
            label: "T".into(),
            method: "GET".into(),
            url: "https://x.com?q={{query}}&n={{limit}}&t=${TOKEN}".into(),
            query_param: None,
            body_template: None,
            headers: Default::default(),
            result_path: None,
            title_field: "t".into(),
            url_field: "u".into(),
            snippet_field: None,
            pre_request: None,
            limit_param: None,
            ..Default::default()
        };
        let rendered = def.render(&def.url, "hello world", 20);
        assert!(rendered.contains("q=hello%20world"));
        assert!(rendered.contains("n=20"));
        assert!(rendered.contains("t=secret123"));
        assert!(!rendered.contains("${TOKEN}"));
    }

    #[test]
    fn render_missing_env_var_becomes_empty() {
        let def = CustomEngineDef {
            label: "T".into(),
            method: "GET".into(),
            url: "https://x.com?t=${MISSING_VAR_XYZ}".into(),
            query_param: None,
            body_template: None,
            headers: Default::default(),
            result_path: None,
            title_field: "t".into(),
            url_field: "u".into(),
            snippet_field: None,
            pre_request: None,
            limit_param: None,
            ..Default::default()
        };
        let rendered = def.render(&def.url, "q", 5);
        assert!(rendered.ends_with("t="));
    }
}
