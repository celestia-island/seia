# 架构

seia 是单一 crate，同时提供库（`src/lib.rs`）与 CLI（`src/main.rs`）。设计目标是
**一个查询入口，多个后端**：调用方选择一个 `Engine`，无论结果由哪个后端产生，都拿到
同一个 `SearchResult`。

## 模块地图

```
src/
├── lib.rs          公开 API 面
├── main.rs         clap CLI（search / engines）
├── engines.rs      Engine 枚举：as_str、api_key_env、needs_key
├── engines_impl/   每个后端一个模块
│   ├── duckduckgo.rs   爬取（HTML）
│   ├── wikipedia.rs    API（JSON）
│   ├── tavily.rs       API（JSON，需密钥）
│   ├── searxng.rs      API（JSON，自建）
│   ├── bing.rs         API（JSON，需密钥）
│   ├── brave.rs        API（JSON，需密钥）
│   ├── zhipu.rs        API（JSON，需密钥 —— 智谱 Web Search）
│   ├── bocha.rs        API（JSON，需密钥 —— 博查 Web Search）
│   └── metaso.rs       API（JSON，需密钥 —— 秘塔 Web Search）
├── client.rs       SearchClient + SearchOptions
├── extractor.rs    完整页面正文抓取器（用于 --fetch）
└── result.rs       SearchResult / SearchItem / SearchMode
```

## 两条执行路径，一种结果类型

所有路径都汇聚到
[`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs)：

```
query + Engine ─► SearchClient ─► engines_impl/* ─► 统一 ─► SearchResult
```

- **API** —— `engines_impl::<engine>::search(&http, query, &opts)` 调用服务商，把 JSON
  反序列化成 `SearchItem`。
- **爬取** —— 签名相同，但解析的是 HTML 结果页。

`SearchMode`（`Api` / `Scrape`）记录结果由哪条路径产生，调用方据此可区分结构化 API
答案与抓取页面。

## 分派

`SearchClient::search_with_options` 是对 `Engine` 的扁平 `match`。新增后端意味着：在
`engines_impl/` 实现一个函数，新增一个 `Engine` 变体，新增一个 `match` 分支。没有
trait object 或动态分派 —— 引擎集合是封闭、编译期已知的，这让 API 可预测、二进制更小。

## 无头浏览器

seia 刻意**不**内置任何浏览器自动化。每个后端都是纯粹的 HTTP 客户端。会激进屏蔽非浏览
器流量的引擎（Google、百度、Yandex 网页搜索）不在范围内 —— 请经由它们的官方 API，
或在专用的浏览器工具（如 [shirabe](https://github.com/celestia-island/shirabe)）以独立
MCP 形式可用时，通过它来访问。

## 正文补全

`SearchOptions::fetch_content` 是一个正交关注点：引擎返回 `SearchItem` 之后，
`extractor::fetch_content` 下载并清洗每个页面。它与引擎无关。
