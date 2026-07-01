# Architecture

seia is a single crate that ships both a library (`src/lib.rs`) and a CLI
(`src/main.rs`). The design goal is **one query surface, many backends**: a
caller picks an `Engine` and gets back the same `SearchResult` regardless of
how the result was obtained.

## Module map

```
src/
├── lib.rs          public API surface + embedded-browser server
├── main.rs         clap CLI (search / engines)
├── engines.rs      Engine enum: as_str, api_key_env, needs_key, needs_browser
├── engines_impl/   one module per API/scrape backend
│   ├── duckduckgo.rs   scrape (HTML)
│   ├── wikipedia.rs    API (JSON)
│   ├── tavily.rs       API (JSON, key)
│   └── searxng.rs      API (JSON, self-hosted)
├── client.rs       SearchClient + SearchOptions (API/scrape path)
├── browser.rs      BrowserClient (talks to tairitsu over HTTP)
├── profiles.rs     SearchProfile: per-engine CSS selectors + URL template
├── extractor.rs    full-page content fetcher (for --fetch)
└── result.rs       SearchResult / SearchItem / SearchMode
```

## Three execution paths, one result type

All three paths converge on [`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs):

```
                       ┌─ engines_impl/* (API / scrape) ─┐
query + Engine ─► SearchClient ─► unify ─► SearchResult
                       └─ browser.rs (tairitsu HTTP) ────┘
```

- **API** — `engines_impl::<engine>::search(&http, query, &opts)` calls the
  provider, deserialises JSON into `SearchItem`s.
- **Scrape** — same signature, but parses the HTML results page.
- **Browser** — `BrowserClient::search` drives tairitsu; the per-engine
  `SearchProfile` supplies the URL and the CSS selectors used by the injected
  extraction JS.

`SearchMode` (`Api` / `Scrape` / `Browser`) records which path produced a
result, so callers can distinguish, e.g., a cached API answer from a rendered
page.

## Dispatch

`SearchClient::search_with_options` is a flat `match` on `Engine`. Adding a
backend means: implement one function in `engines_impl/`, add an `Engine`
variant, add a `match` arm. There is no trait object or dynamic dispatch — the
set of engines is closed and known at compile time, which keeps the API
predictable and the binary small.

## Content enrichment

`SearchOptions::fetch_content` is an orthogonal concern: after the engine
returns `SearchItem`s, `extractor::fetch_content` downloads and cleans each
page. This is engine-agnostic and works for any mode.

## Browser integration boundary

`tairitsu-packager` is an **optional** dependency, gated behind the
`embedded-browser` feature. Without it, seia has zero browser code and
connects to an external tairitsu daemon over plain HTTP (`BrowserClient`).
With it, `seia::embedded::start` spawns the debug server in-process. This keeps
the default build light and the publishable crate free of heavy browser
dependencies.
