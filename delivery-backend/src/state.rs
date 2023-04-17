use std::sync::Arc;

use crate::{
    auth::store::{setup_store, Store},
    database::{setup_database, Database},
    error::AppError
};

/// Global, app level state
///
/// Is injected to every handler that requires database access.
/// Wraps [`Database`] in an [`Arc`] to help with usage inside tokio tasks
#[derive(Clone, Debug)]
pub struct AppState {
    database: Arc<Database>,
    store: Arc<Store>
}

impl AppState {
    /// Creates a new [`AppState`].
    pub fn new(database: Arc<Database>, store: Arc<Store>) -> Self {
        Self { database, store }
    }

    /// Returns the database of this [`AppState`].
    pub fn database(&self) -> Arc<Database> {
        Arc::clone(&self.database)
    }

    /// Return the store of this [`AppState`]
    pub fn store(&self) -> Arc<Store> {
        Arc::clone(&self.store)
    }
}

/// Setup the application wide state
///
/// Will initialize a [`Database`] instance and wrap it in [`AppState`]
/// Initialization might fail, because a MongoDb instance might not be running, etc
#[tracing::instrument]
pub async fn setup_app_state() -> Result<AppState, AppError> {
    tracing::info!("Setting up AppState");
    Ok(AppState::new(setup_database().await?, setup_store()))
}
