# 아키텍처

seia는 라이브러리(`src/lib.rs`)와 CLI(`src/main.rs`)를 모두 제공하는 단일 crate입니다.
설계 목표는 **하나의 쿼리 인터페이스, 여러 백엔드**입니다: 호출자가 `Engine`을 선택하면
결과를 어떻게 얻었는지와 관계없이 동일한 `SearchResult`를 받습니다.

## 모듈 맵

```
src/
├── lib.rs          공개 API 영역
├── main.rs         clap CLI (search / engines)
├── engines.rs      Engine 열거형: as_str, api_key_env, needs_key
├── engines_impl/   각 백엔드당 한 모듈
│   ├── duckduckgo.rs   스크랩 (HTML)
│   ├── wikipedia.rs    API (JSON)
│   ├── tavily.rs       API (JSON, 키 필요)
│   ├── searxng.rs      API (JSON, 자체 호스팅)
│   ├── bing.rs         API (JSON, 키 필요)
│   ├── brave.rs        API (JSON, 키 필요)
│   ├── zhipu.rs        API (JSON, 키 필요 — 智谱 Web Search)
│   ├── bocha.rs        API (JSON, 키 필요 — 博查 Web Search)
│   └── metaso.rs       API (JSON, 키 필요 — 秘塔 Web Search)
├── client.rs       SearchClient + SearchOptions
├── extractor.rs    전체 페이지 본문 가져오기 (--fetch용)
└── result.rs       SearchResult / SearchItem / SearchMode
```

## 두 가지 실행 경로, 하나의 결과 타입

모든 경로는 [`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs)로
수렴합니다:

```
query + Engine ─► SearchClient ─► engines_impl/* ─► 통합 ─► SearchResult
```

- **API** — `engines_impl::<engine>::search(&http, query, &opts)`가 제공자를 호출하고,
  JSON을 `SearchItem`으로 역직렬화합니다.
- **스크랩** — 동일한 시그니처이지만 HTML 결과 페이지를 파싱합니다.

`SearchMode`(`Api` / `Scrape`)는 어떤 경로가 결과를 만들어냈는지 기록하므로,
호출자는 구조화된 API 응답과 스크랩된 페이지를 구별할 수 있습니다.

## 디스패치

`SearchClient::search_with_options`는 `Engine`에 대한 단순 `match`입니다. 백엔드를
추가하려면: `engines_impl/`에 함수를 하나 구현하고, `Engine` 변형을 하나 추가하고,
`match` 분기를 하나 추가하면 됩니다. trait 객체나 동적 디스패치는 없습니다 — 엔진
집합은 닫혀 있고 컴파일 시점에 알려져 있으므로, API를 예측 가능하게 유지하고
바이너리를 작게 만듭니다.

## 헤드리스 브라우저 없음

seia는 의도적으로 브라우저 자동화를 **포함하지 않습니다**. 모든 백엔드는 일반 HTTP
클라이언트입니다. 비브라우저 트래픽을 강력하게 차단하는 엔진(Google, Baidu, Yandex 웹
검색)은 다루지 않습니다 — 공식 API를 통해 접근하거나, 독립형 MCP로 사용 가능해지면
[shirabe](https://github.com/celestia-island/shirabe) 같은 전용 브라우저 도구를
사용하세요.

## 본문 보강

`SearchOptions::fetch_content`은 직교하는 관심사입니다: 엔진이 `SearchItem`을 반환한 후,
`extractor::fetch_content`가 각 페이지를 다운로드하고 정리합니다. 이는 엔진과 무관합니다.
