# 라이브러리 사용법

CLI가 할 수 있는 모든 작업은 라이브러리로도 사용할 수 있습니다. 공개 API는 crate 루트의
[`lib.rs`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs)에 있습니다.

## 검색 클라이언트

`SearchClient`는 단일 `reqwest::Client`(브라우저와 유사한 사용자 에이전트와 15초 타임아웃)를
소유하고 `Engine`에 따라 올바른 백엔드로 디스패치합니다:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client.search("rust async", Engine::Duckduckgo).await?;
```

### 프록시 사용

`SearchClient::with_proxy`는 명시적 프록시를 구성합니다. 클라이언트는 reqwest를 통해
`HTTPS_PROXY` / `HTTP_PROXY`도 자동으로 따릅니다.

```rust
let client = SearchClient::with_proxy("http://localhost:7890");
```

## 검색 옵션

`search_with_options`는 결과 제한, 본문 가져오기, SearXNG 인스턴스 URL을 위해
`SearchOptions`를 받습니다:

```rust
use seia::{SearchClient, Engine, SearchOptions};

let opts = SearchOptions {
    limit: Some(5),
    fetch_content: true,        // 각 페이지의 본문 다운로드 (느림)
    searxng_url: Some("http://localhost:8080".into()),
};

let result = client
    .search_with_options("query", Engine::Searxng, opts)
    .await?;
```

| 필드 | 기본값 | 의미 |
| --- | --- | --- |
| `limit` | `Some(10)` | 결과 목록을 자릅니다. |
| `fetch_content` | `false` | 전체 페이지 본문을 `SearchItem::content`에 가져옵니다. |
| `searxng_url` | `None` | SearXNG 기본 URL (지정하지 않으면 `SEARXNG_URL`에서 읽음). |

## 엔진 간 폴백

`search_fallback`은 엔진 목록을 순서대로 시도하고 비어 있지 않은 첫 번째 결과를
반환합니다 — 무료 백엔드가 속도 제한을 받을 때 유용합니다:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client
    .search_fallback("query", &[Engine::Duckduckgo, Engine::Wikipedia])
    .await?;
```

## 결과 타입

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
    pub snippet: Option<String>,   // 엔진이 반환한 짧은 요약
    pub content: Option<String>,   // 전체 페이지 본문 (--fetch 전용)
}
```

둘 다 `Serialize`/`Deserialize`를 derive하므로, `serde_json::to_string(&result)`는
`seia search --json`이 출력하는 것과 정확히 같은 결과를 제공합니다.

## Prelude

간결한 임포트를 위해:

```rust
use seia::prelude::*;   // SearchClient, Engine, SearchResult, SearchItem, SearchMode
```
