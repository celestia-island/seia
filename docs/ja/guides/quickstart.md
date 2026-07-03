# クイックスタート

## インストール

```bash
# crates.io から（公開後）
cargo install seia

# ソースから
cargo install --path .
```

## 最初の検索（CLI）

デフォルトのエンジンは DuckDuckGo です —— 無料、キー不要、すぐに使えます。

```bash
seia search "rust async patterns"

# 別のエンジンを選ぶ
seia search "Klein bottle" --engine wikipedia

# 機械可読な出力
seia search "climate change" --json

# 各結果のページ全文を取得（遅くなります）
seia search "tokio runtime" --fetch
```

`seia engines` を実行すると、すべてのエンジンとキーが必要かどうかを一覧表示できます。

## キーが必要なエンジン

シェルでキーをエクスポートしてください。seia が自動的に読み取ります。

```bash
export TAVILY_API_KEY=tvly-xxxxx
seia search "react server components" --engine tavily

export SEARXNG_URL=http://localhost:8080
seia search "open source licenses" --engine searxng
```

## プロキシ経由

`SearchClient` は reqwest を経由して標準の `HTTPS_PROXY` / `HTTP_PROXY` 環境変数を
尊重します。フラグは不要です:

```bash
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

## ライブラリとして使う

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

エンジンの一覧は [エンジン](./engines.md) を、プログラム API については
[ライブラリの使い方](./library.md) を参照してください。
