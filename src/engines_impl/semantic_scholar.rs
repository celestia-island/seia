//! Semantic Scholar API — free academic paper search, 200M+ papers.
//!
//! No API key required. Rate limit: 100 requests per 5 minutes without key.
//! API docs: <https://api.semanticscholar.org/api-docs/>

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
        "https://api.semanticscholar.org/graph/v1/paper/search?query={}&limit={}&fields=title,url,abstract",
        crate::utils::urlencode_query(query),
        limit
    );

    let resp: S2Response = http.get(&url).send().await?.json().await?;

    let items = resp
        .data
        .unwrap_or_default()
        .into_iter()
        .map(|p| SearchItem {
            title: p.title,
            url: p.url.unwrap_or_else(|| format!("https://www.semanticscholar.org/paper/{}", p.paper_id)),
            snippet: p.r#abstract,
            content: None,
        })
        .collect();

    Ok((items, SearchMode::Api))
}

#[derive(Deserialize)]
struct S2Response {
    #[serde(default)]
    data: Option<Vec<S2Paper>>,
}

#[derive(Deserialize)]
struct S2Paper {
    #[serde(rename = "paperId")]
    paper_id: String,
    title: String,
    url: Option<String>,
    #[serde(default)]
    r#abstract: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_real_response() {
        let json = r#"{"total":42,"offset":0,"data":[{"paperId":"abc123","title":"Rust: Safe Systems Programming","url":"https://www.semanticscholar.org/paper/abc123","abstract":"Rust is a systems programming language that guarantees memory safety without garbage collection."}]}"#;
        let resp: S2Response = serde_json::from_str(json).unwrap();
        let papers = resp.data.unwrap();
        assert_eq!(papers.len(), 1);
        assert_eq!(papers[0].title, "Rust: Safe Systems Programming");
        assert_eq!(papers[0].url.as_deref(), Some("https://www.semanticscholar.org/paper/abc123"));
    }

    #[test]
    fn parse_empty_response() {
        let json = r#"{"total":0,"offset":0,"data":[]}"#;
        let resp: S2Response = serde_json::from_str(json).unwrap();
        assert!(resp.data.unwrap().is_empty());
    }
}
