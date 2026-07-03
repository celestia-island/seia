# ライブラリの使い方

CLI でできることはすべてライブラリとして利用できます。公開 API は crate のルートにある
[`lib.rs`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs) にあります。

## 検索クライアント

`SearchClient` は単一の `reqwest::Client`（ブラウザ風のユーザエージェントと 15 秒の
タイムアウト付き）を保持し、`Engine` に応じて適切なバックエンドへディスパッチします。

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client.search("rust async", Engine::Duckduckgo).await?;
```

### プロキシを指定する

`SearchClient::with_proxy` は明示的なプロキシを設定します。また、クライアントは
reqwest 経由で `HTTPS_PROXY` / `HTTP_PROXY` を自動的に尊重します。

```rust
let client = SearchClient::with_proxy("http://localhost:7890");
```

## 検索オプション

`search_with_options` は `SearchOptions` を受け取り、件数制限・本文の取得・
SearXNG インスタンスの URL を制御します。

```rust
use seia::{SearchClient, Engine, SearchOptions};

let opts = SearchOptions {
    limit: Some(5),
    fetch_content: true,        // 各ページの本文をダウンロード（遅くなります）
    searxng_url: Some("http://localhost:8080".into()),
};

let result = client
    .search_with_options("query", Engine::Searxng, opts)
    .await?;
```

| フィールド | デフォルト | 意味 |
| --- | --- | --- |
| `limit` | `Some(10)` | 結果リストを切り詰めます。 |
| `fetch_content` | `false` | ページ全文を `SearchItem::content` に取得します。 |
| `searxng_url` | `None` | SearXNG のベース URL（未指定時は `SEARXNG_URL` から読み取ります）。 |

## エンジンをまたぐフォールバック

`search_fallback` はエンジンのリストを順に試し、最初に空でない結果を返したものを採用します。
無料バックエンドがレート制限される場合に便利です。

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client
    .search_fallback("query", &[Engine::Duckduckgo, Engine::Wikipedia])
    .await?;
```

## 結果の型

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
    pub snippet: Option<String>,   // エンジンからの短い要約
    pub content: Option<String>,   // ページ全文（--fetch 時のみ）
}
```

どちらも `Serialize`/`Deserialize` を導出しているため、`serde_json::to_string(&result)`
は `seia search --json` の出力と完全に一致します。

## Prelude

インポートを簡潔にするには:

```rust
use seia::prelude::*;   // SearchClient, Engine, SearchResult, SearchItem, SearchMode
```
