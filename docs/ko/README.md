<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>모든 출처의 지식을 탐색하다</strong>

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](../../LICENSE)
[![Crates.io](https://img.shields.io/crates/v/seia)](https://docs.rs/seia)
[![CI](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)

[English](../en/README.md) · [简体中文](../zhs/README.md) ·
[繁體中文](../zht/README.md) · [日本語](../ja/README.md) ·
**한국어** · [Français](../fr/README.md) ·
[Español](../es/README.md) · [Русский](../ru/README.md) ·
[العربية](../ar/README.md)

</div>

## 소개

seia는 다중 엔진 웹 검색 라이브러리이자 CLI 도구입니다. 다양한 검색 백엔드를
하나의 인터페이스로 사용할 수 있으며, 인증이 필요 없는 엔진은 별도 설정 없이 바로 사용할 수
있습니다.

## 빠른 시작

### CLI

```bash
# 기본 검색 (API 키 불필요)
seia search "rust async patterns"

# 특정 엔진 선택
seia search "Klein bottle" --engine wikipedia

# JSON 출력
seia search "climate change" --json

# 프록시를 통한 검색
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

### 라이브러리

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Wikipedia).await?;
```

## 개발

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## 지원 검색 엔진

| 엔진 | 인증 | 상태 |
|------|------|------|
| [Wikipedia](https://www.mediawiki.org/wiki/API:Search) | 없음 | ✅ |
| [SearXNG](https://docs.searxng.org/) | `SEARXNG_URL` | ✅ |
| [Tavily](https://docs.tavily.com/) | `TAVILY_API_KEY` | ✅ |
| [Bing](https://learn.microsoft.com/en-us/bing/search-apis/bing-web-search/) | `BING_SEARCH_API_KEY` | ✅ |
| [Brave](https://api.search.brave.com/app/documentation) | `BRAVE_SEARCH_API_KEY` | ✅ |
| [智谱 (Zhipu)](https://docs.bigmodel.cn/cn/guide/tools/web-search) | `ZHIPU_API_KEY` | ✅ |
| [博查 (Bocha)](https://open.bochaai.com/docs) | `BOCHA_API_KEY` | ✅ |

## 라이선스

SySL-1.0 (Synthetic Source License). 자세한 내용은 [LICENSE](../../LICENSE)를 참조하세요.
