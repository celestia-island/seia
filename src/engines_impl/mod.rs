//! Engine implementations — one module per backend.

pub mod arxiv;
pub mod bing;
pub mod bocha;
pub mod brave;
pub mod core;
pub mod crossref;
pub mod custom;
pub mod doaj;
pub mod duckduckgo;
pub mod metaso;
pub mod openalex;
pub mod pubmed;
pub mod searxng;
pub mod semantic_scholar;
pub mod tavily;
pub mod wikipedia;
pub mod zhipu;

use crate::result::SearchItem;

/// Common return type for engine implementations.
pub type EngineOutput = (Vec<SearchItem>, crate::result::SearchMode);
