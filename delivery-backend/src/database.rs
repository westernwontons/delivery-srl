use edgedb_tokio::{Client, Error as EdgeDbError};
use std::sync::Arc;

pub struct Database {
    #[allow(dead_code)]
    client: Client
}

impl Database {
    fn new(client: Client) -> Self {
        Self { client }
    }

    #[allow(dead_code)]
    async fn create_client(&self) {
        todo!()
    }

    #[allow(dead_code)]
    async fn edit_client(&self) {
        todo!()
    }

    #[allow(dead_code)]
    async fn activate_client(&self) {
        todo!()
    }

    #[allow(dead_code)]
    async fn deactivate_client(&self) {
        todo!()
    }

    #[allow(dead_code)]
    async fn expired_clients(&self) {
        todo!()
    }
}

/// Setup the `Database` with an initialized EdgeDB Client
pub async fn setup_database() -> Result<Arc<Database>, EdgeDbError> {
    let edgedb_client = edgedb_tokio::create_client().await?;
    Ok(Arc::new(Database::new(edgedb_client)))
}
