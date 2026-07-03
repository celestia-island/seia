# Engines

seia supports 9 backends, all reached through their official HTTP API (or,
where no API exists, lightweight HTML scraping). There is no headless
browser — seia is a pure HTTP client, so every engine works from both the
CLI and the library through the same `Engine` enum.

Most engines expose a free tier; the ones that need a key read it from a
documented environment variable, so no key ever appears in code or CLI args.

## Two execution modes

| Mode | How it works | Used by |
| --- | --- | --- |
| **API** | Calls a search provider's HTTP API, parses JSON. | Tavily, SearXNG, Wikipedia, Bing, Brave, 智谱, 博查, 秘塔 |
| **Scrape** | Fetches the lightweight HTML results page, extracts hits. | DuckDuckGo |

## Engine matrix

### International

| Engine | Enum | Mode | Auth | Free tier | Status |
| --- | --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | Scrape | none | unlimited | ✅ |
| Wikipedia | `Wikipedia` | API | none | unlimited | ✅ |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | self-hosted | ✅ |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1 000/month | ✅ |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1 000/month | ✅ |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2 000/month | ✅ |

### Domestic (China)

| Engine | Enum | Mode | Auth | Status |
| --- | --- | --- | --- | --- |
| 智谱 (Zhipu / BigModel) | `Zhipu` | API | `ZHIPU_API_KEY` | ✅ |
| 博查 (Bocha) | `Bocha` | API | `BOCHA_API_KEY` | ✅ |
| 秘塔 (Metaso) | `Metaso` | API | `METASO_API_KEY` | ✅ |

> 智谱's Web Search API can route through one of several backing engines —
> 智谱基础版 (`search_std`, default), 智谱高阶版 (`search_pro`), 搜狗
> (`search_pro_sogou`), or 夸克 (`search_pro_quark`). Select one with the
> `ZHIPU_SEARCH_ENGINE` env var.

> 博查 returns both a short `snippet` and a longer LLM-generated `summary` per
> page; seia surfaces whichever is longer as the result's `snippet`.

> 秘塔's search scope defaults to `webpage`; override it with the
> `METASO_SCOPE` env var (e.g. `academic`). Its response envelope is parsed
> defensively, so it tolerates either `{data:{results:[…]}}` or `{data:[…]}`.

## Selecting an engine

CLI:

```bash
seia search "query" --engine wikipedia
seia search "查询" --engine zhipu      # needs ZHIPU_API_KEY
```

Library:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("query", Engine::Wikipedia).await?;
client.search("查询", Engine::Zhipu).await?;   // needs ZHIPU_API_KEY
```

## Inspecting engine metadata

`Engine` carries its own metadata:

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing, Engine::Zhipu, Engine::Metaso] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing / zhipu
    println!("  needs key? {}", engine.needs_key());
    println!("  key env:    {:?}", engine.api_key_env());
}
```
