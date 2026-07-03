//! Shared utilities — URL encoding helpers and text helpers.

use std::fmt::Write;

/// Percent-encode a string (space → `%20`). Used by `DuckDuckGo`, Wikipedia, `SearXNG`.
#[must_use]
pub fn urlencode_query(input: &str) -> String {
    let mut out = String::with_capacity(input.len() * 3);
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            b' ' => out.push_str("%20"),
            _ => {
                let _ = write!(out, "%{byte:02X}");
            }
        }
    }
    out
}

/// Truncate a string to at most `max` chars, appending "..." if cut.
#[must_use]
pub fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max])
    } else {
        s.to_string()
    }
}
