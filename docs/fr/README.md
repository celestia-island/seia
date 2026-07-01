<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>La connaissance de toutes les sources</strong>

Recherche web multi-moteurs pour Rust. Les moteurs gratuits fonctionnent immédiatement

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](../../LICENSE)
[![Crates.io](https://img.shields.io/crates/v/seia)](https://docs.rs/seia)
[![CI](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)

[English](../en/README.md) · [简体中文](../zhs/README.md) ·
[繁體中文](../zht/README.md) · [日本語](../ja/README.md) ·
[한국어](../ko/README.md) · **Français** ·
[Español](../es/README.md) · [Русский](../ru/README.md) ·
[العربية](../ar/README.md)

</div>

## Introduction

seia vous permet d'effectuer des recherches sur le web via DuckDuckGo, Tavily, Wikipedia, SearXNG,
Bing, Brave, Google, Baidu, et plus encore — le tout derrière une seule interface. Les moteurs
gratuits fonctionnent sans aucune configuration.

## Démarrage rapide

### CLI

```bash
# Recherche de base (DuckDuckGo, gratuit, sans clé)
seia search "rust async patterns"

# Wikipedia (gratuit, académique)
seia search "Klein bottle" --engine wikipedia

# Sortie JSON
seia search "climate change" --json

# Via un proxy
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# Mode navigateur (Google/Baidu via tairitsu)
seia search "query" --engine google --browser
```

### Bibliothèque

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::DuckDuckGo).await?;
```

## Moteurs

| Moteur | Mode | Authentification | État |
|--------|------|------|--------|
| DuckDuckGo | Scraping | aucun | ✅ |
| Wikipedia | API | aucun | ✅ |
| SearXNG | API | `SEARXNG_URL` | ✅ |
| Tavily | API | `TAVILY_API_KEY` | ✅ |
| Bing | API | `BING_SEARCH_API_KEY` | 🔲 |
| Brave | API | `BRAVE_SEARCH_API_KEY` | 🔲 |
| Google | Navigateur | tairitsu | ✅ |
| Baidu | Navigateur | tairitsu | ✅ |
| Bing Web | Navigateur | tairitsu | ✅ |
| Yandex | Navigateur | tairitsu | ✅ |

Les moteurs en mode navigateur utilisent [tairitsu](https://github.com/celestia-island/tairitsu)
pour le rendu headless. Lancez soit un démon autonome, soit activez la fonctionnalité
`embedded-browser` pour compiler tairitsu dans le processus.

## Développement

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## Licence

SySL-1.0 (Synthetic Source License). Voir [LICENSE](../../LICENSE).
