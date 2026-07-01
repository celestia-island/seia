<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>あらゆる情報源から知識を</strong>

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](../../LICENSE)
[![Crates.io](https://img.shields.io/crates/v/seia)](https://docs.rs/seia)
[![CI](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)

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
# 基本検索（API キー不要）
seia search "rust async patterns"

# 特定のエンジンを選択
seia search "Klein bottle" --engine wikipedia

# JSON 出力
seia search "climate change" --json

# プロキシ経由
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

### ライブラリ

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Wikipedia).await?;
```

## 開発

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## 対応検索エンジン

| エンジン | 認証 | 状態 |
|--------|------|--------|
| [Wikipedia](https://www.mediawiki.org/wiki/API:Search) | なし | ✅ |
| [SearXNG](https://docs.searxng.org/) | `SEARXNG_URL` | ✅ |
| [Tavily](https://docs.tavily.com/) | `TAVILY_API_KEY` | ✅ |
| [Bing](https://learn.microsoft.com/en-us/bing/search-apis/bing-web-search/) | `BING_SEARCH_API_KEY` | ✅ |
| [Brave](https://api.search.brave.com/app/documentation) | `BRAVE_SEARCH_API_KEY` | ✅ |
| [智谱 (Zhipu)](https://docs.bigmodel.cn/cn/guide/tools/web-search) | `ZHIPU_API_KEY` | ✅ |
| [博查 (Bocha)](https://open.bochaai.com/docs) | `BOCHA_API_KEY` | ✅ |

## ライセンス

SySL-1.0（Synthetic Source License）。詳しくは [LICENSE](../../LICENSE) を参照してください。
