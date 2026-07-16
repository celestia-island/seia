//! CrossRef API — DOI metadata registry, 150M+ scholarly records.
//!
//! No API key required for polite usage.
//! API docs: <https://api.crossref.org/>

use anyhow::Result;
use serde::Deserialize;

use crate::{
    client::SearchOptions,
    engines_impl::EngineOutput,
    result::{SearchItem, SearchMode},
};

pub async fn search(
    http: &reqwest::Client,
    query: &str,
    opts: &SearchOptions,
) -> Result<EngineOutput> {
    let limit = opts.limit.unwrap_or(10).min(1000);
    let url = format!(
        "https://api.crossref.org/works?query={}&rows={}&sort=relevance",
        crate::utils::urlencode_query(query),
        limit
    );

    let resp: CrResponse = http.get(&url).send().await?.json().await?;

    let items = resp
        .message
        .items
        .into_iter()
        .map(|w| {
            let snippet = w.r#abstract.clone().or_else(|| w.container_title());
            let title = w.title.unwrap_or_default();
            let url = w.url.unwrap_or(w.doi);
            SearchItem { title, url, snippet, content: None }
        })
        .collect();

    Ok((items, SearchMode::Api))
}

#[derive(Deserialize)]
struct CrResponse {
    message: CrMessage,
}

#[derive(Deserialize)]
struct CrMessage {
    items: Vec<CrWork>,
}

#[derive(Deserialize)]
struct CrWork {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    url: Option<String>,
    #[serde(rename = "DOI")]
    doi: String,
    #[serde(default)]
    r#abstract: Option<String>,
    #[serde(default)]
    #[serde(rename = "container-title")]
    container_title: Option<Vec<String>>,
}

impl CrWork {
    fn container_title(&self) -> Option<String> {
        self.container_title.as_ref().and_then(|v| v.first().cloned())
    }
}
