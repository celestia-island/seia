# 빠른 시작

## 설치

```bash
# crates.io에서 (게시 후)
cargo install seia

# 소스에서
cargo install --path .
```

## 첫 검색 (CLI)

기본 엔진은 DuckDuckGo입니다 — 무료, 키 불필요, 바로 사용 가능:

```bash
seia search "rust async patterns"

# 다른 엔진 선택
seia search "Klein bottle" --engine wikipedia

# 기계 판독 가능 출력
seia search "climate change" --json

# 각 결과의 전체 페이지 본문 가져오기 (느림)
seia search "tokio runtime" --fetch
```

`seia engines`을 실행하면 모든 엔진과 키 필요 여부를 확인할 수 있습니다.

## 키가 필요한 엔진

셸에서 키를 내보내면 seia가 자동으로 읽습니다:

```bash
export TAVILY_API_KEY=tvly-xxxxx
seia search "react server components" --engine tavily

export SEARXNG_URL=http://localhost:8080
seia search "open source licenses" --engine searxng
```

## 프록시를 통한 검색

`SearchClient`는 reqwest를 통해 표준 `HTTPS_PROXY` / `HTTP_PROXY` 환경변수를
준수합니다 — 별도의 플래그가 필요 없습니다:

```bash
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

## 라이브러리로 사용

```rust
use seia::{SearchClient, Engine};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = SearchClient::new();
    let result = client.search("rust async", Engine::Duckduckgo).await?;

    for item in &result.items {
        println!("{} — {}", item.title, item.url);
    }
    Ok(())
}
```

전체 엔진 매트릭스는 [엔진](./engines.md)을, 프로그래밍 API는
[라이브러리 사용법](./library.md)을 참조하세요.
