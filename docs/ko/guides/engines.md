# 엔진

seia는 9개의 백엔드를 지원하며, 모두 공식 HTTP API(API가 없는 경우 가벼운 HTML 스크랩)를
통해 접근합니다. 헤드리스 브라우저는 없습니다 — seia는 순수 HTTP 클라이언트이므로, 모든
엔진이 동일한 `Engine` 열거형을 통해 CLI와 라이브러리 양쪽에서 동작합니다.

대부분의 엔진은 무료 한도를 제공하며, 키가 필요한 엔진은 문서화된 환경변수에서 키를
읽습니다. 따라서 키가 코드나 CLI 인자에 노출되는 일은 없습니다.

## 두 가지 실행 모드

| 모드 | 작동 방식 | 사용 엔진 |
| --- | --- | --- |
| **API** | 검색 제공자의 HTTP API를 호출하고 JSON을 파싱합니다. | Tavily, SearXNG, Wikipedia, Bing, Brave, 智谱, 博查, 秘塔 |
| **스크랩** | 가벼운 HTML 결과 페이지를 가져와 검색 결과를 추출합니다. | DuckDuckGo |

## 엔진 매트릭스

### 국제

| 엔진 | 열거형 값 | 모드 | 인증 | 무료 한도 | 상태 |
| --- | --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | 스크랩 | 없음 | 무제한 | ✅ |
| Wikipedia | `Wikipedia` | API | 없음 | 무제한 | ✅ |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | 자체 호스팅 | ✅ |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1 000/월 | ✅ |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1 000/월 | ✅ |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2 000/월 | ✅ |

### 국내 (중국)

| 엔진 | 열거형 값 | 모드 | 인증 | 상태 |
| --- | --- | --- | --- | --- |
| 智谱 (Zhipu / BigModel) | `Zhipu` | API | `ZHIPU_API_KEY` | ✅ |
| 博查 (Bocha) | `Bocha` | API | `BOCHA_API_KEY` | ✅ |
| 秘塔 (Metaso) | `Metaso` | API | `METASO_API_KEY` | ✅ |

> 智谱의 Web Search API는 여러 백킹 엔진 중 하나를 경유하여 라우팅할 수 있습니다 —
> 智谱基础版 (`search_std`, 기본값), 智谱高阶版 (`search_pro`), 搜狗
> (`search_pro_sogou`), 또는 夸克 (`search_pro_quark`). `ZHIPU_SEARCH_ENGINE`
> 환경변수로 선택하세요.

> 博查는 페이지별로 짧은 `snippet`과 더 긴 LLM 생성 `summary`를 모두 반환합니다;
> seia는 둘 중 더 긴 쪽을 결과의 `snippet`으로 노출합니다.

> 秘塔(Metaso)의 검색 범위는 기본적으로 `webpage`입니다. 다른 범위는 `METASO_SCOPE`
> 환경변수로 덮어쓸 수 있으며, 응답 봉투(envelope)는 방어적으로 파싱됩니다 — 필드가
> 누락되거나 형태가 달라도 seia는 사용 가능한 항목을 최대한 복원합니다.

## 엔진 선택

CLI:

```bash
seia search "query" --engine wikipedia
seia search "查询" --engine zhipu      # ZHIPU_API_KEY 필요
```

라이브러리:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("query", Engine::Wikipedia).await?;
client.search("查询", Engine::Zhipu).await?;   // ZHIPU_API_KEY 필요
```

## 엔진 메타데이터 확인

`Engine`은 자체 메타데이터를 가지고 있습니다:

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing, Engine::Zhipu] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing / zhipu
    println!("  키 필요? {}", engine.needs_key());
    println!("  키 환경변수: {:?}", engine.api_key_env());
}
```
