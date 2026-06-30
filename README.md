# seia — Universal Search Engine Abstraction

A general-purpose, multi-backend web search library and CLI. Designed to be embedded in any application or used standalone.

## Design

Two execution modes:
- **API mode**: calls search provider APIs (Tavily, Bing, Brave, etc.)
- **Browser mode**: drives a headless browser for engines without APIs (DuckDuckGo, Google, Baidu)

Both modes share a unified `SearchResult` interface.

## Engines

| Engine | Mode | Auth | Status |
|--------|------|------|--------|
| DuckDuckGo | Browser (HTML scrape) | None | ✅ Implemented |
| Tavily | API | API key | ✅ Implemented |
| SearXNG | API (self-hosted) | None | ✅ Implemented |
| Wikipedia | API | None | ✅ Implemented |
| Bing | API | API key | 🔲 Planned |
| Brave | API | API key | 🔲 Planned |
| Google (via Serper) | API | API key | 🔲 Planned |

## CLI

```bash
# Quick search
seia search "rust async patterns"

# Specify engine
seia search "capital of France" --engine wikipedia

# JSON output
seia search "climate change 2026" --engine tavily --json

# With content extraction
seia search "how to implement red-black tree" --engine duckduckgo --fetch

# List available engines
seia engines
```

## Library

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async patterns", Engine::DuckDuckGo).await?;
for r in results {
    println!("{}: {}", r.title, r.url);
}
```

## License

BUSL-1.1
