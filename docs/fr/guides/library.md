# Utilisation de la bibliothèque

Tout ce que fait la CLI est disponible en tant que bibliothèque. L'API publique se trouve à la
racine de la crate dans [`lib.rs`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs).

## Le client de recherche

`SearchClient` possède un unique `reqwest::Client` (avec un user agent de type navigateur et un
délai d'attente de 15 s) et distribue la requête au bon backend selon l'`Engine` :

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client.search("rust async", Engine::Duckduckgo).await?;
```

### Avec un proxy

`SearchClient::with_proxy` configure un proxy explicite. Le client respecte également
`HTTPS_PROXY` / `HTTP_PROXY` automatiquement via reqwest.

```rust
let client = SearchClient::with_proxy("http://localhost:7890");
```

## Options de recherche

`search_with_options` prend un `SearchOptions` pour la limite, la récupération de contenu et
l'URL de l'instance SearXNG :

```rust
use seia::{SearchClient, Engine, SearchOptions};

let opts = SearchOptions {
    limit: Some(5),
    fetch_content: true,        // télécharge le texte de chaque page (plus lent)
    searxng_url: Some("http://localhost:8080".into()),
};

let result = client
    .search_with_options("query", Engine::Searxng, opts)
    .await?;
```

| Champ | Défaut | Signification |
| --- | --- | --- |
| `limit` | `Some(10)` | Tronque la liste des résultats. |
| `fetch_content` | `false` | Récupère le texte complet de la page dans `SearchItem::content`. |
| `searxng_url` | `None` | URL de base SearXNG (lue depuis `SEARXNG_URL` sinon). |

## Repli entre moteurs

`search_fallback` essaie une liste de moteurs dans l'ordre et renvoie le premier qui
produit des résultats non vides — pratique lorsqu'un backend gratuit est limité en débit :

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let result = client
    .search_fallback("query", &[Engine::Duckduckgo, Engine::Wikipedia])
    .await?;
```

## Types de résultat

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
    pub snippet: Option<String>,   // court résumé fourni par le moteur
    pub content: Option<String>,   // texte complet de la page (uniquement avec --fetch)
}
```

Tous deux dérivent `Serialize`/`Deserialize`, donc `serde_json::to_string(&result)` produit
exactement ce qu'affiche `seia search --json`.

## Prélude

Pour des imports plus concis :

```rust
use seia::prelude::*;   // SearchClient, Engine, SearchResult, SearchItem, SearchMode
```
