# 函式庫的用法

CLI 能做的一切都可作為函式庫使用。公開 API 位於 crate 根的
[`lib.rs`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs)。

## 搜尋用戶端

`SearchClient` 持有一個 `reqwest::Client`（帶類瀏覽器 UA、15 秒逾時），按
`Engine` 分派到正確的後端：

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client.search("rust 非同步", Engine::Duckduckgo).await?;
```

### 指定代理

`SearchClient::with_proxy` 設定明確代理。用戶端也會經由 reqwest 自動遵循
`HTTPS_PROXY` / `HTTP_PROXY`。

```rust
let client = SearchClient::with_proxy("http://localhost:7890");
```

## 搜尋選項

`search_with_options` 接收 `SearchOptions`，用於限制數量、擷取本文以及指定
SearXNG 實例位址：

```rust
use seia::{SearchClient, Engine, SearchOptions};

let opts = SearchOptions {
    limit: Some(5),
    fetch_content: true,        // 下載每頁本文（較慢）
    searxng_url: Some("http://localhost:8080".into()),
};

let result = client
    .search_with_options("查詢", Engine::Searxng, opts)
    .await?;
```

| 欄位 | 預設值 | 含義 |
| --- | --- | --- |
| `limit` | `Some(10)` | 截斷結果清單。 |
| `fetch_content` | `false` | 擷取完整頁面本文到 `SearchItem::content`。 |
| `searxng_url` | `None` | SearXNG 基底位址（否則從 `SEARXNG_URL` 讀取）。 |

## 跨引擎備援

`search_fallback` 按順序嘗試一組引擎，回傳第一個非空結果 —— 當免費後端被限速時很實用：

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client
    .search_fallback("查詢", &[Engine::Duckduckgo, Engine::Wikipedia])
    .await?;
```

## 結果型別

```rust
pub struct SearchResult {
    pub engine: String,
    pub query: String,
    pub items: Vec<SearchItem>,
    pub elapsed_ms: u64,
}

pub struct SearchItem {
    pub title: String,
    pub url: String,
    pub snippet: Option<String>,   // 引擎回傳的簡短摘要
    pub content: Option<String>,   // 完整頁面本文（僅 --fetch）
}
```

兩者皆推導 `Serialize`/`Deserialize`，因此 `serde_json::to_string(&result)`
的輸出與 `seia search --json` 完全一致。

## Prelude

更簡潔的匯入：

```rust
use seia::prelude::*;   // SearchClient, Engine, SearchResult, SearchItem, SearchMode
```
