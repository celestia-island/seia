<p align="center"><img src="https://raw.githubusercontent.com/celestia-island/docs.celestia.world/dev/res/logo/seia.webp" alt="Seia" width="240" /></p>

<h1 align="center">Seia</h1>

<p align="center"><strong>マルチエンジンウェブ検索</strong></p>

<div align="center">

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](https://sysl.celestia.world)
[![GitHub](https://img.shields.io/badge/github-celestia--island%2Fseia-blue.svg)](https://github.com/celestia-island/seia)
[![Checks](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)
[![docs.rs](https://docs.rs/seia/badge.svg)](https://docs.rs/seia)

</div>

<div align="center">

[English](../en/README.md) · [简体中文](../zhs/README.md) ·
[繁體中文](../zht/README.md) · **日本語** ·
[한국어](../ko/README.md) · [Français](../fr/README.md) ·
[Español](../es/README.md) · [Русский](../ru/README.md) ·
[العربية](../ar/README.md)

</div>

## はじめに

seia はマルチエンジン Web 検索ライブラリ兼 CLI です。多様な検索バックエンドを
単一のインターフェースで利用できます。認証不要のエンジンは設定なしですぐに使えます。

## クイックスタート

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

### ライブラリ

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Wikipedia).await?;
```

## MCP サーバー

`mcp` feature を有効にして seia をビルドし、stdio サーバーを実行します——モデルコンテキストプロトコル（Model Context Protocol）経由でマルチエンジン検索クライアントを AI コーディングアシスタントに公開します：

```bash
seia mcp
```

サーバーは3つのツールを提供します：`seia_search`（単一エンジン、デフォルトは duckduckgo でキー不要）、`seia_search_multi`（エンジンチェーンを試し、最初に結果があったものを返す）、`seia_list_engines`（9つのエンジンとAPIキー環境変数の一覧）。MCP クライアントに組み込むには：

```json
{
  "mcpServers": {
    "seia": { "command": "seia", "args": ["mcp"] }
  }
}
```

`SEIA_PROXY` を設定してプロキシ経由で検索リクエストをルーティングします（例：`http://localhost:7890`）；`HTTPS_PROXY` / `HTTP_PROXY` もサポートされています。

## 開発

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
just test-proxy  # run tests through localhost:7890 proxy (see tests/README)
```

## 対応検索エンジン

| エンジン | 認証 |
|--------|------|
| [DuckDuckGo](https://duckduckgo.com/) | なし |
| [Wikipedia](https://www.mediawiki.org/wiki/API:Search) | なし |
| [SearXNG](https://docs.searxng.org/) | `SEARXNG_URL` |
| [Tavily](https://docs.tavily.com/) | `TAVILY_API_KEY` |
| [Bing](https://learn.microsoft.com/en-us/bing/search-apis/bing-web-search/) | `BING_SEARCH_API_KEY` |
| [Brave](https://api.search.brave.com/app/documentation) | `BRAVE_SEARCH_API_KEY` |
| [秘塔 (MetaSo)](https://metaso.cn/search-api/playground) | `METASO_API_KEY` |
| [智谱 (Zhipu)](https://docs.bigmodel.cn/cn/guide/tools/web-search) | `ZHIPU_API_KEY` |
| [博查 (Bocha)](https://open.bochaai.com/docs) | `BOCHA_API_KEY` |

## ライセンス

SySL-1.0（Synthetic Source License）。詳しくは [LICENSE](https://sysl.celestia.world) を参照してください。
