//! DOAJ (Directory of Open Access Journals) — open-access journal index.
//!
//! No API key required. Free, unlimited access.
//! API docs: <https://doaj.org/api/v2/docs>

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
    let limit = opts.limit.unwrap_or(10).min(100);
    let url = format!(
        "https://doaj.org/api/search/articles/{}?pageSize={}&page=1",
        crate::utils::urlencode_query(query),
        limit
    );

    let resp: DoajResponse = http.get(&url).send().await?.json().await?;

    let items = resp
        .results
        .into_iter()
        .map(|a| SearchItem {
            title: a
                .bibjson
                .title
                .unwrap_or_else(|| "Untitled".to_string()),
            url: a
                .bibjson
                .link
                .into_iter()
                .flatten()
                .find(|l| l.url_type == Some("fulltext".to_string()))
                .map(|l| l.url)
                .unwrap_or_default(),
            snippet: a
                .bibjson
                .r#abstract
                .or_else(|| a.bibjson.keywords.clone()),
            content: None,
        })
        .collect();

    Ok((items, SearchMode::Api))
}

#[derive(Deserialize)]
struct DoajResponse {
    results: Vec<DoajArticle>,
}

#[derive(Deserialize)]
struct DoajArticle {
    bibjson: DoajBibJson,
}

#[derive(Deserialize)]
struct DoajBibJson {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    r#abstract: Option<String>,
    #[serde(default)]
    keywords: Option<String>,
    #[serde(default)]
    link: Vec<Option<DoajLink>>,
}

#[derive(Deserialize)]
struct DoajLink {
    #[serde(rename = "type")]
    url_type: Option<String>,
    url: String,
}
