# Library Usage

Everything the CLI does is available as a library. The public API lives at the
crate root in [`lib.rs`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs).

## The search client

`SearchClient` owns a single `reqwest::Client` (with a browser-like user agent
and a 15 s timeout) and dispatches to the right backend by `Engine`:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client.search("rust async", Engine::Duckduckgo).await?;
```

### With a proxy

`SearchClient::with_proxy` configures an explicit proxy. The client also
honours `HTTPS_PROXY` / `HTTP_PROXY` automatically via reqwest.

```rust
let client = SearchClient::with_proxy("http://localhost:7890");
```

## Search options

`search_with_options` takes a `SearchOptions` for limit, content fetching and
the SearXNG instance URL:

```rust
use seia::{SearchClient, Engine, SearchOptions};

let opts = SearchOptions {
    limit: Some(5),
    fetch_content: true,        // download each page's text (slower)
    searxng_url: Some("http://localhost:8080".into()),
};

let result = client
    .search_with_options("query", Engine::Searxng, opts)
    .await?;
```

| Field | Default | Meaning |
| --- | --- | --- |
| `limit` | `Some(10)` | Truncate the result list. |
| `fetch_content` | `false` | Fetch full page text into `SearchItem::content`. |
| `searxng_url` | `None` | SearXNG base URL (read from `SEARXNG_URL` otherwise). |

## Fallback across engines

`search_fallback` tries a list of engines in order and returns the first that
yields non-empty results — handy when a free backend is rate-limited:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client
    .search_fallback("query", &[Engine::Duckduckgo, Engine::Wikipedia])
    .await?;
```

## Result types

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
    pub snippet: Option<String>,   // short abstract from the engine
    pub content: Option<String>,   // full page text (only with --fetch)
}
```

Both derive `Serialize`/`Deserialize`, so `serde_json::to_string(&result)` gives
you exactly what `seia search --json` prints.

## Prelude

For terser imports:

```rust
use seia::prelude::*;   // SearchClient, Engine, SearchResult, SearchItem, SearchMode
```
