<p align="center"><img src="https://raw.githubusercontent.com/celestia-island/seia/master/docs/logo.webp" alt="Seia" width="240" /></p>

<h1 align="center">Seia</h1>

<p align="center"><strong>多引擎网络搜索</strong></p>

<div align="center">

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](https://sysl.celestia.world)
[![GitHub](https://img.shields.io/badge/github-celestia--island%2Fseia-blue.svg)](https://github.com/celestia-island/seia)
[![Checks](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)
[![docs.rs](https://docs.rs/seia/badge.svg)](https://docs.rs/seia)

</div>

<div align="center">

[English](../en/README.md) · **简体中文** ·
[繁體中文](../zht/README.md) · [日本語](../ja/README.md) ·
[한국어](../ko/README.md) · [Français](../fr/README.md) ·
[Español](../es/README.md) · [Русский](../ru/README.md) ·
[العربية](../ar/README.md)

</div>

## 简介

seia 是一个多引擎 Web 搜索库与 CLI 工具。通过统一接口访问多样化的搜索后端，无需认证的引擎零配置即可使用。

## 快速开始

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

### 作为库使用

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Wikipedia).await?;
```

## 开发

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
just test-proxy  # run tests through localhost:7890 proxy (see tests/README)
```

## 支持的搜索引擎

| 引擎 | 认证 |
|------|------|
| [DuckDuckGo](https://duckduckgo.com/) | 无 |
| [Wikipedia](https://www.mediawiki.org/wiki/API:Search) | 无 |
| [SearXNG](https://docs.searxng.org/) | `SEARXNG_URL` |
| [Tavily](https://docs.tavily.com/) | `TAVILY_API_KEY` |
| [Bing](https://learn.microsoft.com/en-us/bing/search-apis/bing-web-search/) | `BING_SEARCH_API_KEY` |
| [Brave](https://api.search.brave.com/app/documentation) | `BRAVE_SEARCH_API_KEY` |
| [秘塔 (MetaSo)](https://metaso.cn/search-api/playground) | `METASO_API_KEY` |
| [智谱 (Zhipu)](https://docs.bigmodel.cn/cn/guide/tools/web-search) | `ZHIPU_API_KEY` |
| [博查 (Bocha)](https://open.bochaai.com/docs) | `BOCHA_API_KEY` |

## 许可证

SySL-1.0（Synthetic Source License）。详见 [LICENSE](https://sysl.celestia.world)。
