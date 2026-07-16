//! PubMed / Entrez API — biomedical literature from NCBI.
//!
//! No API key required. Use `NCBI_API_KEY` for higher rate limits (10/s
//! instead of 3/s), but works without one.
//! API docs: <https://www.ncbi.nlm.nih.gov/books/NBK25497/>

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
    let api_key = std::env::var("NCBI_API_KEY").ok();

    // Step 1: Search for PMIDs.
    let mut search_url = format!(
        "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=pubmed&term={}&retmax={}&retmode=json&sort=relevance",
        crate::utils::urlencode_query(query),
        limit
    );
    if let Some(ref key) = api_key {
        search_url.push_str(&format!("&api_key={key}"));
    }

    let esearch: ESearchResult = http.get(&search_url).send().await?.json().await?;
    let ids: Vec<String> = esearch
        .esearchresult
        .idlist
        .unwrap_or_default();

    if ids.is_empty() {
        return Ok((vec![], SearchMode::Api));
    }

    // Step 2: Fetch summaries for those PMIDs.
    let id_str = ids.join(",");
    let mut fetch_url = format!(
        "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esummary.fcgi?db=pubmed&id={id_str}&retmode=json"
    );
    if let Some(ref key) = api_key {
        fetch_url.push_str(&format!("&api_key={key}"));
    }

    let esummary: ESummaryResult = http.get(&fetch_url).send().await?.json().await?;

    let items = ids
        .iter()
        .filter_map(|id| {
            let doc = esummary.result.uids.get(id)?;
            let uid = id;
            Some(SearchItem {
                title: doc.title.clone(),
                url: format!("https://pubmed.ncbi.nlm.nih.gov/{uid}"),
                snippet: doc.elocationid.clone(),
                content: None,
            })
        })
        .collect();

    Ok((items, SearchMode::Api))
}

#[derive(Deserialize)]
struct ESearchResult {
    esearchresult: ESearchIdList,
}

#[derive(Deserialize)]
struct ESearchIdList {
    #[serde(default)]
    idlist: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct ESummaryResult {
    result: ESummaryUids,
}

#[derive(Deserialize)]
struct ESummaryUids {
    uids: std::collections::HashMap<String, PubMedDoc>,
}

#[derive(Deserialize)]
struct PubMedDoc {
    title: String,
    #[serde(default)]
    elocationid: Option<String>,
}
