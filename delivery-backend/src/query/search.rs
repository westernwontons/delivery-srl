use std::fmt::Display;

/// Used by `MongoDB` full-text search to query the `customer` collection
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SearchQuery {
    pub query: String
}

impl Display for SearchQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.query)
    }
}
