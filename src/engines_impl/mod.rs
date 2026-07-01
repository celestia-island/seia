//! Engine implementations — one module per backend.

pub mod duckduckgo;
pub mod searxng;
pub mod tavily;
pub mod wikipedia;

use crate::result::SearchItem;

/// Common return type for engine implementations.
pub type EngineOutput = (Vec<SearchItem>, crate::result::SearchMode);
