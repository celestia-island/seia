<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>閱覽網際網路上的知識</strong>

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](../../LICENSE)
[![Crates.io](https://img.shields.io/crates/v/seia)](https://docs.rs/seia)
[![CI](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)

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
# 基礎搜尋（無需 API 金鑰）
seia search "rust 非同步模式"

# 選擇某個搜尋引擎
seia search "克萊因瓶" --engine wikipedia

# JSON 輸出
seia search "氣候變遷" --json

# 透過代理
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

### 作為函式庫使用

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust 非同步", Engine::Wikipedia).await?;
```

## 開發

```bash
just ci          # 格式化檢查 + clippy + 測試
just test        # 執行測試
just test-proxy  # 透過 localhost:7890 代理執行測試
```

## 支援的搜尋引擎

| 引擎 | 認證 | 狀態 |
|------|------|------|
| [Wikipedia](https://www.mediawiki.org/wiki/API:Search) | 無 | ✅ |
| [SearXNG](https://docs.searxng.org/) | `SEARXNG_URL` | ✅ |
| [Tavily](https://docs.tavily.com/) | `TAVILY_API_KEY` | ✅ |
| [Bing](https://learn.microsoft.com/en-us/bing/search-apis/bing-web-search/) | `BING_SEARCH_API_KEY` | ✅ |
| [Brave](https://api.search.brave.com/app/documentation) | `BRAVE_SEARCH_API_KEY` | ✅ |
| [智譜 (Zhipu)](https://docs.bigmodel.cn/cn/guide/tools/web-search) | `ZHIPU_API_KEY` | ✅ |
| [博查 (Bocha)](https://open.bochaai.com/docs) | `BOCHA_API_KEY` | ✅ |

## 授權條款

SySL-1.0（合成原始碼授權）。詳見 [LICENSE](../../LICENSE)。
