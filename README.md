<p align="center"><img src="docs/logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<p align="center"><strong>Knowledge from every source</strong></p>

<p align="center">
  Rust multi-engine web search library and CLI. Free engines work out of the box.
</p>

<p align="center">
[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/seia)](https://docs.rs/seia)
[![CI](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)
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

### API / scrape (in the `Engine` enum — CLI + library)

| Engine | Mode | Auth | Free tier | Status |
|--------|------|------|-----------|--------|
| DuckDuckGo | Scrape | None | unlimited | ✅ |
| Wikipedia | API | None | unlimited | ✅ |
| SearXNG | API | `SEARXNG_URL` | self-hosted | ✅ |
| Tavily | API | `TAVILY_API_KEY` | 1 000/month | ✅ |
| Bing | API | `BING_SEARCH_API_KEY` | 1 000/month | 🔜 |
| Brave | API | `BRAVE_SEARCH_API_KEY` | 2 000/month | 🔜 |

> Bing and Brave API backends are stubs ("not yet implemented"). Use the
> browser profiles as a stopgap, or [contribute](https://github.com/celestia-island/seia).

### Browser (CLI-only — `seia search --browser --engine <name>`)

| Profile | Auth | Description |
|---------|------|-------------|
| google | None (scrapes via tairitsu) | Google web search. Also has a [paid CSE API](https://developers.google.com/custom-search). |
| baidu | None (scrapes via tairitsu) | Baidu web search. |
| bing_web | None (scrapes via tairitsu) | Bing web results. Also has a [paid Search API](https://www.microsoft.com/en-us/bing/apis/bing-web-search-api). |
| yandex | None (scrapes via tairitsu) | Yandex web search. |

Browser-mode engines use [tairitsu](https://github.com/celestia-island/tairitsu)
for headless rendering. Either run a standalone daemon or enable the
`embedded-browser` feature to compile tairitsu in-process.

> Most search engines offer official REST APIs (Google CSE, Bing Search API,
> Brave Search API). The browser profiles are a workaround for engines whose
> API backend hasn't been implemented yet, or where the API is not freely
> available. Long-term, each browser profile gets a matching `Engine` variant
> with API-key support.

## Development

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## License

SySL-1.0 (Synthetic Source License). See [LICENSE](./LICENSE).
