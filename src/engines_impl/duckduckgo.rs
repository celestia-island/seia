//! DuckDuckGo HTML scraping — free, no API key.
//!
//! Scrapes https://html.duckduckgo.com/html/ and parses result anchors.

use anyhow::Result;
use scraper::{Html, Selector};

use crate::client::SearchOptions;
use crate::engines_impl::EngineOutput;
use crate::result::{SearchItem, SearchMode};

pub async fn search(
    http: &reqwest::Client,
    query: &str,
    opts: &SearchOptions,
) -> Result<EngineOutput> {
    let url = format!(
        "https://html.duckduckgo.com/html/?q={}",
        urlencoding::encode(query)
    );

    let resp = http.get(&url).send().await?;
    let html = resp.text().await?;

    let items = parse_ddg_html(&html);

    Ok((items, SearchMode::Scrape))
}

fn parse_ddg_html(html: &str) -> Vec<SearchItem> {
    let document = Html::parse_document(html);
    let mut items = Vec::new();

    // DuckDuckGo HTML uses class="result__a" for result links
    // and class="result__snippet" for snippets
    if let Ok(link_sel) = Selector::parse("a.result__a") {
        let snippet_sel = Selector::parse(".result__snippet").ok();

        for (i, link) in document.select(&link_sel).enumerate() {
            if i >= 20 {
                break;
            }
            let title = link.text().collect::<String>().trim().to_string();
            if title.is_empty() {
                continue;
            }

            // DDG wraps URLs in a redirect; extract the actual URL
            let raw_href = link.value().attr("href").unwrap_or("");
            let url = extract_ddg_url(raw_href);

            if url.is_empty() || title.is_empty() {
                continue;
            }

            // Try to find snippet — use the result__snippet class near this link
            let snippet = snippet_sel.as_ref().and_then(|snip_sel| {
                document.select(snip_sel).nth(i).map(|s| s.text().collect::<String>().trim().to_string())
            });

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

/// DuckDuckGo wraps URLs in /l/?uddg=... redirect. Extract the real URL.
fn extract_ddg_url(raw: &str) -> String {
    // DDG lite/html format: //duckduckgo.com/l/?uddg=<url>&rut=...
    if let Some(start) = raw.find("uddg=") {
        let after = &raw[start + 5..];
        if let Some(end) = after.find('&') {
            return urlencoding::decode(&after[..end])
                .unwrap_or_else(|_| after[..end].to_string());
        }
        return urlencoding::decode(after)
            .unwrap_or_else(|_| after.to_string());
    }
    // Already a clean URL
    if raw.starts_with("http") {
        raw.to_string()
    } else if raw.starts_with("//") {
        format!("https:{}", raw)
    } else {
        raw.to_string()
    }
}

// Minimal URL encoding (avoid pulling in another crate)
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

    pub fn decode(input: &str) -> Result<String, std::string::FromUtf8Error> {
        let mut out = Vec::new();
        let bytes = input.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'%' && i + 2 < bytes.len() {
                let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or("00");
                if let Ok(byte) = u8::from_str_radix(hex, 16) {
                    out.push(byte);
                    i += 3;
                    continue;
                }
            }
            out.push(bytes[i]);
            i += 1;
        }
        String::from_utf8(out)
    }
}
