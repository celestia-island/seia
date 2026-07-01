# Arquitectura

seia es un único crate que incluye tanto una biblioteca (`src/lib.rs`) como un CLI
(`src/main.rs`). El objetivo de diseño es **una superficie de consulta, múltiples
backends**: quien llama elige un `Engine` y obtiene el mismo `SearchResult`
independientemente del backend que lo haya producido.

## Mapa de módulos

```
src/
├── lib.rs          superficie de la API pública
├── main.rs         clap CLI (search / engines)
├── engines.rs      Engine enum: as_str, api_key_env, needs_key
├── engines_impl/   un módulo por backend
│   ├── duckduckgo.rs   raspado (HTML)
│   ├── wikipedia.rs    API (JSON)
│   ├── tavily.rs       API (JSON, clave)
│   ├── searxng.rs      API (JSON, autoalojado)
│   ├── bing.rs         API (JSON, clave)
│   ├── brave.rs        API (JSON, clave)
│   ├── zhipu.rs        API (JSON, clave — 智谱 Web Search)
│   ├── bocha.rs        API (JSON, clave — 博查 Web Search)
│   └── metaso.rs       API (JSON, clave — 秘塔 Web Search)
├── client.rs       SearchClient + SearchOptions
├── extractor.rs    obtenedor de contenido de página completa (para --fetch)
└── result.rs       SearchResult / SearchItem / SearchMode
```

## Dos rutas de ejecución, un tipo de resultado

Todas las rutas convergen en
[`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs):

```
query + Engine ─► SearchClient ─► engines_impl/* ─► unificar ─► SearchResult
```

- **API** — `engines_impl::<engine>::search(&http, query, &opts)` llama al proveedor y
  deserializa el JSON en `SearchItem`.
- **Raspado** — misma firma, pero analiza la página HTML de resultados.

`SearchMode` (`Api` / `Scrape`) registra qué ruta produjo un resultado, de modo que
quien llama pueda distinguir una respuesta estructurada de la API de una página raspada.

## Despacho

`SearchClient::search_with_options` es un `match` plano sobre `Engine`. Añadir un
backend significa: implementar una función en `engines_impl/`, añadir una variante de
`Engine`, añadir un brazo del `match`. No hay trait object ni despacho dinámico: el
conjunto de motores es cerrado y conocido en tiempo de compilación, lo que mantiene la
API predecible y el binario pequeño.

## Sin navegador headless

seia deliberadamente **no** incluye automatización de navegador. Cada backend es un
cliente HTTP puro. Los motores que bloquean de forma agresiva el tráfico que no proviene
de un navegador (Google, Baidu, la búsqueda web de Yandex) quedan fuera del ámbito:
accede a ellos a través de sus APIs oficiales o de una herramienta de navegador dedicada
como [shirabe](https://github.com/celestia-island/shirabe) cuando esté disponible como
MCP independiente.

## Enriquecimiento de contenido

`SearchOptions::fetch_content` es una preocupación ortogonal: una vez que el motor
devuelve los `SearchItem`, `extractor::fetch_content` descarga y limpia cada página.
Esto es independiente del motor.
