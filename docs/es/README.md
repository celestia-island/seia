<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>Conocimiento de todas las fuentes</strong>

Búsqueda web multimotor para Rust. Los motores gratuitos funcionan desde el primer momento

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

seia te permite buscar en la web a través de DuckDuckGo, Tavily, Wikipedia, SearXNG,
Bing, Brave, Google, Baidu y más, todo detrás de una sola interfaz. Los motores
gratuitos funcionan sin configuración alguna.

## Inicio rápido

### CLI

```bash
# Búsqueda básica (DuckDuckGo, gratuito, sin clave)
seia search "rust async patterns"

# Wikipedia (gratuito, académico)
seia search "Klein bottle" --engine wikipedia

# Salida JSON
seia search "climate change" --json

# A través de un proxy
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# Modo navegador (Google/Baidu vía tairitsu)
seia search "query" --engine google --browser
```

### Biblioteca

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::DuckDuckGo).await?;
```

## Motores

| Motor | Modo | Autenticación | Estado |
|-------|------|---------------|--------|
| DuckDuckGo | Raspado | Ninguno | ✅ |
| Wikipedia | API | Ninguno | ✅ |
| SearXNG | API | `SEARXNG_URL` | ✅ |
| Tavily | API | `TAVILY_API_KEY` | ✅ |
| Bing | API | `BING_SEARCH_API_KEY` | 🔲 |
| Brave | API | `BRAVE_SEARCH_API_KEY` | 🔲 |
| Google | Navegador | tairitsu | ✅ |
| Baidu | Navegador | tairitsu | ✅ |
| Bing Web | Navegador | tairitsu | ✅ |
| Yandex | Navegador | tairitsu | ✅ |

Los motores en modo navegador usan [tairitsu](https://github.com/celestia-island/tairitsu)
para el renderizado sin interfaz. Puedes ejecutar un demonio independiente o habilitar
la característica `embedded-browser` para compilar tairitsu dentro del proceso.

## Desarrollo

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## Licencia

SySL-1.0 (Synthetic Source License). Consulta [LICENSE](../../LICENSE).
