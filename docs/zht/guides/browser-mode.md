# 瀏覽器模式

部分引擎 —— Google、Baidu、Bing（網頁，非 API）、Yandex —— 會激進地攔截非瀏覽器
請求。seia 透過 [tairitsu](https://github.com/celestia-island/tairitsu) 這個無頭瀏覽器
執行環境來驅動它們。seia 說的就是 tairitsu 的 HTTP debug API，因此**沒有**原生瀏覽器
綁定。

## 兩種執行 tairitsu 的方式

### 1. 外部常駐程式（預設）

在應用之外執行一個 tairitsu debug 伺服器，然後讓 seia 指向它：

```bash
# 一個終端機
tairitsu debug --proxy http://localhost:7890

# 另一個終端機
seia search "rust 非同步" --engine google --browser --tairitsu http://127.0.0.1:3001
```

這樣可以把沉重的瀏覽器行程擋在你的應用二進位檔之外。

### 2. 內嵌（`embedded-browser` feature）

把 tairitsu 的 debug 伺服器**編譯進** seia，無需單獨常駐程式：

```toml
[dependencies]
seia = { version = "0.1", features = ["embedded-browser"] }
```

```bash
seia search "rust 非同步" --engine google --browser --embedded
```

`embedded` 旗標在行程內啟動伺服器（見
[`seia::embedded::start`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs)）。

## 瀏覽器搜尋的工作流程

每次瀏覽器搜尋分三步，全部發往 tairitsu 的 HTTP API：

1. **導覽** —— `POST /navigate` 到引擎的搜尋 URL。
2. **等待** —— `POST /wait-for-selector` 直到結果容器渲染出來。
3. **擷取** —— `POST /evaluate` 執行一段 JS，從 DOM 中讀取標題、連結與摘要。

每個引擎的選擇器和 URL 模板定義在
[`SearchProfile`](https://github.com/celestia-island/seia/blob/dev/src/profiles.rs) 中：

| Profile | 搜尋 URL | 結果容器 |
| --- | --- | --- |
| `google` | `google.com/search?q=` | `div.g` |
| `baidu` | `baidu.com/s?wd=` | `div.result, div.c-container` |
| `bing_web` | `bing.com/search?q=` | `li.b_algo` |
| `yandex` | `yandex.com/search/?text=` | `li.serps-item, div.Organic` |

## 直接使用瀏覽器用戶端

```rust
use seia::{BrowserClient, profiles};

let client = BrowserClient::new("http://127.0.0.1:3001");

if !client.health().await.unwrap_or(false) {
    panic!("tairitsu 瀏覽器未連線");
}

let profile = profiles::get_profile("google").unwrap();
let result = client.search("rust 非同步", profile).await?;
```

CLI 內部把 `--engine <名稱> --browser` 對應到對應的 profile（無符合項時退回
`google` profile）。
