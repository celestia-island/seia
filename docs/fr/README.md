<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>La connaissance de toutes les sources</strong>

Bibliothèque et CLI de recherche web multi-moteurs.

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

# Mode navigateur (headless, sans clé API)
seia search "query" --browser --browser-engine google
```

### Bibliothèque

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Duckduckgo).await?;
```

## Développement

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## Moteurs supportés

### Moteurs API / scraping

| Moteur | Site officiel | Mode | Authentification | Quota gratuit | État |
|--------|-------------|------|-----------------|-------------|------|
| DuckDuckGo | [duckduckgo.com](https://duckduckgo.com) | Scraping | aucun | illimité | ✅ |
| Wikipedia | [wikipedia.org](https://www.wikipedia.org) | API | aucun | illimité | ✅ |
| SearXNG | [searxng.org](https://searxng.org) | API | `SEARXNG_URL` | auto-hébergé | ✅ |
| Tavily | [tavily.com](https://tavily.com) | API | `TAVILY_API_KEY` | 1 000/mois | ✅ |
| Bing | [bing.com](https://www.bing.com) | API | `BING_SEARCH_API_KEY` | 1 000/mois | 🔜 |
| Brave | [brave.com/search](https://brave.com/search) | API | `BRAVE_SEARCH_API_KEY` | 2 000/mois | 🔜 |

> Les backends API Bing et Brave sont des stubs (non encore implémentés). Utilisez
> les profils navigateur comme solution temporaire, ou
> [contribuez](https://github.com/celestia-island/seia).

### Moteurs navigateur (CLI uniquement)

| Moteur | Site officiel | Authentification | Description |
|--------|-------------|-----------------|-------------|
| Google | [google.com](https://www.google.com) | aucun (scraping via tairitsu) | Recherche web Google. |
| Baidu | [baidu.com](https://www.baidu.com) | aucun (scraping via tairitsu) | Recherche web Baidu. |
| Bing Web | [bing.com](https://www.bing.com) | aucun (scraping via tairitsu) | Résultats web Bing. |
| Yandex | [yandex.com](https://yandex.com) | aucun (scraping via tairitsu) | Recherche web Yandex. |

Les moteurs en mode navigateur utilisent [tairitsu](https://github.com/celestia-island/tairitsu)
pour le rendu headless. Lancez soit un démon autonome, soit activez la fonctionnalité
`embedded-browser` pour compiler tairitsu dans le processus.

> La plupart des moteurs de recherche proposent des API REST officielles. Les profils
> navigateur sont une solution de contournement pour les moteurs dont le backend API n'est
> pas encore implémenté, ou lorsque l'API n'est pas disponible gratuitement. À terme,
> chaque profil navigateur recevra une variante `Engine` avec prise en charge par clé API.

## Licence

SySL-1.0 (Synthetic Source License). Voir [LICENSE](../../LICENSE).
