# 引擎

seia 通过单一的 [`Engine`](https://github.com/celestia-island/seia/blob/dev/src/engines.rs)
枚举暴露所有后端，因此切换后端永远不会影响你的查询代码。

## 三种执行模式

| 模式 | 工作方式 | 引擎 |
| --- | --- | --- |
| **API** | 调用搜索服务商的 HTTP API 并解析 JSON。 | Tavily、SearXNG、Wikipedia |
| **爬取** | 抓取 HTML 结果页并提取命中。 | DuckDuckGo |
| **浏览器** | 驱动无头浏览器（经由 [tairitsu](https://github.com/celestia-island/tairitsu)）渲染 JS 密集型页面。 | Google、Baidu、Bing（网页）、Yandex |

API 与爬取模式只需要一个 HTTP 客户端。浏览器模式见
[浏览器模式](./browser-mode.md)。

## 引擎矩阵

| 引擎 | 枚举值 | 模式 | 认证 | 免费额度 |
| --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | 爬取 | 无 | 无限 |
| Wikipedia | `Wikipedia` | API | 无 | 无限 |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | 自建 |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1000 / 月 |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1000 / 月 |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2000 / 月 |
| Google | 浏览器 profile | 浏览器 | tairitsu | — |
| Baidu | 浏览器 profile | 浏览器 | tairitsu | — |
| Bing（网页） | 浏览器 profile | 浏览器 | tairitsu | — |
| Yandex | 浏览器 profile | 浏览器 | tairitsu | — |

> Bing 与 Brave 的 API 后端目前是占位实现（`Engine::Bing` / `Engine::Brave`
> 会返回 "not yet implemented" 错误）。请使用浏览器 profile，或
> [贡献](https://github.com/celestia-island/seia) 实现。

## 选择引擎

CLI：

```bash
seia search "查询" --engine wikipedia
```

库：

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("查询", Engine::Wikipedia).await?;
```

## 查询引擎元信息

`Engine` 自带元信息，因此无需硬编码即可构建 UI：

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing
    println!("  需要密钥? {}", engine.needs_key());
    println!("  密钥环境变量: {:?}", engine.api_key_env());
}
```
