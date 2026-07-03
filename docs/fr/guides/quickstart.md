# Démarrage rapide

## Installation

```bash
# Depuis crates.io (une fois publié)
cargo install seia

# Depuis les sources
cargo install --path .
```

## Première recherche (CLI)

Le moteur par défaut est DuckDuckGo — gratuit, sans clé, fonctionne immédiatement :

```bash
seia search "rust async patterns"

# Choisir un autre moteur
seia search "Klein bottle" --engine wikipedia

# Sortie lisible par une machine
seia search "climate change" --json

# Récupérer le texte complet de chaque page de résultat (plus lent)
seia search "tokio runtime" --fetch
```

Exécutez `seia engines` pour lister tous les moteurs et savoir s'ils nécessitent une clé.

## Moteurs nécessitant une clé

Exportez la clé dans votre shell — seia la lit automatiquement :

```bash
export TAVILY_API_KEY=tvly-xxxxx
seia search "react server components" --engine tavily

export SEARXNG_URL=http://localhost:8080
seia search "open source licenses" --engine searxng
```

## Via un proxy

`SearchClient` respecte les variables d'environnement standard `HTTPS_PROXY` /
`HTTP_PROXY` via reqwest — aucun drapeau nécessaire :

```bash
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

## Utilisation comme bibliothèque

```rust
use seia::{SearchClient, Engine};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = SearchClient::new();
    let result = client.search("rust async", Engine::Duckduckgo).await?;

    for item in &result.items {
        println!("{} — {}", item.title, item.url);
    }
    Ok(())
}
```

Poursuivez avec [Moteurs](./engines.md) pour la matrice complète des moteurs, ou
[Utilisation de la bibliothèque](./library.md) pour l'API programmatique.
