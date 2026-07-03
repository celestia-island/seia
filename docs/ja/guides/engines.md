# エンジン

seia は 9 つのバックエンドをサポートしています。すべて公式の HTTP API（API が存在しない場合は
軽量な HTML スクレイプ）経由でアクセスします。ヘッドレスブラウザは一切存在せず、seia は
純粋な HTTP クライアントであるため、どのエンジンも CLI とライブラリの双方で同じ `Engine`
列挙型を通じて動作します。

ほとんどのエンジンは無料枠を提供しており、キーが必要なものはドキュメントに記載された環境変数から
読み取るため、コードや CLI 引数にキーが現れることはありません。

## 2 つの実行モード

| モード | 仕組み | エンジン |
| --- | --- | --- |
| **API** | 検索プロバイダの HTTP API を呼び出して JSON を解析します。 | Tavily、SearXNG、Wikipedia、Bing、Brave、智谱、博查、秘塔 |
| **スクレイプ** | HTML の検索結果ページを取得し、ヒットを抽出します。 | DuckDuckGo |

## エンジンマトリクス

### 国際

| エンジン | 列挙値 | モード | 認証 | 無料枠 | 状態 |
| --- | --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | スクレイプ | なし | 無制限 | ✅ |
| Wikipedia | `Wikipedia` | API | なし | 無制限 | ✅ |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | 自己ホスト | ✅ |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1 000 / 月 | ✅ |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1 000 / 月 | ✅ |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2 000 / 月 | ✅ |

### 国内（中国）

| エンジン | 列挙値 | モード | 認証 | 状態 |
| --- | --- | --- | --- | --- |
| 智谱 (Zhipu / BigModel) | `Zhipu` | API | `ZHIPU_API_KEY` | ✅ |
| 博查 (Bocha) | `Bocha` | API | `BOCHA_API_KEY` | ✅ |
| 秘塔 (Metaso) | `Metaso` | API | `METASO_API_KEY` | ✅ |

> 智谱の Web Search API は複数のバッキングエンジンを経由できます —— 智谱基础版
> （`search_std`、デフォルト）、智谱高阶版（`search_pro`）、搜狗（`search_pro_sogou`）、
> 夸克（`search_pro_quark`）。`ZHIPU_SEARCH_ENGINE` 環境変数で選択してください。

> 博查はページごとに短い `snippet` と、より長い LLM 生成の `summary` を返します。seia は
> 長い方を結果の `snippet` として採用します。

> 秘塔の検索スコープは既定で `webpage` です。別のスコープを使うには `METASO_SCOPE`
> 環境変数で上書きしてください。また、応答エンベロプは防御的に解析され、フィールドの欠落や
> 予期しない形状に対して耐性を持ちます。

## エンジンの選択

CLI:

```bash
seia search "query" --engine wikipedia
seia search "查询" --engine zhipu      # ZHIPU_API_KEY が必要
```

ライブラリ:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("query", Engine::Wikipedia).await?;
client.search("查询", Engine::Zhipu).await?;   // ZHIPU_API_KEY が必要
```

## エンジンのメタデータを確認する

`Engine` は自身のメタデータを保持しています。

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing, Engine::Zhipu] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing / zhipu
    println!("  needs key? {}", engine.needs_key());
    println!("  key env:    {:?}", engine.api_key_env());
}
```
