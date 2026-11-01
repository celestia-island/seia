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
///
/// Counts Unicode scalar values rather than bytes so CJK text (e.g. provider
/// error bodies) is never sliced mid-character.
#[must_use]
pub fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() > max {
        let head: String = s.chars().take(max).collect();
        format!("{head}...")
    } else {
        s.to_string()
    }
}

/// Truncate by Unicode scalar value to at most `max` chars, without an
/// ellipsis. Used where an upstream API imposes a hard character limit
/// (e.g. Baidu AI Search caps the query at 72 characters).
#[must_use]
pub fn truncate_chars(s: &str, max: usize) -> String {
    s.chars().take(max).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_does_not_split_multibyte_chars() {
        // 10 CJK chars = 30 bytes; a byte-slice at 4 would panic.
        let s = "翁法罗斯全部资料分析";
        assert_eq!(truncate(s, 4), "翁法罗斯...");
        assert_eq!(truncate(s, 20), s);
        assert_eq!(truncate("abc", 3), "abc");
        assert_eq!(truncate_chars(s, 4), "翁法罗斯");
    }
}
