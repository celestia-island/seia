# Architecture

seia est une crate unique qui fournit à la fois une bibliothèque (`src/lib.rs`)
et une CLI (`src/main.rs`). L'objectif de conception est **une interface de
requête, de nombreux backends** : l'appelant choisit un `Engine` et obtient le
même `SearchResult` quel que soit le backend qui l'a produit.

## Carte des modules

```
src/
├── lib.rs          surface de l'API publique
├── main.rs         CLI clap (search / engines)
├── engines.rs      énumération Engine : as_str, api_key_env, needs_key
├── engines_impl/   un module par backend
│   ├── duckduckgo.rs   scraping (HTML)
│   ├── wikipedia.rs    API (JSON)
│   ├── tavily.rs       API (JSON, clé)
│   ├── searxng.rs      API (JSON, auto-hébergé)
│   ├── bing.rs         API (JSON, clé)
│   ├── brave.rs        API (JSON, clé)
│   ├── zhipu.rs        API (JSON, clé — 智谱 Web Search)
│   ├── bocha.rs        API (JSON, clé — 博查 Web Search)
│   └── metaso.rs       API (JSON, clé — 秘塔 Web Search)
├── client.rs       SearchClient + SearchOptions
├── extractor.rs    récupérateur de contenu de page complète (pour --fetch)
└── result.rs       SearchResult / SearchItem / SearchMode
```

## Deux chemins d'exécution, un type de résultat

Tous les chemins convergent vers [`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs) :

```
query + Engine ─► SearchClient ─► engines_impl/* ─► unifier ─► SearchResult
```

- **API** — `engines_impl::<engine>::search(&http, query, &opts)` appelle le
  fournisseur, désérialise le JSON en `SearchItem`s.
- **Scraping** — même signature, mais analyse la page de résultats HTML.

`SearchMode` (`Api` / `Scrape`) enregistre quel chemin a produit un résultat,
afin que les appelants puissent distinguer une réponse API structurée d'une page
scrapée.

## Répartition

`SearchClient::search_with_options` est un `match` à plat sur `Engine`. Ajouter un
backend signifie : implémenter une fonction dans `engines_impl/`, ajouter une
variante à `Engine`, ajouter un bras au `match`. Il n'y a aucun objet trait ni
répartition dynamique — l'ensemble des moteurs est fermé et connu à la
compilation, ce qui rend l'API prévisible et le binaire léger.

## Pas de navigateur headless

seia n'embarque délibérément **aucune** automatisation de navigateur. Chaque
backend est un pur client HTTP. Les moteurs qui bloquent agressivement le trafic
non émis par un navigateur (Google, Baidu, la recherche web de Yandex) sont hors
périmètre — accédez-y via leurs API officielles ou un outil de navigateur dédié
tel que [shirabe](https://github.com/celestia-island/shirabe) lorsqu'il sera
disponible en tant que MCP autonome.

## Enrichissement du contenu

`SearchOptions::fetch_content` est une préoccupation orthogonale : une fois que le
moteur a renvoyé les `SearchItem`s, `extractor::fetch_content` télécharge et
nettoie chaque page. C'est indépendant du moteur.
