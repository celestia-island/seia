<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>Знания из любых источников</strong>

Мультидвижковый веб-поиск для Rust. Бесплатные движки работают сразу

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

seia позволяет искать в сети через DuckDuckGo, Tavily, Wikipedia, SearXNG,
Bing, Brave, Google, Baidu и другие — всё за одним интерфейсом. Бесплатные
движки работают сразу и без всякой настройки.

## Быстрый старт

### CLI

```bash
# Базовый поиск (DuckDuckGo, бесплатно, без ключа)
seia search "rust async patterns"

# Wikipedia (бесплатно, академический)
seia search "Klein bottle" --engine wikipedia

# Вывод в JSON
seia search "climate change" --json

# Через прокси
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# Режим браузера (Google/Baidu через tairitsu)
seia search "query" --engine google --browser
```

### Библиотека

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::DuckDuckGo).await?;
```

## Поисковые движки

| Движок | Режим | Аутентификация | Статус |
|--------|------|------|--------|
| DuckDuckGo | Парсинг | нет | ✅ |
| Wikipedia | API | нет | ✅ |
| SearXNG | API | `SEARXNG_URL` | ✅ |
| Tavily | API | `TAVILY_API_KEY` | ✅ |
| Bing | API | `BING_SEARCH_API_KEY` | 🔲 |
| Brave | API | `BRAVE_SEARCH_API_KEY` | 🔲 |
| Google | Браузер | tairitsu | ✅ |
| Baidu | Браузер | tairitsu | ✅ |
| Bing Web | Браузер | tairitsu | ✅ |
| Yandex | Браузер | tairitsu | ✅ |

Движки в режиме браузера используют [tairitsu](https://github.com/celestia-island/tairitsu)
для рендеринга в headless-режиме. Можно либо запустить отдельный демон, либо включить
функцию `embedded-browser`, чтобы скомпилировать tairitsu прямо в процесс.

## Разработка

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## Лицензия

SySL-1.0 (Synthetic Source License). См. [LICENSE](../../LICENSE).
