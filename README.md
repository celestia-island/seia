<p align="center"><img src="docs/logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<p align="center"><strong>Knowledge from every source</strong></p>

<p align="center">
  Multi-engine web search library and CLI.
</p>

<div align="center">

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/seia)](https://docs.rs/seia)
[![CI](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)

</div>

<div align="center">
<a href="./docs/en/README.md">English</a> ·
<a href="./docs/zhs/README.md">简体中文</a> ·
<a href="./docs/zht/README.md">繁體中文</a> ·
<a href="./docs/ja/README.md">日本語</a> ·
<a href="./docs/ko/README.md">한국어</a> ·
<a href="./docs/fr/README.md">Français</a> ·
<a href="./docs/es/README.md">Español</a> ·
<a href="./docs/ru/README.md">Русский</a> ·
<a href="./docs/ar/README.md">العربية</a>
</div>

## Introduction

seia is a multi-engine web search library and CLI. It provides a unified
interface to query diverse search backends. Engines that do not require
authentication work out of the box with zero configuration.

## Quick Start

### CLI

```bash
# Basic search (no API key required)
seia search "rust async patterns"

# Choose a specific engine
seia search "Klein bottle" --engine wikipedia

# JSON output
seia search "climate change" --json

# Through a proxy
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# Browser mode (headless, no API key required)
seia search "query" --browser --browser-engine google
```

### Library

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Duckduckgo).await?;
```

## Development

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## Supported Search Engines

### API / scrape engines

| Engine | Website | Mode | Auth | Free tier | Status |
|--------|---------|------|------|-----------|--------|
| DuckDuckGo | [duckduckgo.com](https://duckduckgo.com) | Scrape | None | unlimited | ✅ |
| Wikipedia | [wikipedia.org](https://www.wikipedia.org) | API | None | unlimited | ✅ |
| SearXNG | [searxng.org](https://searxng.org) | API | `SEARXNG_URL` | self-hosted | ✅ |
| Tavily | [tavily.com](https://tavily.com) | API | `TAVILY_API_KEY` | 1 000/month | ✅ |
| Bing | [bing.com](https://www.bing.com) | API | `BING_SEARCH_API_KEY` | 1 000/month | 🔜 |
| Brave | [brave.com/search](https://brave.com/search) | API | `BRAVE_SEARCH_API_KEY` | 2 000/month | 🔜 |

> Bing and Brave API backends are stubs ("not yet implemented"). Use the
> browser profiles as a stopgap, or [contribute](https://github.com/celestia-island/seia).

### Browser engines (CLI-only)

| Engine | Website | Auth | Description |
|--------|---------|------|-------------|
| Google | [google.com](https://www.google.com) | None (scrapes via tairitsu) | Google web search. |
| Baidu | [baidu.com](https://www.baidu.com) | None (scrapes via tairitsu) | Baidu web search. |
| Bing Web | [bing.com](https://www.bing.com) | None (scrapes via tairitsu) | Bing web results. |
| Yandex | [yandex.com](https://yandex.com) | None (scrapes via tairitsu) | Yandex web search. |

Browser-mode engines use [tairitsu](https://github.com/celestia-island/tairitsu)
for headless rendering. Either run a standalone daemon or enable the
`embedded-browser` feature to compile tairitsu in-process.

> Most search engines offer official REST APIs. The browser profiles are a
> workaround for engines whose API backend hasn't been implemented yet, or
> where the API is not freely available. Long-term, each browser profile gets
> a matching `Engine` variant with API-key support.

## License

SySL-1.0 (Synthetic Source License). See [LICENSE](./LICENSE).
