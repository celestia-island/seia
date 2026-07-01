<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>La connaissance de toutes les sources</strong>

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

seia est une bibliothèque et un CLI de recherche web multi-moteurs. Il fournit une
interface unifiée pour interroger divers backends de recherche. Les moteurs qui ne
nécessitent pas d'authentification fonctionnent immédiatement sans configuration.

## Démarrage rapide

### CLI

```bash
# Recherche de base (sans clé API)
seia search "rust async patterns"

# Choisir un moteur spécifique
seia search "Klein bottle" --engine wikipedia

# Sortie JSON
seia search "climate change" --json

# Via un proxy
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

### Bibliothèque

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Wikipedia).await?;
```

## Développement

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## Moteurs de recherche supportés

| Moteur | Authentification | État |
|--------|-----------------|------|
| [Wikipedia](https://www.mediawiki.org/wiki/API:Search) | aucun | ✅ |
| [SearXNG](https://docs.searxng.org/) | `SEARXNG_URL` | ✅ |
| [Tavily](https://docs.tavily.com/) | `TAVILY_API_KEY` | ✅ |
| [Bing](https://learn.microsoft.com/en-us/bing/search-apis/bing-web-search/) | `BING_SEARCH_API_KEY` | ✅ |
| [Brave](https://api.search.brave.com/app/documentation) | `BRAVE_SEARCH_API_KEY` | ✅ |
| [智谱 (Zhipu)](https://docs.bigmodel.cn/cn/guide/tools/web-search) | `ZHIPU_API_KEY` | ✅ |
| [博查 (Bocha)](https://open.bochaai.com/docs) | `BOCHA_API_KEY` | ✅ |

## Licence

SySL-1.0 (Synthetic Source License). Voir [LICENSE](../../LICENSE).
