# Browser Mode

Some engines — Google, Baidu, Bing (the web page, not the API), Yandex —
aggressively block non-browser requests. seia drives them through
[tairitsu](https://github.com/celestia-island/tairitsu), a headless browser
runtime. seia speaks tairitsu's HTTP debug API, so there are **no** native
browser bindings.

## Two ways to run tairitsu

### 1. External daemon (default)

Run a tairitsu debug server out-of-band and point seia at it:

```bash
# in one terminal
tairitsu debug --proxy http://localhost:7890

# in another
seia search "rust async" --engine google --browser --tairitsu http://127.0.0.1:3001
```

This keeps the heavy browser process out of your application binary.

### 2. Embedded (the `embedded-browser` feature)

Compile tairitsu's debug server *into* seia. No separate daemon needed:

```toml
[dependencies]
seia = { version = "0.1", features = ["embedded-browser"] }
```

```bash
seia search "rust async" --engine google --browser --embedded
```

The `embedded` flag spawns the in-process server (see
[`seia::embedded::start`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs)).

## How a browser search works

Each browser search is three steps, all issued against the tairitsu HTTP API:

1. **Navigate** — `POST /navigate` to the engine's search URL.
2. **Wait** — `POST /wait-for-selector` until the results container renders.
3. **Extract** — `POST /evaluate` runs a snippet of JS that reads titles, links
   and snippets out of the DOM.

The selectors and URL template for each engine live in a
[`SearchProfile`](https://github.com/celestia-island/seia/blob/dev/src/profiles.rs):

| Profile | Search URL | Result container |
| --- | --- | --- |
| `google` | `google.com/search?q=` | `div.g` |
| `baidu` | `baidu.com/s?wd=` | `div.result, div.c-container` |
| `bing_web` | `bing.com/search?q=` | `li.b_algo` |
| `yandex` | `yandex.com/search/?text=` | `li.serps-item, div.Organic` |

## Using the browser client directly

```rust
use seia::{BrowserClient, profiles};

let client = BrowserClient::new("http://127.0.0.1:3001");

if !client.health().await.unwrap_or(false) {
    panic!("tairitsu browser not connected");
}

let profile = profiles::get_profile("google").unwrap();
let result = client.search("rust async", profile).await?;
```

The CLI maps `--engine <name> --browser` to the matching profile internally
(falling back to the `google` profile when none matches).
