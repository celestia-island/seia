# Architecture

seia is a single crate that ships both a library (`src/lib.rs`) and a CLI
(`src/main.rs`). The design goal is **one query surface, many backends**: a
caller picks an `Engine` and gets back the same `SearchResult` regardless of
which backend produced it.

## Module map

```
src/
├── lib.rs          public API surface
├── main.rs         clap CLI (search / engines)
├── engines.rs      Engine enum: as_str, api_key_env, needs_key
├── engines_impl/   one module per backend
│   ├── duckduckgo.rs   scrape (HTML)
│   ├── wikipedia.rs    API (JSON)
│   ├── tavily.rs       API (JSON, key)
│   ├── searxng.rs      API (JSON, self-hosted)
│   ├── bing.rs         API (JSON, key)
│   ├── brave.rs        API (JSON, key)
│   ├── zhipu.rs        API (JSON, key — 智谱 Web Search)
│   └── bocha.rs        API (JSON, key — 博查 Web Search)
├── client.rs       SearchClient + SearchOptions
├── extractor.rs    full-page content fetcher (for --fetch)
└── result.rs       SearchResult / SearchItem / SearchMode
```

## Two execution paths, one result type

All paths converge on [`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs):

```
query + Engine ─► SearchClient ─► engines_impl/* ─► unify ─► SearchResult
```

- **API** — `engines_impl::<engine>::search(&http, query, &opts)` calls the
  provider, deserialises JSON into `SearchItem`s.
- **Scrape** — same signature, but parses the HTML results page.

`SearchMode` (`Api` / `Scrape`) records which path produced a result, so
callers can distinguish a structured API answer from a scraped page.

## Dispatch

`SearchClient::search_with_options` is a flat `match` on `Engine`. Adding a
backend means: implement one function in `engines_impl/`, add an `Engine`
variant, add a `match` arm. There is no trait object or dynamic dispatch — the
set of engines is closed and known at compile time, which keeps the API
predictable and the binary small.

## No headless browser

seia deliberately ships **no** browser automation. Every backend is a plain
HTTP client. Engines that aggressively block non-browser traffic (Google,
Baidu, Yandex web search) are out of scope — reach them through their official
APIs or a dedicated browser tool such as
[shirabe](https://github.com/celestia-island/shirabe) when one becomes
available as a standalone MCP.

## Content enrichment

`SearchOptions::fetch_content` is an orthogonal concern: after the engine
returns `SearchItem`s, `extractor::fetch_content` downloads and cleans each
page. This is engine-agnostic.
