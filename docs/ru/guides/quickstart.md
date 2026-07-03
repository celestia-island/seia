# Быстрый старт

## Установка

```bash
# Из crates.io (после публикации)
cargo install seia

# Из исходников
cargo install --path .
```

## Первый поиск (CLI)

Поисковый движок по умолчанию — DuckDuckGo: бесплатно, без ключа, работает сразу:

```bash
seia search "rust async patterns"

# Выбрать другой движок
seia search "Klein bottle" --engine wikipedia

# Машинно-читаемый вывод
seia search "climate change" --json

# Получить полный текст страницы для каждого результата (медленнее)
seia search "tokio runtime" --fetch
```

Выполните `seia engines`, чтобы увидеть список всех движков и узнать, нужен ли ключ.

## Движки, требующие ключа

Экспортируйте ключ в вашей оболочке — seia считает его автоматически:

```bash
export TAVILY_API_KEY=tvly-xxxxx
seia search "react server components" --engine tavily

export SEARXNG_URL=http://localhost:8080
seia search "open source licenses" --engine searxng
```

## Через прокси

```bash
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

## Использование как библиотеки

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

Дальше см. [Поисковые движки](./engines.md) с полной матрицей движков или
[Использование библиотеки](./library.md) с описанием программного API.
