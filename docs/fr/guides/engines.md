# Moteurs

seia prend en charge 9 backends, tous atteints via leur API HTTP officielle (ou,
lorsqu'aucune API n'existe, un léger scraping HTML). Il n'y a aucun navigateur
headless — seia est un pur client HTTP, de sorte que chaque moteur fonctionne
aussi bien depuis la CLI que depuis la bibliothèque via la même énumération
`Engine`.

La plupart des moteurs exposent une offre gratuite ; ceux qui nécessitent une clé
la lisent depuis une variable d'environnement documentée, si bien qu'aucune clé
n'apparaît jamais dans le code ou les arguments de la CLI.

## Deux modes d'exécution

| Mode | Fonctionnement | Utilisé par |
| --- | --- | --- |
| **API** | Appelle l'API HTTP d'un fournisseur de recherche, analyse le JSON. | Tavily, SearXNG, Wikipedia, Bing, Brave, 智谱, 博查, 秘塔 |
| **Scraping** | Récupère la légère page de résultats HTML, en extrait les correspondances. | DuckDuckGo |

## Matrice des moteurs

### International

| Moteur | Valeur d'énumération | Mode | Authentification | Offre gratuite | État |
| --- | --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | Scraping | aucun | illimité | ✅ |
| Wikipedia | `Wikipedia` | API | aucun | illimité | ✅ |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | auto-hébergé | ✅ |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1 000/mois | ✅ |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1 000/mois | ✅ |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2 000/mois | ✅ |

### National (Chine)

| Moteur | Valeur d'énumération | Mode | Authentification | État |
| --- | --- | --- | --- | --- |
| 智谱 (Zhipu / BigModel) | `Zhipu` | API | `ZHIPU_API_KEY` | ✅ |
| 博查 (Bocha) | `Bocha` | API | `BOCHA_API_KEY` | ✅ |
| 秘塔 (Metaso) | `Metaso` | API | `METASO_API_KEY` | ✅ |

> L'API Web Search de 智谱 peut router via l'un de plusieurs moteurs sous-jacents —
> 智谱基础版 (`search_std`, par défaut), 智谱高阶版 (`search_pro`), 搜狗
> (`search_pro_sogou`), ou 夸克 (`search_pro_quark`). Sélectionnez-en un avec la
> variable d'environnement `ZHIPU_SEARCH_ENGINE`.

> 博查 renvoie à la fois un court `snippet` et un plus long `summary` généré par
> LLM pour chaque page ; seia expose le plus long des deux comme `snippet` du
> résultat.

> La portée de recherche de 秘塔 est par défaut `webpage` ; surchargez-la avec la
> variable d'environnement `METASO_SCOPE`. L'enveloppe de la réponse est analysée
> de manière défensive afin de tolérer les variations de schéma.

## Choisir un moteur

CLI :

```bash
seia search "query" --engine wikipedia
seia search "查询" --engine zhipu      # nécessite ZHIPU_API_KEY
```

Bibliothèque :

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("query", Engine::Wikipedia).await?;
client.search("查询", Engine::Zhipu).await?;   // nécessite ZHIPU_API_KEY
```

## Inspecter les métadonnées d'un moteur

`Engine` porte ses propres métadonnées :

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing, Engine::Zhipu] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing / zhipu
    println!("  clé requise ? {}", engine.needs_key());
    println!("  var. d'env : {:?}", engine.api_key_env());
}
```
