<p align="center"><img src="https://raw.githubusercontent.com/celestia-island/docs.celestia.world/dev/res/logo/seia.webp" alt="Seia" width="240" /></p>

<h1 align="center">Seia</h1>

<p align="center"><strong>Мультидвижковый веб-поиск</strong></p>

<div align="center">

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](https://sysl.celestia.world)
[![GitHub](https://img.shields.io/badge/github-celestia--island%2Fseia-blue.svg)](https://github.com/celestia-island/seia)
[![Checks](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)
[![docs.rs](https://docs.rs/seia/badge.svg)](https://docs.rs/seia)

</div>

<div align="center">

[English](../en/README.md) · [简体中文](../zhs/README.md) ·
[繁體中文](../zht/README.md) · [日本語](../ja/README.md) ·
[한국어](../ko/README.md) · [Français](../fr/README.md) ·
[Español](../es/README.md) · **Русский** ·
[العربية](../ar/README.md)

</div>

## Введение

seia — это мультидвижковая библиотека и CLI для веб-поиска. Она предоставляет единый
интерфейс для запросов к различным поисковым бэкендам. Движки, не требующие
аутентификации, работают сразу без какой-либо настройки.

## Быстрый старт

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

### Библиотека

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Wikipedia).await?;
```

## MCP-сервер

Соберите seia с feature `mcp` и запустите stdio-сервер — он предоставляет клиент многоядерного поиска AI-ассистентам программиста по протоколу Model Context Protocol:

```bash
seia mcp
```

Сервер предоставляет три инструмента: `seia_search` (один движок, по умолчанию duckduckgo без ключа), `seia_search_multi` (перебирает цепочку движков, возвращает первый с результатами) и `seia_list_engines` (девять движков и их переменные окружения для API-ключей). Подключите его к MCP-клиенту:

```json
{
  "mcpServers": {
    "seia": { "command": "seia", "args": ["mcp"] }
  }
}
```

Установите `SEIA_PROXY` для маршрутизации поисковых запросов через прокси (напр. `http://localhost:7890`); `HTTPS_PROXY` / `HTTP_PROXY` также поддерживаются.

## Разработка

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
just test-proxy  # run tests through localhost:7890 proxy (see tests/README)
```

## Поддерживаемые поисковые движки

| Движок | Аутентификация |
|--------|---------------|
| [DuckDuckGo](https://duckduckgo.com/) | нет |
| [Wikipedia](https://www.mediawiki.org/wiki/API:Search) | нет |
| [SearXNG](https://docs.searxng.org/) | `SEARXNG_URL` |
| [Tavily](https://docs.tavily.com/) | `TAVILY_API_KEY` |
| [Bing](https://learn.microsoft.com/en-us/bing/search-apis/bing-web-search/) | `BING_SEARCH_API_KEY` |
| [Brave](https://api.search.brave.com/app/documentation) | `BRAVE_SEARCH_API_KEY` |
| [秘塔 (MetaSo)](https://metaso.cn/search-api/playground) | `METASO_API_KEY` |
| [智谱 (Zhipu)](https://docs.bigmodel.cn/cn/guide/tools/web-search) | `ZHIPU_API_KEY` |
| [博查 (Bocha)](https://open.bochaai.com/docs) | `BOCHA_API_KEY` |

## Лицензия

SySL-1.0 (Synthetic Source License). См. [LICENSE](https://sysl.celestia.world).

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
