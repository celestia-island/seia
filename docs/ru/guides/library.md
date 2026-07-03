# Использование библиотеки

Всё, что умеет CLI, доступно и как библиотека. Публичный API находится в корне
крейта в [`lib.rs`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs).

## Клиент поиска

`SearchClient` владеет единственным `reqwest::Client` (с user agent,
имитирующим браузер, и таймаутом 15 с) и маршрутизирует запросы к нужному
бэкенду по `Engine`:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client.search("rust async", Engine::Duckduckgo).await?;
```

### С прокси

`SearchClient::with_proxy` настраивает явный прокси. Клиент также
автоматически учитывает `HTTPS_PROXY` / `HTTP_PROXY` через reqwest.

```rust
let client = SearchClient::with_proxy("http://localhost:7890");
```

## Параметры поиска

`search_with_options` принимает `SearchOptions` для ограничения, выборки
содержимого и URL экземпляра SearXNG:

```rust
use seia::{SearchClient, Engine, SearchOptions};

let opts = SearchOptions {
    limit: Some(5),
    fetch_content: true,        // загрузить текст каждой страницы (медленнее)
    searxng_url: Some("http://localhost:8080".into()),
};

let result = client
    .search_with_options("query", Engine::Searxng, opts)
    .await?;
```

| Поле | По умолчанию | Значение |
| --- | --- | --- |
| `limit` | `Some(10)` | Обрезать список результатов. |
| `fetch_content` | `false` | Загрузить полный текст страницы в `SearchItem::content`. |
| `searxng_url` | `None` | Базовый URL SearXNG (иначе берётся из `SEARXNG_URL`). |

## Резервирование между движками

`search_fallback` перебирает список движков по порядку и возвращает первый
непустой результат — удобно, когда бесплатный бэкенд упирается в лимиты:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client
    .search_fallback("query", &[Engine::Duckduckgo, Engine::Wikipedia])
    .await?;
```

## Типы результатов

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
    pub snippet: Option<String>,   // краткая аннотация от движка
    pub content: Option<String>,   // полный текст страницы (только с --fetch)
}
```

Оба реализуют `Serialize`/`Deserialize`, поэтому `serde_json::to_string(&result)`
даёт ровно то же, что печатает `seia search --json`.

## Prelude

Для более лаконичных импортов:

```rust
use seia::prelude::*;   // SearchClient, Engine, SearchResult, SearchItem, SearchMode
```
