<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>모든 출처의 지식을 탐색하다</strong>

다중 엔진 웹 검색 라이브러리이자 CLI 도구입니다.

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

# 브라우저 모드 (헤드리스, API 키 불필요)
seia search "query" --browser --browser-engine google
```

### 라이브러리

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Duckduckgo).await?;
```

## 개발

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## 지원 검색 엔진

### API / 스크랩 엔진

| 엔진 | 공식 사이트 | 모드 | 인증 | 무료 한도 | 상태 |
|------|---------|------|------|---------|------|
| DuckDuckGo | [duckduckgo.com](https://duckduckgo.com) | 스크랩 | 없음 | 무제한 | ✅ |
| Wikipedia | [wikipedia.org](https://www.wikipedia.org) | API | 없음 | 무제한 | ✅ |
| SearXNG | [searxng.org](https://searxng.org) | API | `SEARXNG_URL` | 자체 호스팅 | ✅ |
| Tavily | [tavily.com](https://tavily.com) | API | `TAVILY_API_KEY` | 1 000/월 | ✅ |
| Bing | [bing.com](https://www.bing.com) | API | `BING_SEARCH_API_KEY` | 1 000/월 | 🔜 |
| Brave | [brave.com/search](https://brave.com/search) | API | `BRAVE_SEARCH_API_KEY` | 2 000/월 | 🔜 |

> Bing과 Brave API 백엔드는 스텁(미구현) 상태입니다. 브라우저 프로필로 임시 대체하거나
> [기여](https://github.com/celestia-island/seia)해 주세요.

### 브라우저 엔진 (CLI 전용)

| 엔진 | 공식 사이트 | 인증 | 설명 |
|------|---------|------|------|
| Google | [google.com](https://www.google.com) | 없음 (tairitsu로 스크랩) | Google 웹 검색 |
| Baidu | [baidu.com](https://www.baidu.com) | 없음 (tairitsu로 스크랩) | Baidu 웹 검색 |
| Bing Web | [bing.com](https://www.bing.com) | 없음 (tairitsu로 스크랩) | Bing 웹 결과 |
| Yandex | [yandex.com](https://yandex.com) | 없음 (tairitsu로 스크랩) | Yandex 웹 검색 |

브라우저 모드 엔진은 헤드리스 렌더링을 위해
[tairitsu](https://github.com/celestia-island/tairitsu)를 사용합니다. 독립 데몬을
실행하거나 `embedded-browser` 기능을 활성화하여 tairitsu를 프로세스 내에 컴파일할 수
있습니다.

> 대부분의 검색 엔진은 공식 REST API를 제공합니다. 브라우저 프로필은 API 백엔드가 아직
> 구현되지 않았거나 API를 무료로 사용할 수 없는 경우의 대체 수단입니다. 장기적으로는 각
> 브라우저 프로필에 대응하는 `Engine` 배리언트를 추가하고 API 키 지원을 제공할 예정입니다.

## 라이선스

SySL-1.0 (Synthetic Source License). 자세한 내용은 [LICENSE](../../LICENSE)를 참조하세요.
