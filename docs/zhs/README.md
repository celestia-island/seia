# seia — 通用搜索引擎抽象层

<div align="center">

**一次查询，所有搜索引擎。**

Rust 多后端 Web 搜索库与 CLI。免费引擎开箱即用。

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](../../LICENSE)
[![CI](https://github.com/celestia-island/seia/actions/workflows/checks.yml/badge.svg)](https://github.com/celestia-island/seia/actions)

**[English](../en/README.md)** · **[简体中文](README.md)**

</div>

## 简介

seia 提供统一的接口访问多个搜索后端 —— 从免费的 HTML 爬取到付费 API，再到通过
[tairitsu](https://github.com/celestia-island/tairitsu) 驱动的无头浏览器。切换引擎时
无需改动查询代码。

## 快速开始

```bash
# 默认引擎 DuckDuckGo，免费、无需密钥
seia search "rust 异步模式"

# 选择引擎
seia search "克莱因瓶" --engine wikipedia

# JSON 输出
seia search "气候变化" --json

# 拉取每条结果的完整页面文本（较慢）
seia search "tokio 运行时" --fetch
```

运行 `seia engines` 查看全部引擎及是否需要密钥。

## 作为库使用

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client.search("rust 异步", Engine::Duckduckgo).await?;

for item in &result.items {
    println!("{} — {}", item.title, item.url);
}
```

更多见 [快速开始](./guides/quickstart.md)、[引擎](./guides/engines.md) 与
[架构](./design/architecture.md)。

## 许可证

SySL-1.0（合成源码许可证）。详见 [LICENSE](../../LICENSE)。
