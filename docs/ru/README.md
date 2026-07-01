<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>Знания из любых источников</strong>

Мультидвижковая библиотека и CLI для веб-поиска.

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](../../LICENSE)
[![Crates.io](https://img.shields.io/crates/v/seia)](https://docs.rs/seia)
[![CI](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)

[English](../en/README.md) · [简体中文](../zhs/README.md) ·
[繁體中文](../zht/README.md) · [日本語](../ja/README.md) ·
[한국어](../ko/README.md) · [Français](../fr/README.md) ·
[Español](../es/README.md) · **Русский** ·
[العربية](../ar/README.md)

</div>

## Введение

seia — это мультидвижковая библиотека и CLI для веб-поиска. Она предоставляет единый
интерфейс для запросов к различным поисковым бэкендам. Движки, не требующие
аутентификации, работают сразу без какой-либо настройки.

## Быстрый старт

### CLI

```bash
# Базовый поиск (без API-ключа)
seia search "rust async patterns"

# Выбор конкретного движка
seia search "Klein bottle" --engine wikipedia

# Вывод в JSON
seia search "climate change" --json

# Через прокси
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# Режим браузера (headless, без API-ключа)
seia search "query" --browser --browser-engine google
```

### Библиотека

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Duckduckgo).await?;
```

## Разработка

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## Поддерживаемые поисковые движки

### API / парсинг

| Движок | Официальный сайт | Режим | Аутентификация | Бесплатный лимит | Статус |
|--------|-----------------|------|---------------|-----------------|--------|
| DuckDuckGo | [duckduckgo.com](https://duckduckgo.com) | Парсинг | нет | безлимитный | ✅ |
| Wikipedia | [wikipedia.org](https://www.wikipedia.org) | API | нет | безлимитный | ✅ |
| SearXNG | [searxng.org](https://searxng.org) | API | `SEARXNG_URL` | свой хостинг | ✅ |
| Tavily | [tavily.com](https://tavily.com) | API | `TAVILY_API_KEY` | 1 000/мес | ✅ |
| Bing | [bing.com](https://www.bing.com) | API | `BING_SEARCH_API_KEY` | 1 000/мес | 🔜 |
| Brave | [brave.com/search](https://brave.com/search) | API | `BRAVE_SEARCH_API_KEY` | 2 000/мес | 🔜 |

> API-бэкенды Bing и Brave являются заглушками (пока не реализованы). Используйте
> браузерные профили как временное решение или
> [внесите свой вклад](https://github.com/celestia-island/seia).

### Браузерные движки (только CLI)

| Движок | Официальный сайт | Аутентификация | Описание |
|--------|-----------------|---------------|----------|
| Google | [google.com](https://www.google.com) | нет (парсинг через tairitsu) | Веб-поиск Google. |
| Baidu | [baidu.com](https://www.baidu.com) | нет (парсинг через tairitsu) | Веб-поиск Baidu. |
| Bing Web | [bing.com](https://www.bing.com) | нет (парсинг через tairitsu) | Веб-результаты Bing. |
| Yandex | [yandex.com](https://yandex.com) | нет (парсинг через tairitsu) | Веб-поиск Yandex. |

Движки в режиме браузера используют [tairitsu](https://github.com/celestia-island/tairitsu)
для рендеринга в headless-режиме. Можно либо запустить отдельный демон, либо включить
функцию `embedded-browser`, чтобы скомпилировать tairitsu прямо в процесс.

> Большинство поисковых систем предлагают официальные REST API. Браузерные профили —
> это обходной путь для движков, чей API-бэкенд ещё не реализован, или когда API
> недоступен бесплатно. В перспективе каждый браузерный профиль получит соответствующий
> вариант `Engine` с поддержкой API-ключа.

## Лицензия

SySL-1.0 (Synthetic Source License). См. [LICENSE](../../LICENSE).
