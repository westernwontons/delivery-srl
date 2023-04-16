use std::sync::Arc;

use crate::{
    database::{setup_database, Database},
    error::AppError
};

/// Global, app level state
///
/// Is injected to every handler that requires database access.
/// Wraps [`Database`] in an [`Arc`] to help with usage inside tokio tasks
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
///
/// Will initialize a [`Database`] instance and wrap it in [`AppState`]
/// Initialization might fail, because a MongoDb instance might not be running, etc
#[tracing::instrument]
pub async fn setup_app_state() -> Result<AppState, AppError> {
    tracing::info!("Setting up AppState");
    Ok(AppState::new(setup_database().await?))
}
