use std::sync::Arc;

use crate::database::{setup_database, Database};
use crate::error::AppError;

#[derive(Clone)]
pub struct AppState {
    database: Arc<Database>
}

impl AppState {
    /// Creates a new [`AppState`].
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    /// Returns the database of this [`AppState`].
    pub fn database(&self) -> Arc<Database> {
        Arc::clone(&self.database)
    }
}

/// Setup the application wide state
#[tracing::instrument]
pub async fn setup_app_state() -> Result<AppState, AppError> {
    tracing::info!("Setting up AppState");
    Ok(AppState::new(setup_database().await?))
}
