/// Search for client in the database. Accepts a `SearchQuery`
/// which contains all the possible fields to search for.
pub async fn search() {
    todo!()
}

/// Represents a search query for clients
#[derive(serde::Serialize, serde::Deserialize)]
struct SearchQuery {}
