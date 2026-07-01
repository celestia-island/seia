# seia — 通用搜尋引擎抽象層

**一次查詢，所有搜尋引擎。**

Rust 多後端 Web 搜尋函式庫與 CLI。免費引擎開箱即用。

## 簡介

seia 提供統一的介面來存取多個搜尋後端 —— DuckDuckGo、Tavily、Wikipedia、SearXNG、
Bing、Brave、Google、Baidu 等。免費引擎零設定即可使用。

## 快速開始

### CLI

```bash
# 基礎搜尋（DuckDuckGo，免費、無需金鑰）
seia search "rust 非同步模式"

# Wikipedia（免費、學術）
seia search "克萊因瓶" --engine wikipedia

# JSON 輸出
seia search "氣候變遷" --json

# 透過代理
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# 瀏覽器模式（Google/Baidu 經由 tairitsu）
seia search "查詢" --engine google --browser
```

### 作為函式庫使用

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust 非同步", Engine::DuckDuckGo).await?;
```

## 引擎

| 引擎 | 模式 | 認證 | 狀態 |
|------|------|------|------|
| DuckDuckGo | 擷取 | 無 | ✅ |
| Wikipedia | API | 無 | ✅ |
| SearXNG | API | `SEARXNG_URL` | ✅ |
| Tavily | API | `TAVILY_API_KEY` | ✅ |
| Bing | API | `BING_SEARCH_API_KEY` | 🔲 |
| Brave | API | `BRAVE_SEARCH_API_KEY` | 🔲 |
| Google | 瀏覽器 | tairitsu | ✅ |
| Baidu | 瀏覽器 | tairitsu | ✅ |
| Bing Web | 瀏覽器 | tairitsu | ✅ |
| Yandex | 瀏覽器 | tairitsu | ✅ |

瀏覽器模式引擎使用 [tairitsu](https://github.com/celestia-island/tairitsu)
進行無頭渲染。可以執行獨立常駐程式，或啟用 `embedded-browser` feature 將 tairitsu
編譯進行程。

## 開發

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## 授權

SySL-1.0（合成原始碼授權）。詳見 [LICENSE](../../LICENSE)。
