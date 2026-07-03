# Uso como biblioteca

Todo lo que hace el CLI está disponible como biblioteca. La API pública reside en la
raíz del crate, en [`lib.rs`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs).

## El cliente de búsqueda

`SearchClient` posee un único `reqwest::Client` (con un agente de usuario tipo
navegador y un tiempo de espera de 15 s) y deriva al backend correcto según `Engine`:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client.search("rust async", Engine::Duckduckgo).await?;
```

### Con un proxy

`SearchClient::with_proxy` configura un proxy explícito. El cliente también respeta
`HTTPS_PROXY` / `HTTP_PROXY` automáticamente a través de reqwest.

```rust
let client = SearchClient::with_proxy("http://localhost:7890");
```

## Opciones de búsqueda

`search_with_options` recibe un `SearchOptions` para el límite, la obtención de
contenido y la URL de la instancia de SearXNG:

```rust
use seia::{SearchClient, Engine, SearchOptions};

let opts = SearchOptions {
    limit: Some(5),
    fetch_content: true,        // descargar el texto de cada página (más lento)
    searxng_url: Some("http://localhost:8080".into()),
};

let result = client
    .search_with_options("query", Engine::Searxng, opts)
    .await?;
```

| Campo | Por defecto | Significado |
| --- | --- | --- |
| `limit` | `Some(10)` | Trunca la lista de resultados. |
| `fetch_content` | `false` | Obtiene el texto completo de la página en `SearchItem::content`. |
| `searxng_url` | `None` | URL base de SearXNG (se lee de `SEARXNG_URL` en caso contrario). |

## Respaldo entre motores

`search_fallback` prueba una lista de motores en orden y devuelve el primero que
produzca resultados no vacíos — útil cuando un backend gratuito está limitado por tasa:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client
    .search_fallback("query", &[Engine::Duckduckgo, Engine::Wikipedia])
    .await?;
```

## Tipos de resultado

```rust
pub struct SearchResult {
    pub engine: String,
    pub query: String,
    pub items: Vec<SearchItem>,
    pub elapsed_ms: u64,
}

pub struct SearchItem {
    pub title: String,
    pub url: String,
    pub snippet: Option<String>,   // resumen breve del motor
    pub content: Option<String>,   // texto completo de la página (solo con --fetch)
}
```

Ambos derivan `Serialize`/`Deserialize`, por lo que `serde_json::to_string(&result)`
produce exactamente lo que imprime `seia search --json`.

## Prelude

Para importaciones más concisas:

```rust
use seia::prelude::*;   // SearchClient, Engine, SearchResult, SearchItem, SearchMode
```
