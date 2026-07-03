# 架構

seia 是單一 crate，同時提供函式庫（`src/lib.rs`）與 CLI（`src/main.rs`）。設計目標是
**一個查詢入口，多個後端**：呼叫端選擇一個 `Engine`，無論結果由哪個後端產生，都拿到同一個
`SearchResult`。

## 模組地圖

```
src/
├── lib.rs          公開 API 面
├── main.rs         clap CLI（search / engines）
├── engines.rs      Engine 列舉：as_str、api_key_env、needs_key
├── engines_impl/   每個後端一個模組
│   ├── duckduckgo.rs   擷取（HTML）
│   ├── wikipedia.rs    API（JSON）
│   ├── tavily.rs       API（JSON，需金鑰）
│   ├── searxng.rs      API（JSON，自建）
│   ├── bing.rs         API（JSON，需金鑰）
│   ├── brave.rs        API（JSON，需金鑰）
│   ├── zhipu.rs        API（JSON，需金鑰 —— 智譜 Web Search）
│   ├── bocha.rs        API（JSON，需金鑰 —— 博查 Web Search）
│   └── metaso.rs       API（JSON，需金鑰 —— 秘塔 Web Search）
├── client.rs       SearchClient + SearchOptions
├── extractor.rs    完整頁面本文擷取器（用於 --fetch）
└── result.rs       SearchResult / SearchItem / SearchMode
```

## 兩條執行路徑，一種結果型別

所有路徑都匯聚到
[`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs)：

```
query + Engine ─► SearchClient ─► engines_impl/* ─► 統一 ─► SearchResult
```

- **API** —— `engines_impl::<engine>::search(&http, query, &opts)` 呼叫服務商，把 JSON
  反序列化成 `SearchItem`。
- **擷取** —— 簽章相同，但解析的是 HTML 結果頁。

`SearchMode`（`Api` / `Scrape`）記錄結果由哪條路徑產生，呼叫端據此可區分結構化 API 答案
與擷取頁面。

## 分派

`SearchClient::search_with_options` 是對 `Engine` 的扁平 `match`。新增後端意味著：在
`engines_impl/` 實作一個函式，新增一個 `Engine` 變體，新增一個 `match` 分支。沒有
trait object 或動態分派 —— 引擎集合是封閉、編譯期已知的，這讓 API 可預測、二進位檔更小。

## 無頭瀏覽器

seia 刻意**不**附帶任何瀏覽器自動化。每個後端都是普通的 HTTP 用戶端。會激進攔截非瀏覽器
流量的引擎（Google、Baidu、Yandex 網頁搜尋）不在範圍內 —— 請透過其官方 API，或當
[shirabe](https://github.com/celestia-island/shirabe) 未來以獨立 MCP 形式提供時，使用這類
專用瀏覽器工具來存取。

## 本文補全

`SearchOptions::fetch_content` 是一個正交關注點：引擎回傳 `SearchItem` 之後，
`extractor::fetch_content` 下載並清洗每個頁面。它與引擎無關。
