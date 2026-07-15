<p align="center"><img src="https://raw.githubusercontent.com/celestia-island/docs.celestia.world/dev/res/logo/seia.webp" alt="Seia" width="240" /></p>

<h1 align="center">Seia</h1>

<p align="center"><strong>멀티 엔진 웹 검색</strong></p>

<div align="center">

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](https://sysl.celestia.world)
[![GitHub](https://img.shields.io/badge/github-celestia--island%2Fseia-blue.svg)](https://github.com/celestia-island/seia)
[![Checks](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)
[![docs.rs](https://docs.rs/seia/badge.svg)](https://docs.rs/seia)

</div>

<div align="center">

[English](../en/README.md) ·
[简体中文](../zhs/README.md) ·
[繁體中文](../zht/README.md) ·
[日本語](../ja/README.md) ·
**한국어** ·
[Français](../fr/README.md) ·
[Español](../es/README.md) ·
[Русский](../ru/README.md) ·
[العربية](../ar/README.md)

</div>

## 소개

seia는 다중 엔진 웹 검색 라이브러리이자 CLI 도구입니다. 다양한 검색 백엔드를
하나의 인터페이스로 사용할 수 있으며, 인증이 필요 없는 엔진은 별도 설정 없이 바로 사용할 수
있습니다.

## 빠른 시작

### CLI

```bash
# Basic search (no API key required)
seia search "rust async patterns"

# Choose a specific engine
seia search "Klein bottle" --engine wikipedia

# JSON output
seia search "climate change" --json

# Through a proxy
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

### 라이브러리

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Wikipedia).await?;
```

## MCP 서버

`mcp` feature로 seia를 빌드하고 stdio 서버를 실행합니다——모델 컨텍스트 프로토콜(Model Context Protocol)을 통해 다중 엔진 검색 클라이언트를 AI 코딩 어시스턴트에 노출합니다:

```bash
seia mcp
```

서버는 세 가지 도구를 제공합니다: `seia_search`(단일 엔진, 기본값 duckduckgo, 키 불필요), `seia_search_multi`(엔진 체인 시도, 첫 번째 결과 반환), `seia_list_engines`(9개 엔진 및 API 키 환경 변수 목록). MCP 클라이언트에 연결하려면:

```json
{
  "mcpServers": {
    "seia": { "command": "seia", "args": ["mcp"] }
  }
}
```

`SEIA_PROXY`를 설정하여 프록시를 통해 검색 요청을 라우팅합니다(예: `http://localhost:7890`); `HTTPS_PROXY` / `HTTP_PROXY`도 지원됩니다.

## 개발

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
just test-proxy  # run tests through localhost:7890 proxy (see tests/README)
```

## 지원 검색 엔진

| 엔진 | 인증 |
|------|------|
| [DuckDuckGo](https://duckduckgo.com/) | 없음 |
| [Wikipedia](https://www.mediawiki.org/wiki/API:Search) | 없음 |
| [SearXNG](https://docs.searxng.org/) | `SEARXNG_URL` |
| [Tavily](https://docs.tavily.com/) | `TAVILY_API_KEY` |
| [Bing](https://learn.microsoft.com/en-us/bing/search-apis/bing-web-search/) | `BING_SEARCH_API_KEY` |
| [Brave](https://api.search.brave.com/app/documentation) | `BRAVE_SEARCH_API_KEY` |
| [秘塔 (MetaSo)](https://metaso.cn/search-api/playground) | `METASO_API_KEY` |
| [智谱 (Zhipu)](https://docs.bigmodel.cn/cn/guide/tools/web-search) | `ZHIPU_API_KEY` |
| [博查 (Bocha)](https://open.bochaai.com/docs) | `BOCHA_API_KEY` |

## 라이선스

SySL-1.0 (Synthetic Source License). 자세한 내용은 [LICENSE](https://sysl.celestia.world)를 참조하세요.
