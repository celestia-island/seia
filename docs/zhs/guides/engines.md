# 引擎

seia 支持 9 个后端，全部经由各自官方的 HTTP API（在没有官方 API 时，则进行轻量 HTML
抓取）。seia 不内置任何无头浏览器 —— 它是一个纯粹的 HTTP 客户端，因此每个引擎在 CLI
与库中都通过同一个 `Engine` 枚举使用。

大多数引擎提供免费额度；需要密钥的引擎会从文档中约定的环境变量读取，所以代码或 CLI
参数里永远不会出现密钥。

## 两种执行模式

| 模式 | 工作方式 | 引擎 |
| --- | --- | --- |
| **API** | 调用搜索服务商的 HTTP API 并解析 JSON。 | Tavily、SearXNG、Wikipedia、Bing、Brave、智谱、博查、秘塔 |
| **爬取** | 抓取轻量 HTML 结果页并提取命中。 | DuckDuckGo |

## 引擎矩阵

### 国际

| 引擎 | 枚举值 | 模式 | 认证 | 免费额度 | 状态 |
| --- | --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | 爬取 | 无 | 无限 | ✅ |
| Wikipedia | `Wikipedia` | API | 无 | 无限 | ✅ |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | 自建 | ✅ |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1 000 / 月 | ✅ |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1 000 / 月 | ✅ |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2 000 / 月 | ✅ |

### 国内（中国）

| 引擎 | 枚举值 | 模式 | 认证 | 状态 |
| --- | --- | --- | --- | --- |
| 智谱（Zhipu / BigModel） | `Zhipu` | API | `ZHIPU_API_KEY` | ✅ |
| 博查（Bocha） | `Bocha` | API | `BOCHA_API_KEY` | ✅ |
| 秘塔（Metaso） | `Metaso` | API | `METASO_API_KEY` | ✅ |

> 智谱的 Web Search API 可经由若干底层引擎之一路由 —— 智谱基础版
> （`search_std`，默认）、智谱高阶版（`search_pro`）、搜狗
> （`search_pro_sogou`）或夸克（`search_pro_quark`）。用 `ZHIPU_SEARCH_ENGINE`
> 环境变量选择其一。

> 博查对每个页面会同时返回简短的 `snippet` 和较长的、由 LLM 生成的 `summary`；seia
> 取其中较长者作为结果的 `snippet`。

> 秘塔的搜索范围默认为 `webpage`；可通过 `METASO_SCOPE` 环境变量覆盖。其响应信封会以
> 防御性方式解析 —— 缺失或格式异常的字段不会导致整体失败。

## 选择引擎

CLI：

```bash
seia search "查询" --engine wikipedia
seia search "查询" --engine zhipu      # 需要 ZHIPU_API_KEY
```

库：

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("查询", Engine::Wikipedia).await?;
client.search("查询", Engine::Zhipu).await?;   // 需要 ZHIPU_API_KEY
```

## 查询引擎元信息

`Engine` 自带元信息：

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing, Engine::Zhipu] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing / zhipu
    println!("  需要密钥? {}", engine.needs_key());
    println!("  密钥环境变量: {:?}", engine.api_key_env());
}
```
