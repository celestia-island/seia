# 架構

seia 是單一 crate，同時提供函式庫（`src/lib.rs`）與 CLI（`src/main.rs`）。設計目標是
**一個查詢入口，多個後端**：呼叫端選擇一個 `Engine`，無論結果如何取得，都拿到同一個
`SearchResult`。

## 模組地圖

```
src/
├── lib.rs          公開 API 面 + embedded-browser 伺服器
├── main.rs         clap CLI（search / engines）
├── engines.rs      Engine 列舉：as_str、api_key_env、needs_key、needs_browser
├── engines_impl/   每個 API/擷取後端一個模組
│   ├── duckduckgo.rs   擷取（HTML）
│   ├── wikipedia.rs    API（JSON）
│   ├── tavily.rs       API（JSON，需金鑰）
│   └── searxng.rs      API（JSON，自行託管）
├── client.rs       SearchClient + SearchOptions（API/擷取路徑）
├── browser.rs      BrowserClient（經 HTTP 與 tairitsu 通訊）
├── profiles.rs     SearchProfile：每個引擎的 CSS 選擇器 + URL 模板
├── extractor.rs    完整頁面本文擷取器（用於 --fetch）
└── result.rs       SearchResult / SearchItem / SearchMode
```

## 三條執行路徑，一種結果型別

三條路徑都匯聚到
[`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs)：

```
                       ┌─ engines_impl/*（API / 擷取）─┐
query + Engine ─► SearchClient ─► 統一 ─► SearchResult
                       └─ browser.rs（tairitsu HTTP）──┘
```

- **API** —— `engines_impl::<engine>::search(&http, query, &opts)` 呼叫服務商，把 JSON
  反序列化成 `SearchItem`。
- **擷取** —— 簽章相同，但解析的是 HTML 結果頁。
- **瀏覽器** —— `BrowserClient::search` 驅動 tairitsu；每個引擎的 `SearchProfile`
  提供 URL 與注入的擷取 JS 所用的 CSS 選擇器。

`SearchMode`（`Api` / `Scrape` / `Browser`）記錄結果由哪條路徑產生，呼叫端據此可區分，
例如快取 API 答案與渲染頁面。

## 分派

`SearchClient::search_with_options` 是對 `Engine` 的扁平 `match`。新增後端意味著：在
`engines_impl/` 實作一個函式，新增一個 `Engine` 變體，新增一個 `match` 分支。沒有
trait object 或動態分派 —— 引擎集合是封閉、編譯期已知的，這讓 API 可預測、二進位檔更小。

## 本文補全

`SearchOptions::fetch_content` 是一個正交關注點：引擎回傳 `SearchItem` 之後，
`extractor::fetch_content` 下載並清洗每個頁面。它與引擎無關，對任意模式都生效。

## 瀏覽器整合邊界

`tairitsu-packager` 是**可選**相依性，由 `embedded-browser` feature 控制。不開啟時 seia
不含任何瀏覽器程式碼，而是透過普通 HTTP 連接外部 tairitsu 常駐程式（`BrowserClient`）。
開啟時 `seia::embedded::start` 在行程內啟動 debug 伺服器。這樣既保證預設建置輕量，又讓
可發布的 crate 不背負沉重的瀏覽器相依性。
