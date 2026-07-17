<p align="center"><img src="https://raw.githubusercontent.com/celestia-island/docs.celestia.world/dev/res/logo/seia.webp" alt="Seia" width="240" /></p>

<h1 align="center">Seia</h1>

<p align="center"><strong>多引擎網路搜尋</strong></p>

<div align="center">

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](https://sysl.celestia.world)
[![GitHub](https://img.shields.io/badge/github-celestia--island%2Fseia-blue.svg)](https://github.com/celestia-island/seia)
[![Checks](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)
[![docs.rs](https://docs.rs/seia/badge.svg)](https://docs.rs/seia)

</div>

<div align="center">

[English](../en/README.md) · [简体中文](../zhs/README.md) ·
**繁體中文** · [日本語](../ja/README.md) ·
[한국어](../ko/README.md) · [Français](../fr/README.md) ·
[Español](../es/README.md) · [Русский](../ru/README.md) ·
[العربية](../ar/README.md)

</div>

## 簡介

seia 是一個多引擎 Web 搜尋函式庫與 CLI 工具。透過統一介面存取多樣化的搜尋後端，無需認證的引擎零設定即可使用。

## 快速開始

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

### 作為函式庫使用

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Wikipedia).await?;
```

## MCP 伺服器

使用 `mcp` feature 建置 seia 並執行 stdio 伺服器——它透過模型上下文協定（Model Context Protocol）將多引擎搜尋客戶端暴露給 AI 編碼助手：

```bash
seia mcp
```

伺服器提供三個工具：`seia_search`（單引擎，預設使用 duckduckgo，無需 API 金鑰）、`seia_search_multi`（嘗試一系列引擎，回傳第一個有結果的）和 `seia_list_engines`（列出九個引擎及其 API 金鑰環境變數）。將其接入 MCP 客戶端：

```json
{
  "mcpServers": {
    "seia": { "command": "seia", "args": ["mcp"] }
  }
}
```

設定 `SEIA_PROXY` 透過代理路由搜尋請求（例如 `http://localhost:7890`）；也支援 `HTTPS_PROXY` / `HTTP_PROXY`。

## 開發

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
just test-proxy  # run tests through localhost:7890 proxy (see tests/README)
```

## 支援的搜尋引擎

| 引擎 | 認證 |
|------|------|
| [DuckDuckGo](https://duckduckgo.com/) | 無 |
| [Wikipedia](https://www.mediawiki.org/wiki/API:Search) | 無 |
| [SearXNG](https://docs.searxng.org/) | `SEARXNG_URL` |
| [Tavily](https://docs.tavily.com/) | `TAVILY_API_KEY` |
| [Bing](https://learn.microsoft.com/en-us/bing/search-apis/bing-web-search/) | `BING_SEARCH_API_KEY` |
| [Brave](https://api.search.brave.com/app/documentation) | `BRAVE_SEARCH_API_KEY` |
| [秘塔 (MetaSo)](https://metaso.cn/search-api/playground) | `METASO_API_KEY` |
| [智譜 (Zhipu)](https://docs.bigmodel.cn/cn/guide/tools/web-search) | `ZHIPU_API_KEY` |
| [博查 (Bocha)](https://open.bochaai.com/docs) | `BOCHA_API_KEY` |

## 授權條款

SySL-1.0（Synthetic Source License）。詳見 [LICENSE](https://sysl.celestia.world)。

## MCP Server Deployment

> (English section — translation pending)

For production MCP deployments, use an **auto-restart wrapper** to keep the server alive across updates without interrupting the client session.

### Recommended launcher

#!/bin/bash
while true; do
  /path/to/seia mcp
  sleep 0.2
done

### How it works

1. The wrapper runs `seia mcp` in a `while true` loop.
2. If the process exits, it restarts within 0.2 seconds.
3. To update: `kill $(pgrep -f "seia mcp" | head -1)`
4. For managed restarts, use [malkuth](https://github.com/celestia-island/malkuth) as a supervised watcher.
