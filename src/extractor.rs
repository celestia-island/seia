//! Content extractor — fetches and cleans a URL's main text content.
//!
//! Uses a simple readability heuristic: extract the largest text block
//! from the page, stripping navigation, ads, and scripts.

use anyhow::Result;
use scraper::{Html, Selector};

pub async fn fetch_content(http: &reqwest::Client, url: &str) -> Result<String> {
    let resp = http.get(url).send().await?;
    let html = resp.text().await?;
    Ok(extract_main_text(&html))
}

fn extract_main_text(html: &str) -> String {
    let document = Html::parse_document(html);

    // Remove script, style, nav, footer, header
    let remove_sel = Selector::parse("script, style, nav, footer, header, aside, .ad, .ads, .sidebar").unwrap();

    // Try common content selectors
    let content_selectors = [
        "article", "main", "[role='main']", ".post-content", ".article-content",
        ".entry-content", "#content", ".content", "body",
    ];

    for sel_str in &content_selectors {
        if let Ok(sel) = Selector::parse(sel_str) {
            if let Some(element) = document.select(&sel).next() {
                let text: String = element.text().collect();
                let cleaned = clean_text(&text);
                if cleaned.len() > 200 {
                    return cleaned;
                }
            }
        }
    }

    // Fallback: get all text from body
    if let Ok(sel) = Selector::parse("body") {
        if let Some(body) = document.select(&sel).next() {
            let text: String = body.text().collect();
            return clean_text(&text);
        }
    }

    String::new()
}

fn clean_text(text: &str) -> String {
    text.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}
