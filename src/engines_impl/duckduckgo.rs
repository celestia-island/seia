//! `DuckDuckGo` HTML scraping — free, no API key.
//!
//! Scrapes <https://html.duckduckgo.com/html/> and parses result anchors.

use anyhow::{Result, anyhow};
use scraper::{Html, Selector};

use crate::client::SearchOptions;
use crate::engines_impl::EngineOutput;
use crate::result::{SearchItem, SearchMode};

/// Search using `DuckDuckGo` HTML scraping.
///
/// # Errors
///
/// Returns `Err` on network failure, CAPTCHA challenge, or zero results.
pub async fn search(
    http: &reqwest::Client,
    query: &str,
    _opts: &SearchOptions,
) -> Result<EngineOutput> {
    let url = format!(
        "https://html.duckduckgo.com/html/?q={}",
        crate::utils::urlencode_query(query)
    );

    let resp = http.get(&url).send().await?;
    let html = resp.text().await?;

    // DuckDuckGo sometimes serves a CAPTCHA / anomaly challenge instead of
    // results (especially from flagged IPs — proxy, VPS, Tor). Return an
    // explicit error so the fallback system can try the next engine instead
    // of silently reporting "0 results".
    if html.contains("anomaly-modal") || html.contains("Unfortunately, bots use DuckDuckGo too") {
        return Err(anyhow!(
            "DuckDuckGo served a CAPTCHA challenge (proxy/VPS IP may be flagged). \
             Try --engine wikipedia or use a different proxy."
        ));
    }

    let items = parse_ddg_html(&html);

    if items.is_empty() {
        return Err(anyhow!("DuckDuckGo returned 0 results for: {query}"));
    }

    Ok((items, SearchMode::Scrape))
}

fn parse_ddg_html(html: &str) -> Vec<SearchItem> {
    let document = Html::parse_document(html);
    let mut items = Vec::new();

    // Container-based parsing: walk .result / .web-result divs so the snippet
    // always comes from the SAME result as the link (positional nth() misaligns
    // when a link is skipped).
    let result_sel = Selector::parse("div.result, div.web-result").ok();
    let Ok(link_sel) = Selector::parse("a.result__a") else {
        return items;
    };
    let snippet_sel = Selector::parse(".result__snippet").ok();

    if let Some(result_sel) = result_sel {
        for container in document.select(&result_sel) {
            if items.len() >= 20 {
                break;
            }
            let Some(link) = container.select(&link_sel).next() else {
                continue;
            };
            let title = link.text().collect::<String>().trim().to_string();
            if title.is_empty() {
                continue;
            }
            let raw_href = link.value().attr("href").unwrap_or("");
            let url = extract_ddg_url(raw_href);
            if url.is_empty() {
                continue;
            }
            let snippet = snippet_sel.as_ref().and_then(|s| {
                container
                    .select(s)
                    .next()
                    .map(|e| e.text().collect::<String>().trim().to_string())
            });
            items.push(SearchItem {
                title,
                url,
                snippet,
                content: None,
            });
        }
    }

    // Fallback: link-only parsing (if DDG changes the container structure).
    if items.is_empty() {
        for link in document.select(&link_sel) {
            if items.len() >= 20 {
                break;
            }
            let title = link.text().collect::<String>().trim().to_string();
            if title.is_empty() {
                continue;
            }
            let raw_href = link.value().attr("href").unwrap_or("");
            let url = extract_ddg_url(raw_href);
            if url.is_empty() {
                continue;
            }
            items.push(SearchItem {
                title,
                url,
                snippet: None,
                content: None,
            });
        }
    }

    items
}

/// `DuckDuckGo` wraps URLs in /l/?uddg=... redirect. Extract the real URL.
fn extract_ddg_url(raw: &str) -> String {
    if let Some(start) = raw.find("uddg=") {
        let after = &raw[start + 5..];
        if let Some(end) = after.find('&') {
            return urlencoding::decode(&after[..end]).unwrap_or_else(|_| after[..end].to_string());
        }
        return urlencoding::decode(after).unwrap_or_else(|_| after.to_string());
    }
    if raw.starts_with("http") {
        raw.to_string()
    } else if raw.starts_with("//") {
        format!("https:{raw}")
    } else {
        raw.to_string()
    }
}

mod urlencoding {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ddg_html_parsing() {
        let html = r#"
        <html><body>
        <div class="result">
            <h2 class="result__title">
                <a rel="nofollow" class="result__a" href="//duckduckgo.com/l/?uddg=https%3A%2F%2Fexample.com%2Ftest&rut=abc">
                    Example Test Page
                </a>
            </h2>
            <a class="result__snippet">This is a test snippet about example topics.</a>
        </div>
        <div class="result">
            <h2 class="result__title">
                <a rel="nofollow" class="result__a" href="https://another.com/page">
                    Another Page
                </a>
            </h2>
            <a class="result__snippet">Another snippet here.</a>
        </div>
        </body></html>
        "#;

        let items = parse_ddg_html(html);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].title, "Example Test Page");
        assert_eq!(items[1].title, "Another Page");
        assert!(items[0].url.contains("example.com"));
        assert_eq!(items[1].url, "https://another.com/page");
    }

    #[test]
    fn test_ddg_url_extraction() {
        let result = extract_ddg_url("//duckduckgo.com/l/?uddg=https%3A%2F%2Fexample.com&rut=abc");
        assert_eq!(result, "https://example.com");

        let result2 = extract_ddg_url("https://direct.com/page");
        assert_eq!(result2, "https://direct.com/page");

        let result3 = extract_ddg_url("//cdn.example.com/resource");
        assert_eq!(result3, "https://cdn.example.com/resource");
    }

    #[test]
    fn test_ddg_empty_html() {
        let html = "<html><body>No results</body></html>";
        let items = parse_ddg_html(html);
        assert!(items.is_empty());
    }

    #[test]
    fn test_url_encoding() {
        assert_eq!(
            crate::utils::urlencode_query("hello world"),
            "hello%20world"
        );
        assert_eq!(crate::utils::urlencode_query("a+b=c"), "a%2Bb%3Dc");
        assert_eq!(crate::utils::urlencode_query("safe123-_.~"), "safe123-_.~");
    }

    #[test]
    fn test_url_decoding() {
        let decoded = urlencoding::decode("https%3A%2F%2Fexample.com%2Fpage").unwrap();
        assert_eq!(decoded, "https://example.com/page");
    }
}
