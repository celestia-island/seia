<p align="center"><img src="docs/logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<p align="center"><strong>Knowledge from every source</strong></p>

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

All engines go through their official HTTP API (or, where none exists,
lightweight HTML scraping). No headless browser is bundled — seia is a
pure HTTP client.

### International

| Engine | Website | Mode | Auth | Free tier | Status |
|--------|---------|------|------|-----------|--------|
| DuckDuckGo | [duckduckgo.com](https://duckduckgo.com) | Scrape | None | unlimited | ✅ |
| Wikipedia | [wikipedia.org](https://www.wikipedia.org) | API | None | unlimited | ✅ |
| SearXNG | [searxng.org](https://searxng.org) | API | `SEARXNG_URL` | self-hosted | ✅ |
| Tavily | [tavily.com](https://tavily.com) | API | `TAVILY_API_KEY` | 1 000/month | ✅ |
| Bing | [bing.com](https://www.bing.com) | API | `BING_SEARCH_API_KEY` | 1 000/month | ✅ |
| Brave | [brave.com/search](https://brave.com/search) | API | `BRAVE_SEARCH_API_KEY` | 2 000/month | ✅ |

### Domestic (China)

| Engine | Website | Mode | Auth | Status |
|--------|---------|------|------|--------|
| 智谱 (Zhipu) | [bigmodel.cn](https://bigmodel.cn) | API | `ZHIPU_API_KEY` | ✅ |
| 博查 (Bocha) | [open.bochaai.com](https://open.bochaai.com) | API | `BOCHA_API_KEY` | ✅ |

> 智谱 routes through one of several backing engines (智谱基础版/高阶版, 搜狗,
> 夸克). Pick one with the `ZHIPU_SEARCH_ENGINE` env var (`search_std` by
> default; also `search_pro`, `search_pro_sogou`, `search_pro_quark`).

## License

SySL-1.0 (Synthetic Source License). See [LICENSE](./LICENSE).
