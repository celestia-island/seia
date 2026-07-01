<p align="center"><img src="docs/logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<p align="center"><strong>One query, every search engine.</strong></p>

<p align="center">
  Multi-engine web search for Rust. Free engines work out of the box.
</p>

<p align="center">
[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](./LICENSE)
[![CI](https://github.com/celestia-island/seia/actions/workflows/checks.yml/badge.svg)](https://github.com/celestia-island/seia/actions)
</p>

<p align="center">
<a href="./docs/en/README.md">English</a> ·
<a href="./docs/zhs/README.md">简体中文</a> ·
<a href="./docs/zht/README.md">繁體中文</a> ·
<a href="./docs/ja/README.md">日本語</a> ·
<a href="./docs/ko/README.md">한국어</a> ·
<a href="./docs/fr/README.md">Français</a> ·
<a href="./docs/es/README.md">Español</a> ·
<a href="./docs/ru/README.md">Русский</a> ·
<a href="./docs/ar/README.md">العربية</a>
</p>

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

SySL-1.0 (Synthetic Source License). See [LICENSE](./LICENSE).
