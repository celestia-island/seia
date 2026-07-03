//! Engine implementations — one module per backend.

pub mod bing;
pub mod bocha;
pub mod brave;
pub mod duckduckgo;
pub mod metaso;
pub mod searxng;
pub mod tavily;
pub mod wikipedia;
pub mod zhipu;

use crate::result::SearchItem;

/// Common return type for engine implementations.
pub type EngineOutput = (Vec<SearchItem>, crate::result::SearchMode);
