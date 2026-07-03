# Motores

seia admite 9 backends, todos accesibles a través de su API HTTP oficial (o, donde no
existe API, mediante un raspado ligero del HTML). No hay ningún navegador sin interfaz:
seia es un cliente HTTP puro, por lo que cada motor funciona igual desde el CLI y desde
la biblioteca a través del mismo enum `Engine`.

La mayoría de los motores exponen un nivel gratuito; los que necesitan una clave la leen
de una variable de entorno documentada, de modo que ninguna clave aparece nunca en el
código ni en los argumentos del CLI.

## Dos modos de ejecución

| Modo | Cómo funciona | Usado por |
| --- | --- | --- |
| **API** | Llama a la API HTTP del proveedor de búsqueda y analiza el JSON. | Tavily, SearXNG, Wikipedia, Bing, Brave, 智谱, 博查, 秘塔 |
| **Raspado** | Descarga la página HTML ligera de resultados y extrae las coincidencias. | DuckDuckGo |

## Matriz de motores

### Internacional

| Motor | Enum | Modo | Autenticación | Nivel gratuito | Estado |
| --- | --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | Raspado | ninguno | ilimitado | ✅ |
| Wikipedia | `Wikipedia` | API | ninguno | ilimitado | ✅ |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | autoalojado | ✅ |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1 000/mes | ✅ |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1 000/mes | ✅ |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2 000/mes | ✅ |

### Nacional (China)

| Motor | Enum | Modo | Autenticación | Estado |
| --- | --- | --- | --- | --- |
| 智谱 (Zhipu / BigModel) | `Zhipu` | API | `ZHIPU_API_KEY` | ✅ |
| 博查 (Bocha) | `Bocha` | API | `BOCHA_API_KEY` | ✅ |
| 秘塔 (Metaso) | `Metaso` | API | `METASO_API_KEY` | ✅ |

> La Web Search API de 智谱 puede enrutar a través de uno de varios motores de respaldo:
> 智谱基础版 (`search_std`, por defecto), 智谱高阶版 (`search_pro`), 搜狗
> (`search_pro_sogou`) o 夸克 (`search_pro_quark`). Selecciona uno con la variable de
> entorno `ZHIPU_SEARCH_ENGINE`.

> 博查 devuelve tanto un `snippet` corto como un `summary` más largo generado por un LLM
> por cada página; seia expone el más largo de los dos como `snippet` del resultado.

> 秘塔 (Metaso) busca por defecto en el ámbito `webpage`; anula el ámbito con la variable
> de entorno `METASO_SCOPE`. El sobre de la respuesta se analiza de forma defensiva: si la
> estructura que devuelve el proveedor cambia, seia extrae los resultados que pueda en
> lugar de fallar.

## Seleccionar un motor

CLI:

```bash
seia search "query" --engine wikipedia
seia search "查询" --engine zhipu      # necesita ZHIPU_API_KEY
```

Biblioteca:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("query", Engine::Wikipedia).await?;
client.search("查询", Engine::Zhipu).await?;   // necesita ZHIPU_API_KEY
```

## Inspeccionar los metadatos del motor

`Engine` incorpora sus propios metadatos:

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing, Engine::Zhipu] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing / zhipu
    println!("  needs key? {}", engine.needs_key());
    println!("  key env:    {:?}", engine.api_key_env());
}
```
