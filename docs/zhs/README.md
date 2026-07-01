<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>阅览互联网上的知识</strong>

多引擎 Web 搜索库与 CLI，Rust 实现。免费引擎开箱即用

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](../../LICENSE)
[![Crates.io](https://img.shields.io/crates/v/seia)](https://docs.rs/seia)
[![CI](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)

[English](../en/README.md) · **简体中文** ·
[繁體中文](../zht/README.md) · [日本語](../ja/README.md) ·
[한국어](../ko/README.md) · [Français](../fr/README.md) ·
[Español](../es/README.md) · [Русский](../ru/README.md) ·
[العربية](../ar/README.md)

</div>

## 简介

seia 提供统一的接口访问多个搜索后端 —— DuckDuckGo、Tavily、Wikipedia、SearXNG、
Bing、Brave、Google、Baidu 等。免费引擎零配置即可使用。

## 快速开始

### CLI

```bash
# 基础搜索（DuckDuckGo，免费、无需密钥）
seia search "rust 异步模式"

# Wikipedia（免费 API，学术知识）
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

### API / 爬取（`Engine` 枚举 — CLI + 库均可）

| 引擎 | 模式 | 认证 | 免费额度 | 状态 |
|------|------|------|---------|------|
| DuckDuckGo | 爬取 | 无 | 无限 | ✅ |
| Wikipedia | API | 无 | 无限 | ✅ |
| SearXNG | API | `SEARXNG_URL` | 自建 | ✅ |
| Tavily | API | `TAVILY_API_KEY` | 1 000 / 月 | ✅ |
| Bing | API | `BING_SEARCH_API_KEY` | 1 000 / 月 | 🔜 |
| Brave | API | `BRAVE_SEARCH_API_KEY` | 2 000 / 月 | 🔜 |

> Bing 和 Brave 的 API 后端目前是占位实现（返回"尚未实现"）。可以先用对应浏览
> 器 profile 临时替代，或[贡献实现](https://github.com/celestia-island/seia)。

### 浏览器（仅 CLI — `seia search --browser --engine <名称>`）

| Profile | 认证 | 说明 |
|---------|------|------|
| google | 无（通过 tairitsu 爬取） | Google 网页搜索。也有[付费 CSE API](https://developers.google.com/custom-search) |
| baidu | 无（通过 tairitsu 爬取） | 百度网页搜索 |
| bing_web | 无（通过 tairitsu 爬取） | Bing 网页结果。也有[付费 Search API](https://www.microsoft.com/en-us/bing/apis/bing-web-search-api) |
| yandex | 无（通过 tairitsu 爬取） | Yandex 网页搜索 |

浏览器模式引擎使用 [tairitsu](https://github.com/celestia-island/tairitsu)
进行无头渲染。可以运行独立守护进程，或启用 `embedded-browser` feature 将 tairitsu
编译进进程。

> 大多数搜索引擎都提供官方 REST API（Google CSE、Bing Search API、Brave Search
> API 等）。浏览器 profile 是 API 后端尚未实现时的临时方案，或者是 API 并非免费
> 时的替代路径。长期计划是为每个浏览器 profile 增加对应的 `Engine` 变体并提供
> API-key 支持。

## 开发

```bash
just ci          # 格式化检查 + clippy + 测试
just test        # 运行测试
just test-proxy  # 通过 localhost:7890 代理运行测试（见 tests/README）
```

## 许可证

SySL-1.0（合成源码许可证）。详见 [LICENSE](../../LICENSE)。
