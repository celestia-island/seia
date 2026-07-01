# 引擎

seia 透過單一的 [`Engine`](https://github.com/celestia-island/seia/blob/dev/src/engines.rs)
列舉暴露所有後端，因此切換後端永遠不會影響你的查詢程式碼。

## 三種執行模式

| 模式 | 運作方式 | 引擎 |
| --- | --- | --- |
| **API** | 呼叫搜尋服務商的 HTTP API 並解析 JSON。 | Tavily、SearXNG、Wikipedia |
| **擷取** | 抓取 HTML 結果頁並擷取命中。 | DuckDuckGo |
| **瀏覽器** | 驅動無頭瀏覽器（經由 [tairitsu](https://github.com/celestia-island/tairitsu)）渲染 JS 密集頁面。 | Google、Baidu、Bing（網頁）、Yandex |

API 與擷取模式只需要一個 HTTP 用戶端。瀏覽器模式見
[瀏覽器模式](./browser-mode.md)。

## 引擎矩陣

| 引擎 | 列舉值 | 模式 | 認證 | 免費額度 |
| --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | 擷取 | 無 | 無限 |
| Wikipedia | `Wikipedia` | API | 無 | 無限 |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | 自行託管 |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1000 / 月 |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1000 / 月 |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2000 / 月 |
| Google | 瀏覽器 profile | 瀏覽器 | tairitsu | — |
| Baidu | 瀏覽器 profile | 瀏覽器 | tairitsu | — |
| Bing（網頁） | 瀏覽器 profile | 瀏覽器 | tairitsu | — |
| Yandex | 瀏覽器 profile | 瀏覽器 | tairitsu | — |

> Bing 與 Brave 的 API 後端目前是佔位實作（`Engine::Bing` / `Engine::Brave`
> 會回傳 "not yet implemented" 錯誤）。請使用瀏覽器 profile，或
> [貢獻](https://github.com/celestia-island/seia) 實作。

## 選擇引擎

CLI：

```bash
seia search "查詢" --engine wikipedia
```

函式庫：

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("查詢", Engine::Wikipedia).await?;
```

## 查詢引擎元資訊

`Engine` 自帶元資訊，因此無需硬編碼即可建構 UI：

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing
    println!("  需要金鑰? {}", engine.needs_key());
    println!("  金鑰環境變數: {:?}", engine.api_key_env());
}
```
