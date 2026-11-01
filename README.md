<p align="center"><img src="https://raw.githubusercontent.com/celestia-island/docs.celestia.world/master/res/logo/seia.webp" alt="Seia" width="240" /></p>

<h1 align="center">Seia</h1>

<p align="center"><strong>Multi-engine web search</strong></p>

<div align="center">

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](https://sysl.celestia.world)
[![GitHub](https://img.shields.io/badge/github-celestia--island%2Fseia-blue.svg)](https://github.com/celestia-island/seia)
[![Checks](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)
[![docs.rs](https://docs.rs/seia/badge.svg)](https://docs.rs/seia)

</div>

<div align="center">

**English** ·
[简体中文](./docs/zhs/README.md) ·
[繁體中文](./docs/zht/README.md) ·
[日本語](./docs/ja/README.md) ·
[한국어](./docs/ko/README.md) ·
[Français](./docs/fr/README.md) ·
[Español](./docs/es/README.md) ·
[Русский](./docs/ru/README.md) ·
[العربية](./docs/ar/README.md)

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

### npx (no Rust toolchain required)

Prebuilt binaries are published to npm, so you can run `seia` with a single
command — no `cargo build`:

```bash
npx @celestia-island/seia search "rust async patterns"
npx @celestia-island/seia mcp        # the MCP server (needs the mcp build)
```

The `@celestia-island/seia` root package pulls the right platform subpackage
(`-linux-x64` / `-darwin-arm64` / `-win32-x64`) automatically. To pin a version:

```bash
npx @celestia-island/seia@0.1.0 search "Klein bottle" --engine wikipedia
```

### Library

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Wikipedia).await?;
```

## MCP server

Build seia with the `mcp` feature and run the stdio server — it exposes the
multi-engine search client to AI coding assistants over the Model Context
Protocol:

```bash
seia mcp
```

The server advertises three tools: `seia_search` (one engine, defaults to
duckduckgo so it needs no key), `seia_search_multi` (try a chain of engines,
return the first with results), and `seia_list_engines` (the nine engines and
their API-key env vars). Wire it into an MCP client:

```json
{
  "mcpServers": {
    "seia": { "command": "seia", "args": ["mcp"] }
  }
}
```

Set `SEIA_PROXY` to route search requests through a proxy
(e.g. `http://localhost:7890`); `HTTPS_PROXY` / `HTTP_PROXY` are also honoured.

## HTTP search service (`seia serve`)

`seia serve` exposes the engine roster over HTTP so another service (e.g. the
`arona` agent gateway) can run a provider chain without embedding seia.

Build with the `serve` feature (`just build` and the release workflows use
`--all-features`, so this is already on) and run it:

```bash
# defaults: bind 127.0.0.1:3013, no bearer token required when SEIA_TOKEN is unset
seia serve

# explicit bind + bearer token
SEIA_BIND=127.0.0.1:3013 SEIA_TOKEN='<your-token>' seia serve
```

`POST /v1/search` runs a provider chain in order and returns the first provider
that produced results; every attempt (ok/failed, duration, error) is reported:

```bash
curl -s http://127.0.0.1:3013/v1/search \
  -H 'Authorization: Bearer <your-token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "query": "翁法罗斯",
    "count": 8,
    "need_content": false,
    "ori_question": "翁法罗斯是什么？",
    "locale": "zh-CN",
    "chain": [
      {"provider": "deepseek", "protocol": "deepseek_search_v1",
       "base_url": "https://api.deepseek.com", "api_key": "<your-key>"},
      {"provider": "baidu", "protocol": "baidu_ai_search_v1",
       "base_url": "https://qianfan.baidubce.com", "api_key": "<your-key>"}
    ]
  }'
```

Response: `{query, provider, engine, protocol, count, took_ms, results, attempts}`
where each result is the normalized shape (`title`, `url`, `snippet`, `summary`,
`content`, `site_name`, `site_icon`, `date`, `authority`, `rerank_score`,
`engine`). Errors: `400` bad request, `401` missing/invalid bearer token,
`502` every provider failed (`{"error":{"code":"search_failed",…,"attempts":[…]}}`).
Provider protocols are dispatched by name (`deepseek_search_v1`,
`baidu_ai_search_v1`, `tavily_search_v1`, `brave_search_v1`, `bing_search_v1`,
`bocha_search_v1`, `metaso_search_v1`, `zhipu_search_v1`, `searxng_search_v1`,
`duckduckgo_search_v1`, `wikipedia_search_v1`); an unknown protocol fails that
attempt with `unsupported_protocol` and the chain continues.

`GET /v1/engines` returns `{engines:[{id, protocol, needs_key, env_var,
default_base_url, capabilities}], default_chain:[…]}` and `GET /health` is an
unauthenticated liveness probe.

| Env var | Default | Meaning |
|---|---|---|
| `SEIA_BIND` | `127.0.0.1:3013` | Listen address (`--bind` overrides) |
| `SEIA_TOKEN` | unset | When set, `/v1/*` requires `Authorization: Bearer <token>` |
| `SEIA_DEFAULT_CHAIN` | `deepseek,baidu,duckduckgo,wikipedia` | Chain used when a request omits `chain` |
| `RUST_LOG` | `info` | Log filter (the `seia_search served` / `search_failed` lines are INFO) |

Chain entries carry plaintext credentials by design (caller → seia over
loopback); seia never logs, echoes or serializes the caller-supplied `api_key`,
and scrubs it from attempt errors.

## Development

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
just test-proxy  # run tests through localhost:7890 proxy (see tests/integration.rs)
```

## Supported Search Engines

| Engine | Auth |
|--------|------|
| [DuckDuckGo](https://duckduckgo.com/) | None |
| [Wikipedia](https://www.mediawiki.org/wiki/API:Search) | None |
| [SearXNG](https://docs.searxng.org/) | `SEARXNG_URL` |
| [Tavily](https://docs.tavily.com/) | `TAVILY_API_KEY` |
| [Bing](https://learn.microsoft.com/en-us/bing/search-apis/bing-web-search/) | `BING_SEARCH_API_KEY` |
| [Brave](https://api.search.brave.com/app/documentation) | `BRAVE_SEARCH_API_KEY` |
| [DeepSeek Search](https://www.deepseek.com/) | `DEEPSEEK_API_KEY` |
| [百度 AI 搜索 (Qianfan)](https://qianfan.cloud.baidu.com/) | `BAIDU_API_KEY` |
| [秘塔 (MetaSo)](https://metaso.cn/search-api/playground) | `METASO_API_KEY` |
| [智谱 (Zhipu)](https://docs.bigmodel.cn/cn/guide/tools/web-search) | `ZHIPU_API_KEY` |
| [博查 (Bocha)](https://open.bochaai.com/docs) | `BOCHA_API_KEY` |


## License

SySL-1.0 (Synthetic Source License). See [LICENSE](./LICENSE) or the [SySL website](https://sysl.celestia.world).

## MCP Server Deployment

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
