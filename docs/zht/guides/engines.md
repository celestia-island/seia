# 引擎

seia 支援 9 個後端，全部透過其官方 HTTP API（沒有 API 時則為輕量 HTML 爬取）存取。seia 不含無頭瀏覽器 —— 是純 HTTP 用戶端，因此每個引擎在 CLI 與函式庫中都透過同一個 [`Engine`](https://github.com/celestia-island/seia/blob/dev/src/engines.rs) 列舉運作。

多數引擎提供免費額度；需要金鑰的引擎會從文件記載的環境變數讀取，因此金鑰永遠不會出現在程式碼或 CLI 引數中。

## 兩種執行模式

| 模式 | 運作方式 | 引擎 |
| --- | --- | --- |
| **API** | 呼叫搜尋服務商的 HTTP API 並解析 JSON。 | Tavily、SearXNG、Wikipedia、Bing、Brave、智譜、博查、秘塔 |
| **擷取** | 抓取輕量 HTML 結果頁並擷取命中。 | DuckDuckGo |

## 引擎矩陣

### 國際

| 引擎 | 列舉值 | 模式 | 認證 | 免費額度 | 狀態 |
| --- | --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | 擷取 | 無 | 無限 | ✅ |
| Wikipedia | `Wikipedia` | API | 無 | 無限 | ✅ |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | 自建 | ✅ |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1 000 / 月 | ✅ |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1 000 / 月 | ✅ |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2 000 / 月 | ✅ |

### 國內（中國）

| 引擎 | 列舉值 | 模式 | 認證 | 狀態 |
| --- | --- | --- | --- | --- |
| 智譜 (Zhipu / BigModel) | `Zhipu` | API | `ZHIPU_API_KEY` | ✅ |
| 博查 (Bocha) | `Bocha` | API | `BOCHA_API_KEY` | ✅ |
| 秘塔 (Metaso) | `Metaso` | API | `METASO_API_KEY` | ✅ |

> 智譜的 Web Search API 可路由到多個後端引擎之一 —— 智譜基礎版（`search_std`，預設）、
> 智譜高階版（`search_pro`）、搜狗（`search_pro_sogou`）或夸克（`search_pro_quark`）。
> 透過 `ZHIPU_SEARCH_ENGINE` 環境變數選擇。

> 博查對每個頁面同時回傳簡短的 `snippet` 與較長的 LLM 生成 `summary`；seia 會取兩者中
> 較長者作為結果的 `snippet`。

> 秘塔的搜尋範圍預設為 `webpage`，可透過 `METASO_SCOPE` 環境變數覆寫為其他範圍。
> 回應封包（envelope）以寬鬆方式解析，即使部分欄位缺失也能盡可能回傳結果。

## 選擇引擎

CLI：

```bash
seia search "查詢" --engine wikipedia
seia search "查詢" --engine zhipu      # 需要 ZHIPU_API_KEY
```

函式庫：

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("查詢", Engine::Wikipedia).await?;
client.search("查詢", Engine::Zhipu).await?;   // 需要 ZHIPU_API_KEY
```

## 查詢引擎元資料

`Engine` 自帶元資料：

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing, Engine::Zhipu] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing / zhipu
    println!("  需要金鑰? {}", engine.needs_key());
    println!("  金鑰環境變數: {:?}", engine.api_key_env());
}
```
