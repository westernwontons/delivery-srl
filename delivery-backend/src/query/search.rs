/// Used by `MongoDB` full-text search to query the `customer` collection
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SearchQuery {
    pub query: String
}
