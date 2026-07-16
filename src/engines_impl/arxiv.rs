//! arXiv API — open-access preprints in physics, math, CS, and more.
//!
//! No API key required. Returns Atom XML — we extract titles, links, and
//! summaries with a lightweight regex-based parser to avoid extra deps.
//! API docs: <https://info.arxiv.org/help/api/>

use anyhow::Result;
use regex::Regex;

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
        "http://export.arxiv.org/api/query?search_query=all:{}&start=0&max_results={}",
        crate::utils::urlencode_query(query),
        limit
    );

    let body = http.get(&url).send().await?.text().await?;
    let items = parse_atom_entries(&body);

    Ok((items, SearchMode::Api))
}

fn parse_atom_entries(xml: &str) -> Vec<SearchItem> {
    let entry_re = Regex::new(r"(?s)<entry>(.*?)</entry>").unwrap();
    let title_re = Regex::new(r"<title[^>]*>(.*?)</title>").unwrap();
    let id_re = Regex::new(r"<id[^>]*>(.*?)</id>").unwrap();
    let summary_re = Regex::new(r"<summary[^>]*>(.*?)</summary>").unwrap();

    let mut items = Vec::new();
    for cap in entry_re.captures_iter(xml) {
        let entry = cap.get(1).unwrap().as_str();
        let title = title_re
            .captures(entry)
            .and_then(|c| c.get(1))
            .map(|m| decode_xml_entities(m.as_str().trim()))
            .unwrap_or_default();
        // Skip the feed-level <title> (first entry match)
        if title.is_empty() || title == "ArXiv Query:" {
            continue;
        }
        let url = id_re
            .captures(entry)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();
        let snippet = summary_re
            .captures(entry)
            .and_then(|c| c.get(1))
            .map(|m| {
                let s = decode_xml_entities(m.as_str().trim());
                crate::utils::truncate(&s, 300)
            });
        if !title.is_empty() && !url.is_empty() {
            items.push(SearchItem {
                title,
                url,
                snippet,
                content: None,
            });
        }
    }
    items
}

fn decode_xml_entities(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace('\n', " ")
        .replace("  ", " ")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_arxiv_atom() {
        let xml = r#"<?xml version="1.0"?><feed xmlns="http://www.w3.org/2005/Atom">
            <title>ArXiv Query: all:rust</title>
            <entry>
                <title>Rust: A Safe Language for Systems Programming</title>
                <id>http://arxiv.org/abs/1234.5678v1</id>
                <summary>We present Rust, a language that provides memory safety without garbage collection.</summary>
            </entry>
            <entry>
                <title>Ownership Types in Practice</title>
                <id>http://arxiv.org/abs/9876.5432v1</id>
                <summary>This paper examines ownership type systems in real-world codebases.</summary>
            </entry>
        </feed>"#;
        let items = parse_atom_entries(xml);
        // Should skip the feed-level <title> and keep the 2 paper entries.
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].title, "Rust: A Safe Language for Systems Programming");
        assert_eq!(items[0].url, "http://arxiv.org/abs/1234.5678v1");
        assert!(items[0].snippet.as_ref().unwrap().contains("memory safety"));
        assert_eq!(items[1].title, "Ownership Types in Practice");
    }

    #[test]
    fn parse_arxiv_empty() {
        let xml = r#"<?xml version="1.0"?><feed xmlns="http://www.w3.org/2005/Atom"><title>ArXiv Query: nothing</title></feed>"#;
        let items = parse_atom_entries(xml);
        assert!(items.is_empty());
    }
}
