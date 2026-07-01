<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>閱覽網際網路上的知識</strong>

多引擎 Web 搜尋函式庫與 CLI。

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

# 瀏覽器模式（無頭繪製，無需 API 金鑰）
seia search "查詢" --browser --browser-engine google
```

### 作為函式庫使用

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust 非同步", Engine::Duckduckgo).await?;
```

## 開發

```bash
just ci          # 格式化檢查 + clippy + 測試
just test        # 執行測試
just test-proxy  # 透過 localhost:7890 代理執行測試
```

## 支援的搜尋引擎

### API / 爬取引擎

| 引擎 | 官網 | 模式 | 認證 | 免費額度 | 狀態 |
|------|------|------|------|---------|------|
| DuckDuckGo | [duckduckgo.com](https://duckduckgo.com) | 爬取 | 無 | 無限 | ✅ |
| Wikipedia | [wikipedia.org](https://www.wikipedia.org) | API | 無 | 無限 | ✅ |
| SearXNG | [searxng.org](https://searxng.org) | API | `SEARXNG_URL` | 自建 | ✅ |
| Tavily | [tavily.com](https://tavily.com) | API | `TAVILY_API_KEY` | 1 000 / 月 | ✅ |
| Bing | [bing.com](https://www.bing.com) | API | `BING_SEARCH_API_KEY` | 1 000 / 月 | 🔜 |
| Brave | [brave.com/search](https://brave.com/search) | API | `BRAVE_SEARCH_API_KEY` | 2 000 / 月 | 🔜 |

> Bing 和 Brave 的 API 後端目前是佔位實現（返回「尚未實現」）。可以先用對應瀏
> 覽器 profile 暫代，或[貢獻實現](https://github.com/celestia-island/seia)。

### 瀏覽器引擎（僅 CLI）

| 引擎 | 官網 | 認證 | 說明 |
|------|------|------|------|
| Google | [google.com](https://www.google.com) | 無（透過 tairitsu 爬取） | Google 網頁搜尋 |
| Baidu | [baidu.com](https://www.baidu.com) | 無（透過 tairitsu 爬取） | 百度網頁搜尋 |
| Bing Web | [bing.com](https://www.bing.com) | 無（透過 tairitsu 爬取） | Bing 網頁結果 |
| Yandex | [yandex.com](https://yandex.com) | 無（透過 tairitsu 爬取） | Yandex 網頁搜尋 |

瀏覽器模式引擎使用 [tairitsu](https://github.com/celestia-island/tairitsu)
進行無頭繪製。可以執行獨立守護行程，或啟用 `embedded-browser` feature 將
tairitsu 編譯進行程。

> 大多數搜尋引擎都提供官方 REST API。瀏覽器 profile 是 API 後端尚未實現時的臨時
> 方案，或者是 API 並非免費時的替代路徑。長期計劃是為每個瀏覽器 profile 增加對應
> 的 `Engine` 變體並提供 API-key 支援。

## 授權條款

SySL-1.0（合成原始碼授權）。詳見 [LICENSE](../../LICENSE)。
