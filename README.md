# seia

<div align="center">

**One query, every search engine.**

Multi-engine web search for Rust. Free engines work out of the box.

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](./LICENSE)
[![CI](https://github.com/celestia-island/seia/actions/workflows/checks.yml/badge.svg)](https://github.com/celestia-island/seia/actions)

[English](./README.md) · [简体中文](./docs/zhs/README.md) · [Docs](./docs/en/SUMMARY.md)

</div>

## Introduction

seia lets you search the web through DuckDuckGo, Tavily, Wikipedia, SearXNG,
Bing, Brave, Google, Baidu, and more — all behind one interface. Free engines
work out of the box with zero configuration.

## Quick Start

### CLI

```bash
# Basic search (DuckDuckGo, free, no key)
seia search "rust async patterns"

# Wikipedia (free, academic)
seia search "Klein bottle" --engine wikipedia

# JSON output
seia search "climate change" --json

# Through a proxy
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# Browser mode (Google/Baidu via tairitsu)
seia search "query" --engine google --browser
```

### Library

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::DuckDuckGo).await?;
```

## Engines

| Engine | Mode | Auth | Status |
|--------|------|------|--------|
| DuckDuckGo | Scrape | None | ✅ |
| Wikipedia | API | None | ✅ |
| SearXNG | API | `SEARXNG_URL` | ✅ |
| Tavily | API | `TAVILY_API_KEY` | ✅ |
| Bing | API | `BING_SEARCH_API_KEY` | 🔲 |
| Brave | API | `BRAVE_SEARCH_API_KEY` | 🔲 |
| Google | Browser | tairitsu | ✅ |
| Baidu | Browser | tairitsu | ✅ |
| Bing Web | Browser | tairitsu | ✅ |
| Yandex | Browser | tairitsu | ✅ |

Browser-mode engines use [tairitsu](https://github.com/celestia-island/tairitsu)
for headless rendering. Either run a standalone daemon or enable the
`embedded-browser` feature to compile tairitsu in-process.

## Development

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## License

SySL-1.0 (Synthetic Source License).
