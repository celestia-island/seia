# seia — 通用搜索引擎抽象层

**一次查询，所有搜索引擎。**

Rust 多后端 Web 搜索库与 CLI。免费引擎开箱即用。

## 简介

seia 提供统一的接口访问多个搜索后端 —— DuckDuckGo、Tavily、Wikipedia、SearXNG、
Bing、Brave、Google、Baidu 等。免费引擎零配置即可使用。

## 快速开始

### CLI

```bash
# 基础搜索（DuckDuckGo，免费、无需密钥）
seia search "rust 异步模式"

# Wikipedia（免费、学术）
seia search "克莱因瓶" --engine wikipedia

# JSON 输出
seia search "气候变化" --json

# 通过代理
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# 浏览器模式（Google/Baidu 经由 tairitsu）
seia search "查询" --engine google --browser
```

### 作为库使用

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust 异步", Engine::DuckDuckGo).await?;
```

## 引擎

| 引擎 | 模式 | 认证 | 状态 |
|------|------|------|------|
| DuckDuckGo | 爬取 | 无 | ✅ |
| Wikipedia | API | 无 | ✅ |
| SearXNG | API | `SEARXNG_URL` | ✅ |
| Tavily | API | `TAVILY_API_KEY` | ✅ |
| Bing | API | `BING_SEARCH_API_KEY` | 🔲 |
| Brave | API | `BRAVE_SEARCH_API_KEY` | 🔲 |
| Google | 浏览器 | tairitsu | ✅ |
| Baidu | 浏览器 | tairitsu | ✅ |
| Bing Web | 浏览器 | tairitsu | ✅ |
| Yandex | 浏览器 | tairitsu | ✅ |

浏览器模式引擎使用 [tairitsu](https://github.com/celestia-island/tairitsu)
进行无头渲染。可以运行独立守护进程，或启用 `embedded-browser` feature 将 tairitsu
编译进进程。

## 开发

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## 许可证

SySL-1.0（合成源码许可证）。详见 [LICENSE](../../LICENSE)。
