# 库的用法

CLI 能做的一切都可作为库使用。公开 API 位于 crate 根的
[`lib.rs`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs)。

## 搜索客户端

`SearchClient` 持有一个 `reqwest::Client`（带类浏览器 UA、15 秒超时），按
`Engine` 分派到正确的后端：

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client.search("rust 异步", Engine::Duckduckgo).await?;
```

### 指定代理

`SearchClient::with_proxy` 配置显式代理。客户端也会经由 reqwest 自动遵循
`HTTPS_PROXY` / `HTTP_PROXY`。

```rust
let client = SearchClient::with_proxy("http://localhost:7890");
```

## 搜索选项

`search_with_options` 接收 `SearchOptions`，用于限制数量、抓取正文以及指定
SearXNG 实例地址：

```rust
use seia::{SearchClient, Engine, SearchOptions};

let opts = SearchOptions {
    limit: Some(5),
    fetch_content: true,        // 下载每页正文（较慢）
    searxng_url: Some("http://localhost:8080".into()),
};

let result = client
    .search_with_options("查询", Engine::Searxng, opts)
    .await?;
```

| 字段 | 默认值 | 含义 |
| --- | --- | --- |
| `limit` | `Some(10)` | 截断结果列表。 |
| `fetch_content` | `false` | 抓取完整页面正文到 `SearchItem::content`。 |
| `searxng_url` | `None` | SearXNG 基址（否则从 `SEARXNG_URL` 读取）。 |

## 跨引擎回退

`search_fallback` 按顺序尝试一组引擎，返回第一个非空结果 —— 当免费后端被限速时很有用：

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client
    .search_fallback("查询", &[Engine::Duckduckgo, Engine::Wikipedia])
    .await?;
```

## 结果类型

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
    pub snippet: Option<String>,   // 引擎返回的简短摘要
    pub content: Option<String>,   // 完整页面正文（仅 --fetch）
}
```

二者均派生 `Serialize`/`Deserialize`，因此 `serde_json::to_string(&result)`
的输出与 `seia search --json` 完全一致。

## Prelude

更简洁的导入：

```rust
use seia::prelude::*;   // SearchClient, Engine, SearchResult, SearchItem, SearchMode
```
