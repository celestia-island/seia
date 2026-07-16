//! OpenAlex API — open scholarly index, 250M+ works, fully free.
//!
//! No API key required. Polite pool has reasonable rate limits.
//! API docs: <https://docs.openalex.org/>

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
    let limit = opts.limit.unwrap_or(10).min(200);
    let url = format!(
        "https://api.openalex.org/works?search={}&per_page={}&sort=relevance",
        crate::utils::urlencode_query(query),
        limit
    );

    let resp: OaResponse = http
        .get(&url)
        .header("User-Agent", "mailto:seia@celestia-island.dev")
        .send()
        .await?
        .json()
        .await?;

    let items = resp
        .results
        .into_iter()
        .map(|w| SearchItem {
            title: w.title,
            url: w.id,
            snippet: w.abstract_inverted_index.map(|a| invert_abstract(&a)),
            content: None,
        })
        .collect();

    Ok((items, SearchMode::Api))
}

#[derive(Deserialize)]
struct OaResponse {
    results: Vec<OaWork>,
}

#[derive(Deserialize)]
struct OaWork {
    id: String,
    title: String,
    #[serde(default)]
    abstract_inverted_index: Option<serde_json::Value>,
}

fn invert_abstract(ai: &serde_json::Value) -> String {
    let mut pairs: Vec<(usize, &str)> = Vec::new();
    if let Some(obj) = ai.as_object() {
        for (word, positions) in obj {
            if let Some(pos_arr) = positions.as_array() {
                for pos in pos_arr {
                    if let Some(idx) = pos.as_u64() {
                        pairs.push((idx as usize, word.as_str()));
                    }
                }
            }
        }
    }
    if pairs.is_empty() {
        return String::new();
    }
    pairs.sort_by_key(|(i, _)| *i);
    let words: Vec<&str> = pairs.into_iter().map(|(_, w)| w).collect();
    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_openalex_response() {
        let json = r#"{"meta":{"count":1},"results":[{"id":"https://openalex.org/W123","title":"Machine Learning Basics","abstract_inverted_index":{"machine":[0],"learning":[1],"basics":[2]}}]}"#;
        let resp: OaResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.results.len(), 1);
        assert_eq!(resp.results[0].title, "Machine Learning Basics");
        assert_eq!(resp.results[0].id, "https://openalex.org/W123");
    }

    #[test]
    fn invert_abstract_basic() {
        let ai: serde_json::Value = serde_json::from_str(
            r#"{"hello":[0],"world":[1],"rust":[2]}"#,
        )
        .unwrap();
        let result = invert_abstract(&ai);
        assert_eq!(result, "hello world rust");
    }

    #[test]
    fn invert_abstract_empty() {
        let ai = serde_json::json!({});
        assert_eq!(invert_abstract(&ai), "");
    }
}
