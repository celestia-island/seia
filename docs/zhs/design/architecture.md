# 架构

seia 是单一 crate，同时提供库（`src/lib.rs`）与 CLI（`src/main.rs`）。设计目标是
**一个查询入口，多个后端**：调用方选择一个 `Engine`，无论结果如何取得，都拿到同一个
`SearchResult`。

## 模块地图

```
src/
├── lib.rs          公开 API 面 + embedded-browser 服务
├── main.rs         clap CLI（search / engines）
├── engines.rs      Engine 枚举：as_str、api_key_env、needs_key、needs_browser
├── engines_impl/   每个 API/爬取后端一个模块
│   ├── duckduckgo.rs   爬取（HTML）
│   ├── wikipedia.rs    API（JSON）
│   ├── tavily.rs       API（JSON，需密钥）
│   └── searxng.rs      API（JSON，自建）
├── client.rs       SearchClient + SearchOptions（API/爬取路径）
├── browser.rs      BrowserClient（经 HTTP 与 tairitsu 通信）
├── profiles.rs     SearchProfile：每个引擎的 CSS 选择器 + URL 模板
├── extractor.rs    完整页面正文抓取器（用于 --fetch）
└── result.rs       SearchResult / SearchItem / SearchMode
```

## 三条执行路径，一种结果类型

三条路径都汇聚到
[`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs)：

```
                       ┌─ engines_impl/*（API / 爬取）─┐
query + Engine ─► SearchClient ─► 统一 ─► SearchResult
                       └─ browser.rs（tairitsu HTTP）──┘
```

- **API** —— `engines_impl::<engine>::search(&http, query, &opts)` 调用服务商，把 JSON
  反序列化成 `SearchItem`。
- **爬取** —— 签名相同，但解析的是 HTML 结果页。
- **浏览器** —— `BrowserClient::search` 驱动 tairitsu；每个引擎的 `SearchProfile`
  提供 URL 与注入的提取 JS 所用的 CSS 选择器。

`SearchMode`（`Api` / `Scrape` / `Browser`）记录结果由哪条路径产生，调用方据此可区分，
例如缓存 API 答案与渲染页面。

## 分派

`SearchClient::search_with_options` 是对 `Engine` 的扁平 `match`。新增后端意味着：在
`engines_impl/` 实现一个函数，新增一个 `Engine` 变体，新增一个 `match` 分支。没有
trait object 或动态分派 —— 引擎集合是封闭、编译期已知的，这让 API 可预测、二进制更小。

## 正文补全

`SearchOptions::fetch_content` 是一个正交关注点：引擎返回 `SearchItem` 之后，
`extractor::fetch_content` 下载并清洗每个页面。它与引擎无关，对任意模式都生效。

## 浏览器集成边界

`tairitsu-packager` 是**可选**依赖，由 `embedded-browser` feature 门控。不开启时 seia
不含任何浏览器代码，而是通过普通 HTTP 连接外部 tairitsu 守护进程（`BrowserClient`）。
开启时 `seia::embedded::start` 在进程内启动 debug 服务。这样既保证默认构建轻量，又让
可发布的 crate 不背负沉重的浏览器依赖。
