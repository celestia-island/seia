# アーキテクチャ

seia はライブラリ（`src/lib.rs`）と CLI（`src/main.rs`）を兼ね備えた単一の crate です。
設計目標は**ひとつのクエリインターフェース、複数のバックエンド**です。呼び出し側は
`Engine` を選ぶだけで、結果の取得方法によらず同じ `SearchResult` を得られます。

## モジュール構成

```
src/
├── lib.rs          公開 API 群
├── main.rs         clap CLI（search / engines）
├── engines.rs      Engine 列挙型: as_str、api_key_env、needs_key
├── engines_impl/   バックエンドごとに 1 モジュール
│   ├── duckduckgo.rs   スクレイプ（HTML）
│   ├── wikipedia.rs    API（JSON）
│   ├── tavily.rs       API（JSON、キー必要）
│   ├── searxng.rs      API（JSON、自己ホスト）
│   ├── bing.rs         API（JSON、キー必要）
│   ├── brave.rs        API（JSON、キー必要）
│   ├── zhipu.rs        API（JSON、キー必要 —— 智谱 Web Search）
│   ├── bocha.rs        API（JSON、キー必要 —— 博查 Web Search）
│   └── metaso.rs       API（JSON、キー必要 —— 秘塔 Web Search）
├── client.rs       SearchClient + SearchOptions
├── extractor.rs    ページ全文取得（--fetch 用）
└── result.rs       SearchResult / SearchItem / SearchMode
```

## 2 つの実行パス、1 つの結果型

すべてのパスは [`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs)
に集約されます。

```
query + Engine ─► SearchClient ─► engines_impl/* ─► 統一 ─► SearchResult
```

- **API** —— `engines_impl::<engine>::search(&http, query, &opts)` がプロバイダを呼び出し、
  JSON を `SearchItem` にデシリアライズします。
- **スクレイプ** —— 同じシグネチャですが、HTML の結果ページを解析します。

`SearchMode`（`Api` / `Scrape`）はどのパスが結果を生成したかを記録するため、呼び出し側は
構造化された API の応答とスクレイプしたページを区別できます。

## ディスパッチ

`SearchClient::search_with_options` は `Engine` に対するフラットな `match` です。
バックエンドを追加するには、`engines_impl/` に 1 つの関数を実装し、`Engine` のバリアントを
追加し、`match` の腕を 1 つ増やします。トレイトオブジェクトや動的ディスパッチはありません
—— エンジンの集合は閉じておりコンパイル時に確定しているため、API は予測可能でバイナリも
小さくなります。

## ヘッドレスブラウザなし

seia は意図的にブラウザ自動化を**一切**バンドルしていません。すべてのバックエンドは純粋な
HTTP クライアントです。非ブラウザのトラフィックを強力にブロックするエンジン（Google、Baidu、
Yandex の Web 検索）はスコープ外であり、公式 API を使うか、スタンドアロンの MCP として利用可能に
なった際に [shirabe](https://github.com/celestia-island/shirabe) のような専用ブラウザツールを
経由してください。

## 本文の補完

`SearchOptions::fetch_content` は直交する関心事です。エンジンが `SearchItem` を返した後、
`extractor::fetch_content` が各ページをダウンロードして整形します。これはエンジンに依存しません。
