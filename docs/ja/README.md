<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>あらゆる情報源から知識を</strong>

Rust 向けマルチエンジン Web 検索。無料エンジンがすぐに使えます

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

seia は DuckDuckGo、Tavily、Wikipedia、SearXNG、Bing、Brave、Google、Baidu
など多数のバックエンドを、ひとつのインターフェースで扱えるようにします。無料エンジンは
設定なしですぐに使えます。

## クイックスタート

### CLI

```bash
# 基本検索（DuckDuckGo、無料、キー不要）
seia search "rust async patterns"

# Wikipedia（無料、学術）
seia search "Klein bottle" --engine wikipedia

# JSON 出力
seia search "climate change" --json

# プロキシ経由
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# ブラウザモード（Google/Baidu を tairitsu 経由で）
seia search "query" --engine google --browser
```

### ライブラリ

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::DuckDuckGo).await?;
```

## エンジン

| エンジン | モード | 認証 | 状態 |
|--------|------|------|--------|
| DuckDuckGo | スクレイプ | なし | ✅ |
| Wikipedia | API | なし | ✅ |
| SearXNG | API | `SEARXNG_URL` | ✅ |
| Tavily | API | `TAVILY_API_KEY` | ✅ |
| Bing | API | `BING_SEARCH_API_KEY` | 🔲 |
| Brave | API | `BRAVE_SEARCH_API_KEY` | 🔲 |
| Google | ブラウザ | tairitsu | ✅ |
| Baidu | ブラウザ | tairitsu | ✅ |
| Bing Web | ブラウザ | tairitsu | ✅ |
| Yandex | ブラウザ | tairitsu | ✅ |

ブラウザモードのエンジンはヘッドレスレンダリングに [tairitsu](https://github.com/celestia-island/tairitsu)
を使用します。スタンドアロンのデーモンを動かすか、`embedded-browser` feature を有効にして
tairitsu をプロセス内に組み込んでください。

## 開発

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## ライセンス

SySL-1.0（Synthetic Source License）。詳しくは [LICENSE](../../LICENSE) を参照してください。
