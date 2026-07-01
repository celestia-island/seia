# Архитектура

seia — это единый крейт, поставляющий как библиотеку (`src/lib.rs`), так и CLI
(`src/main.rs`). Цель проектирования — **единая поверхность запроса, множество
бэкендов**: вызывающий выбирает `Engine` и получает один и тот же
`SearchResult` независимо от того, какой бэкенд его произвёл.

## Карта модулей

```
src/
├── lib.rs          публичная поверхность API
├── main.rs         clap CLI (search / engines)
├── engines.rs      перечисление Engine: as_str, api_key_env, needs_key
├── engines_impl/   по одному модулю на каждый бэкенд
│   ├── duckduckgo.rs   парсинг (HTML)
│   ├── wikipedia.rs    API (JSON)
│   ├── tavily.rs       API (JSON, ключ)
│   ├── searxng.rs      API (JSON, собственный хостинг)
│   ├── bing.rs         API (JSON, ключ)
│   ├── brave.rs        API (JSON, ключ)
│   ├── zhipu.rs        API (JSON, ключ — 智谱 Web Search)
│   ├── bocha.rs        API (JSON, ключ — 博查 Web Search)
│   └── metaso.rs       API (JSON, ключ — 秘塔 Web Search)
├── client.rs       SearchClient + SearchOptions
├── extractor.rs    загрузчик полного содержимого страницы (для --fetch)
└── result.rs       SearchResult / SearchItem / SearchMode
```

## Два пути выполнения, один тип результата

Все пути сходятся к
[`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs):

```
query + Engine ─► SearchClient ─► engines_impl/* ─► унификация ─► SearchResult
```

- **API** — `engines_impl::<engine>::search(&http, query, &opts)` вызывает
  провайдера и десериализует JSON в `SearchItem`.
- **Парсинг** — та же сигнатура, но разбирается HTML-страница результатов.

`SearchMode` (`Api` / `Scrape`) фиксирует, каким путём получен результат, поэтому
вызывающий может отличить структурированный ответ API от страницы, полученной
парсингом.

## Диспетчеризация

`SearchClient::search_with_options` — это плоский `match` по `Engine`. Добавить
бэкенд означает: реализовать одну функцию в `engines_impl/`, добавить вариант в
`Engine` и добавить ветвь `match`. Никаких trait-объектов или динамической
диспетчеризации — набор движков замкнут и известен на этапе компиляции, что
сохраняет API предсказуемым, а бинарник — компактным.

## Без headless-браузера

В seia намеренно **нет** никакой автоматизации браузера. Каждый бэкенд — это
обычный HTTP-клиент. Движки, агрессивно блокирующие не-браузерный трафик (Google,
Baidu, веб-поиск Yandex), выходят за рамки проекта — обращайтесь к ним через их
официальные API или через специализированный браузерный инструмент вроде
[shirabe](https://github.com/celestia-island/shirabe), когда он станет доступен
как самостоятельный MCP.

## Обогащение содержимым

`SearchOptions::fetch_content` — ортогональная забота: после того как движок
вернёт `SearchItem`, `extractor::fetch_content` загружает и очищает каждую
страницу. Это не зависит от движка.
