//! Wikipedia API — free, unlimited, perfect for academic/knowledge queries.
//!
//! Uses the official MediaWiki Action API. No key needed.

use anyhow::Result;
use serde::Deserialize;

use crate::client::SearchOptions;
use crate::engines_impl::EngineOutput;
use crate::result::{SearchItem, SearchMode};

pub async fn search(
    http: &reqwest::Client,
    query: &str,
    _opts: &SearchOptions,
) -> Result<EngineOutput> {
    // Step 1: Search for relevant articles
    let search_url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&srlimit=10&format=json",
        urlencoding::encode(query)
    );

    let resp: WikiSearchResponse = http.get(&search_url).send().await?.json().await?;

    let mut items = Vec::new();
    for hit in resp.query.search {
        let url = format!(
            "https://en.wikipedia.org/wiki/{}",
            hit.title.replace(' ', "_")
        );
        items.push(SearchItem {
            title: hit.title,
            url,
            snippet: Some(strip_wiki_markup(&hit.snippet)),
            content: None,
        });
    }

    Ok((items, SearchMode::Api))
}

/// Fetch the full intro section of a Wikipedia article by title.
pub async fn fetch_intro(http: &reqwest::Client, title: &str) -> Result<String> {
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&prop=extracts&exintro&explaintext&titles={}&format=json",
        urlencoding::encode(title)
    );

    #[derive(Deserialize)]
    struct Resp {
        query: PageQuery,
    }
    #[derive(Deserialize)]
    struct PageQuery {
        pages: std::collections::HashMap<String, Page>,
    }
    #[derive(Deserialize)]
    struct Page {
        extract: Option<String>,
    }

    let resp: Resp = http.get(&url).send().await?.json().await?;
    let extract = resp
        .query
        .pages
        .into_values()
        .next()
        .and_then(|p| p.extract)
        .unwrap_or_default();

    Ok(extract)
}

fn strip_wiki_markup(s: &str) -> String {
    s.replace("<span class=\"searchmatch\">", "")
        .replace("</span>", "")
        .replace("&quot;", "\"")
        .replace("&amp;", "&")
        .trim()
        .to_string()
}

#[derive(Deserialize)]
struct WikiSearchResponse {
    query: WikiQuery,
}

#[derive(Deserialize)]
struct WikiQuery {
    search: Vec<WikiSearchHit>,
}

#[derive(Deserialize)]
struct WikiSearchHit {
    title: String,
    snippet: String,
}

mod urlencoding {
    pub fn encode(input: &str) -> String {
        let mut out = String::with_capacity(input.len() * 3);
        for byte in input.bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    out.push(byte as char);
                }
                b' ' => out.push_str("%20"),
                _ => out.push_str(&format!("%{:02X}", byte)),
            }
        }
        out
    }
}
