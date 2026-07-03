# Поисковые движки

seia поддерживает 9 бэкендов, все они доступны через официальный HTTP-API (или,
при отсутствии API, через лёгкий парсинг HTML). Никакого headless-браузера нет —
seia — это чистый HTTP-клиент, поэтому каждый движок работает и из CLI, и из
библиотеки через одно и то же перечисление `Engine`.

Большинство движков предоставляет бесплатный лимит; те из них, что требуют ключ,
считывают его из документированной переменной окружения, поэтому ключ никогда не
фигурирует ни в коде, ни в аргументах CLI.

## Два режима выполнения

| Режим | Как работает | Используется |
| --- | --- | --- |
| **API** | Вызывает HTTP-API поискового провайдера, разбирает JSON. | Tavily, SearXNG, Wikipedia, Bing, Brave, 智谱, 博查, 秘塔 |
| **Парсинг** | Загружает лёгкую HTML-страницу результатов, извлекает совпадения. | DuckDuckGo |

## Матрица движков

### Международные

| Движок | Значение enum | Режим | Аутентификация | Бесплатный лимит | Статус |
| --- | --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | Парсинг | нет | без ограничений | ✅ |
| Wikipedia | `Wikipedia` | API | нет | без ограничений | ✅ |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | собственный хостинг | ✅ |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1 000 / месяц | ✅ |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1 000 / месяц | ✅ |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2 000 / месяц | ✅ |

### Национальные (Китай)

| Движок | Значение enum | Режим | Аутентификация | Статус |
| --- | --- | --- | --- | --- |
| 智谱 (Zhipu / BigModel) | `Zhipu` | API | `ZHIPU_API_KEY` | ✅ |
| 博查 (Bocha) | `Bocha` | API | `BOCHA_API_KEY` | ✅ |
| 秘塔 (Metaso) | `Metaso` | API | `METASO_API_KEY` | ✅ |

> API веб-поиска 智谱 может маршрутизироваться через один из нескольких
> нижележащих движков — 智谱基础版 (`search_std`, по умолчанию), 智谱高阶版
> (`search_pro`), 搜狗 (`search_pro_sogou`) или 夸克 (`search_pro_quark`).
> Выберите нужный через переменную окружения `ZHIPU_SEARCH_ENGINE`.

> 博查 возвращает на каждую страницу как короткий `snippet`, так и более
> развёрнутый сгенерированный LLM `summary`; seia выводит как результат то, что
> длиннее.

> 秘та по умолчанию ищет в скоупе `webpage`. Изменить его можно переменной
> окружения `METASO_SCOPE`. Оболочка ответа разбирается защитно: seia
> рассчитывает на отсутствие части полей и проверяет их перед чтением.

## Выбор движка

CLI:

```bash
seia search "query" --engine wikipedia
seia search "查询" --engine zhipu      # нужен ZHIPU_API_KEY
```

Библиотека:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("query", Engine::Wikipedia).await?;
client.search("查询", Engine::Zhipu).await?;   // нужен ZHIPU_API_KEY
```

## Просмотр метаданных движка

`Engine` несёт собственные метаданные:

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing, Engine::Zhipu] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing / zhipu
    println!("  нужен ключ? {}", engine.needs_key());
    println!("  env ключа:  {:?}", engine.api_key_env());
}
```
