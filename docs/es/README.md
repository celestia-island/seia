<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>Conocimiento de todas las fuentes</strong>

Biblioteca y CLI de búsqueda web multimotor.

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](../../LICENSE)
[![Crates.io](https://img.shields.io/crates/v/seia)](https://docs.rs/seia)
[![CI](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)

[English](../en/README.md) · [简体中文](../zhs/README.md) ·
[繁體中文](../zht/README.md) · [日本語](../ja/README.md) ·
[한국어](../ko/README.md) · [Français](../fr/README.md) ·
**Español** · [Русский](../ru/README.md) ·
[العربية](../ar/README.md)

</div>

## Introducción

seia es una biblioteca y CLI de búsqueda web multimotor. Proporciona una interfaz
unificada para consultar diversos backends de búsqueda. Los motores que no requieren
autenticación funcionan de inmediato sin configuración.

## Inicio rápido

### CLI

```bash
# Búsqueda básica (sin clave API)
seia search "rust async patterns"

# Elegir un motor específico
seia search "Klein bottle" --engine wikipedia

# Salida JSON
seia search "climate change" --json

# A través de un proxy
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# Modo navegador (headless, sin clave API)
seia search "query" --browser --browser-engine google
```

### Biblioteca

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Duckduckgo).await?;
```

## Desarrollo

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## Motores soportados

### Motores API / scraping

| Motor | Sitio oficial | Modo | Autenticación | Cuota gratuita | Estado |
|-------|-------------|------|---------------|---------------|--------|
| DuckDuckGo | [duckduckgo.com](https://duckduckgo.com) | Raspado | Ninguno | ilimitado | ✅ |
| Wikipedia | [wikipedia.org](https://www.wikipedia.org) | API | Ninguno | ilimitado | ✅ |
| SearXNG | [searxng.org](https://searxng.org) | API | `SEARXNG_URL` | autoalojado | ✅ |
| Tavily | [tavily.com](https://tavily.com) | API | `TAVILY_API_KEY` | 1 000/mes | ✅ |
| Bing | [bing.com](https://www.bing.com) | API | `BING_SEARCH_API_KEY` | 1 000/mes | 🔜 |
| Brave | [brave.com/search](https://brave.com/search) | API | `BRAVE_SEARCH_API_KEY` | 2 000/mes | 🔜 |

> Los backends API de Bing y Brave son stubs (aún no implementados). Usa los
> perfiles de navegador como solución temporal, o
> [contribuye](https://github.com/celestia-island/seia).

### Motores de navegador (solo CLI)

| Motor | Sitio oficial | Autenticación | Descripción |
|-------|-------------|---------------|-------------|
| Google | [google.com](https://www.google.com) | Ninguno (raspado vía tairitsu) | Búsqueda web de Google. |
| Baidu | [baidu.com](https://www.baidu.com) | Ninguno (raspado vía tairitsu) | Búsqueda web de Baidu. |
| Bing Web | [bing.com](https://www.bing.com) | Ninguno (raspado vía tairitsu) | Resultados web de Bing. |
| Yandex | [yandex.com](https://yandex.com) | Ninguno (raspado vía tairitsu) | Búsqueda web de Yandex. |

Los motores en modo navegador usan [tairitsu](https://github.com/celestia-island/tairitsu)
para el renderizado sin interfaz. Puedes ejecutar un demonio independiente o habilitar
la característica `embedded-browser` para compilar tairitsu dentro del proceso.

> La mayoría de los motores de búsqueda ofrecen API REST oficiales. Los perfiles de
> navegador son una solución alternativa para motores cuyo backend API aún no se ha
> implementado, o cuando la API no está disponible de forma gratuita. A largo plazo,
> cada perfil de navegador recibirá una variante `Engine` con soporte para clave API.

## Licencia

SySL-1.0 (Synthetic Source License). Consulta [LICENSE](../../LICENSE).
