<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>あらゆる情報源から知識を</strong>

マルチエンジン Web 検索ライブラリ兼 CLI。

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

# ブラウザモード（ヘッドレス、APIキー不要）
seia search "query" --browser --browser-engine google
```

### ライブラリ

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Duckduckgo).await?;
```

## 開発

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## 対応検索エンジン

### API / スクレイプエンジン

| エンジン | 公式サイト | モード | 認証 | 無料枠 | 状態 |
|--------|--------|------|------|------|--------|
| DuckDuckGo | [duckduckgo.com](https://duckduckgo.com) | スクレイプ | なし | 無制限 | ✅ |
| Wikipedia | [wikipedia.org](https://www.wikipedia.org) | API | なし | 無制限 | ✅ |
| SearXNG | [searxng.org](https://searxng.org) | API | `SEARXNG_URL` | セルフホスト | ✅ |
| Tavily | [tavily.com](https://tavily.com) | API | `TAVILY_API_KEY` | 1 000/月 | ✅ |
| Bing | [bing.com](https://www.bing.com) | API | `BING_SEARCH_API_KEY` | 1 000/月 | 🔜 |
| Brave | [brave.com/search](https://brave.com/search) | API | `BRAVE_SEARCH_API_KEY` | 2 000/月 | 🔜 |

> Bing と Brave の API バックエンドはスタブ（未実装）です。ブラウザプロファイルで代用するか、
> [コントリビュート](https://github.com/celestia-island/seia)してください。

### ブラウザエンジン（CLI のみ）

| エンジン | 公式サイト | 認証 | 説明 |
|--------|---------|------|------|
| Google | [google.com](https://www.google.com) | なし（tairitsu 経由でスクレイプ） | Google ウェブ検索 |
| Baidu | [baidu.com](https://www.baidu.com) | なし（tairitsu 経由でスクレイプ） | Baidu ウェブ検索 |
| Bing Web | [bing.com](https://www.bing.com) | なし（tairitsu 経由でスクレイプ） | Bing ウェブ結果 |
| Yandex | [yandex.com](https://yandex.com) | なし（tairitsu 経由でスクレイプ） | Yandex ウェブ検索 |

ブラウザモードのエンジンはヘッドレスレンダリングに [tairitsu](https://github.com/celestia-island/tairitsu)
を使用します。スタンドアロンのデーモンを動かすか、`embedded-browser` feature を有効にして
tairitsu をプロセス内に組み込んでください。

> ほとんどの検索エンジンは公式 REST API を提供しています。ブラウザプロファイルは API バックエンドが
> 未実装の場合、または API が無料で利用できない場合の代替手段です。長期的には、各ブラウザ
> プロファイルに対応する `Engine` バリアントを追加し、API キーサポートを提供する予定です。

## ライセンス

SySL-1.0（Synthetic Source License）。詳しくは [LICENSE](../../LICENSE) を参照してください。
