//! Engine implementations — one module per backend.

pub mod duckduckgo;
pub mod tavily;
pub mod searxng;
pub mod wikipedia;

use crate::result::SearchItem;

/// Common return type for engine implementations.
pub type EngineOutput = (Vec<SearchItem>, crate::result::SearchMode);
