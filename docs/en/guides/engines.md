# Engines

seia exposes every backend through the single [`Engine`](https://github.com/celestia-island/seia/blob/dev/src/engines.rs)
enum, so switching backends never touches your query code.

## Three execution modes

| Mode | How it works | Engines |
| --- | --- | --- |
| **API** | Calls a search provider's HTTP API and parses JSON. | Tavily, SearXNG, Wikipedia |
| **Scrape** | Fetches the HTML results page and extracts hits. | DuckDuckGo |
| **Browser** | Drives a headless browser (via [tairitsu](https://github.com/celestia-island/tairitsu)) to render JS-heavy pages. | Google, Baidu, Bing (web), Yandex |

API and scrape modes need nothing but an HTTP client. Browser mode is
described in [Browser Mode](./browser-mode.md).

## Engine matrix

| Engine | Enum value | Mode | Auth | Free tier |
| --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | Scrape | none | unlimited |
| Wikipedia | `Wikipedia` | API | none | unlimited |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | self-hosted |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1 000 / month |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1 000 / month |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2 000 / month |
| Google | browser profile | Browser | tairitsu | — |
| Baidu | browser profile | Browser | tairitsu | — |
| Bing (web) | browser profile | Browser | tairitsu | — |
| Yandex | browser profile | Browser | tairitsu | — |

> Bing and Brave API backends are stubbed (`Engine::Bing` / `Engine::Brave`
> return an "not yet implemented" error). Use the browser profiles or
> [contribute](https://github.com/celestia-island/seia) an implementation.

## Selecting an engine

CLI:

```bash
seia search "query" --engine wikipedia
```

Library:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("query", Engine::Wikipedia).await?;
```

## Inspecting engine metadata

`Engine` carries its own metadata, so you can build UIs without hard-coding:

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing
    println!("  needs key? {}", engine.needs_key());
    println!("  key env:    {:?}", engine.api_key_env());
}
```
