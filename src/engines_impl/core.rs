//! CORE API — world's largest collection of open-access research papers.
//!
//! Requires a free API key from <https://core.ac.uk/services/api>.
//! Set `CORE_API_KEY` environment variable.
//! API docs: <https://api.core.ac.uk/docs/>

use anyhow::Result;
use serde::Deserialize;

use crate::{
    client::{EngineConfig, SearchOptions},
    engines::Engine,
    engines_impl::EngineOutput,
    result::{SearchItem, SearchMode},
};

/// Search path appended to the (configurable) base URL.
pub const SEARCH_PATH: &str = "/v3/search/works";
const KEY_HINT: &str = "Get one at https://core.ac.uk/services/api";

pub async fn search(
    http: &reqwest::Client,
    query: &str,
    opts: &SearchOptions,
    cfg: &EngineConfig,
) -> Result<EngineOutput> {
    let api_key = cfg.resolve_api_key(&Engine::Core, KEY_HINT)?;

    let limit = opts.limit.unwrap_or(10).min(100);
    let base_url = cfg.resolve_base_url(&Engine::Core)?;
    let url = format!(
        "{base_url}{SEARCH_PATH}?q={}&limit={}&offset=0",
        crate::utils::urlencode_query(query),
        limit
    );

    let resp: CoreResponse = http
        .get(&url)
        .header("Authorization", format!("Bearer {api_key}"))
        .send()
        .await?
        .json()
        .await?;

    let items = resp
        .results
        .into_iter()
        .map(|w| SearchItem {
            title: w.title,
            url: w.download_url.unwrap_or(w.source_url),
            snippet: w.r#abstract,
            content: None,
            ..Default::default()
        })
        .collect();

    Ok((items, SearchMode::Api))
}

#[derive(Deserialize)]
struct CoreResponse {
    results: Vec<CoreWork>,
}

#[derive(Deserialize)]
struct CoreWork {
    title: String,
    #[serde(default)]
    r#abstract: Option<String>,
    #[serde(default)]
    #[serde(rename = "downloadUrl")]
    download_url: Option<String>,
    #[serde(default)]
    #[serde(rename = "sourceUrl")]
    source_url: String,
}
