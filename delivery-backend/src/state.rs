use crate::database::{setup_database, Database};
use edgedb_tokio::Error as EdgeDbError;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    database: Arc<Database>
}

impl AppState {
    fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

pub async fn setup_app_state() -> Result<AppState, EdgeDbError> {
    Ok(AppState::new(setup_database().await?))
}
