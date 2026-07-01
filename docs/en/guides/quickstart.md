# Quick Start

## Install

```bash
# From crates.io (once published)
cargo install seia

# From source
cargo install --path .
```

## First search (CLI)

The default engine is DuckDuckGo — free, no key, works immediately:

```bash
seia search "rust async patterns"

# Choose another engine
seia search "Klein bottle" --engine wikipedia

# Machine-readable output
seia search "climate change" --json

# Pull the full page text for each result (slower)
seia search "tokio runtime" --fetch
```

Run `seia engines` to list every engine and whether it needs a key.

## Engines that need a key

Export the key in your shell — seia reads it automatically:

```bash
export TAVILY_API_KEY=tvly-xxxxx
seia search "react server components" --engine tavily

export SEARXNG_URL=http://localhost:8080
seia search "open source licenses" --engine searxng
```

## Through a proxy

```bash
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# or explicitly
seia search "hello world" --proxy http://localhost:7890
```

## Use it as a library

```rust
use seia::{SearchClient, Engine};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = SearchClient::new();
    let result = client.search("rust async", Engine::Duckduckgo).await?;

    for item in &result.items {
        println!("{} — {}", item.title, item.url);
    }
    Ok(())
}
```

Continue to [Engines](./engines.md) for the full engine matrix, or
[Library Usage](./library.md) for the programmatic API.
