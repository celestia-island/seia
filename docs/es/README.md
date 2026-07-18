<p align="center"><img src="https://raw.githubusercontent.com/celestia-island/docs.celestia.world/dev/res/logo/seia.webp" alt="Seia" width="240" /></p>

<h1 align="center">Seia</h1>

<p align="center"><strong>Búsqueda web multi-motor</strong></p>

<div align="center">

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](https://sysl.celestia.world)
[![GitHub](https://img.shields.io/badge/github-celestia--island%2Fseia-blue.svg)](https://github.com/celestia-island/seia)
[![Checks](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)
[![docs.rs](https://docs.rs/seia/badge.svg)](https://docs.rs/seia)

</div>

<div align="center">

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
# Basic search (no API key required)
seia search "rust async patterns"

# Choose a specific engine
seia search "Klein bottle" --engine wikipedia

# JSON output
seia search "climate change" --json

# Through a proxy
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

### Biblioteca

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Wikipedia).await?;
```

## Servidor MCP

Construye seia con la feature `mcp` y ejecuta el servidor stdio — expone el cliente de búsqueda multi-motor a los asistentes de codificación de IA a través del Model Context Protocol:

```bash
seia mcp
```

El servidor anuncia tres herramientas: `seia_search` (un motor, duckduckgo por defecto sin necesidad de clave), `seia_search_multi` (prueba una cadena de motores, devuelve el primero con resultados) y `seia_list_engines` (los nueve motores y sus variables de entorno de clave API). Conéctalo a un cliente MCP:

```json
{
  "mcpServers": {
    "seia": { "command": "seia", "args": ["mcp"] }
  }
}
```

Establece `SEIA_PROXY` para enrutar las solicitudes de búsqueda a través de un proxy (ej. `http://localhost:7890`); `HTTPS_PROXY` / `HTTP_PROXY` también se respetan.

## Desarrollo

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
just test-proxy  # run tests through localhost:7890 proxy (see tests/README)
```

## Motores soportados

| Motor | Autenticación |
|-------|---------------|
| [DuckDuckGo](https://duckduckgo.com/) | Ninguno |
| [Wikipedia](https://www.mediawiki.org/wiki/API:Search) | Ninguno |
| [SearXNG](https://docs.searxng.org/) | `SEARXNG_URL` |
| [Tavily](https://docs.tavily.com/) | `TAVILY_API_KEY` |
| [Bing](https://learn.microsoft.com/en-us/bing/search-apis/bing-web-search/) | `BING_SEARCH_API_KEY` |
| [Brave](https://api.search.brave.com/app/documentation) | `BRAVE_SEARCH_API_KEY` |
| [秘塔 (MetaSo)](https://metaso.cn/search-api/playground) | `METASO_API_KEY` |
| [智谱 (Zhipu)](https://docs.bigmodel.cn/cn/guide/tools/web-search) | `ZHIPU_API_KEY` |
| [博查 (Bocha)](https://open.bochaai.com/docs) | `BOCHA_API_KEY` |

## Licencia

SySL-1.0 (Synthetic Source License). Consulta [LICENSE](https://sysl.celestia.world).

## MCP Server Deployment

> (English section — translation pending)

For production MCP deployments, use an **auto-restart wrapper** to keep the server alive across updates without interrupting the client session.

### Recommended launcher

#!/bin/bash
while true; do
  /path/to/seia mcp
  sleep 0.2
done

### How it works

1. The wrapper runs `seia mcp` in a `while true` loop.
2. If the process exits, it restarts within 0.2 seconds.
3. To update: `kill $(pgrep -f "seia mcp" | head -1)`
