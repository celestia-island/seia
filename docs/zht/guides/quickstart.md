# 快速開始

## 安裝

```bash
# 從 crates.io（發布後）
cargo install seia

# 從原始碼
cargo install --path .
```

## 第一次搜尋（CLI）

預設引擎是 DuckDuckGo —— 免費、無需金鑰、立即可用：

```bash
seia search "rust 非同步模式"

# 選擇其它引擎
seia search "克萊因瓶" --engine wikipedia

# 機器可讀輸出
seia search "氣候變遷" --json

# 擷取每條結果的完整頁面本文（較慢）
seia search "tokio 執行階段" --fetch
```

執行 `seia engines` 列出全部引擎及是否需要金鑰。

## 需要金鑰的引擎

在 shell 中匯出金鑰，seia 會自動讀取：

```bash
export TAVILY_API_KEY=tvly-xxxxx
seia search "react server components" --engine tavily

export SEARXNG_URL=http://localhost:8080
seia search "開源授權" --engine searxng
```

## 透過代理

```bash
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# 或明確指定
seia search "hello world" --proxy http://localhost:7890
```

## 作為函式庫使用

```rust
use seia::{SearchClient, Engine};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = SearchClient::new();
    let result = client.search("rust 非同步", Engine::Duckduckgo).await?;

    for item in &result.items {
        println!("{} — {}", item.title, item.url);
    }
    Ok(())
}
```

繼續閱讀 [引擎](./engines.md) 查看完整引擎矩陣，或
[函式庫的用法](./library.md) 查看程式介面。
