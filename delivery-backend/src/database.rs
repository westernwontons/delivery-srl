#![allow(
    unused_variables,
    unused_mut,
    unused_imports,
    dead_code,
    unused_assignments
)]
use edgedb_tokio::{Client, Error as EdgeDbError};
use std::sync::Arc;

use crate::{customer::DeliveryCustomer, expiration::ExpirationRange};

pub struct Database {
    client: Client
}

impl Database {
    /// Creates a new [`Database`].
    fn new(client: Client) -> Self {
        Self { client }
    }

    /// Commit a [`DeliveryCustomer`] to the database
    async fn create_customer(&self, customer: DeliveryCustomer) {
        todo!()
    }

    /// Update a [`DeliveryCustomer`] in the database
    async fn update_customer(&self, customer: DeliveryCustomer) {
        todo!()
    }

    /// Activate a [`DeliveryCustomer`]
    async fn activate_customer(&self, customer_id: String) {
        todo!()
    }

    /// Deactivate a [`DeliveryCustomer`]
    async fn deactivate_customer(&self, customer_id: String) {
        todo!()
    }

    /// Fetch expired [`DeliveryCustomer`]s
    ///
    /// An [`ExpirationRange`] can be provided, which will be used to
    /// only return [`DeliveryCustomer`]s between a given time range
    async fn expired_customers(&self, range: Option<ExpirationRange>) {
        todo!()
    }
}

/// Initialize the [`Database`] with an EdgeDB [`Client`]
pub async fn setup_database() -> Result<Arc<Database>, EdgeDbError> {
    let edgedb_client = edgedb_tokio::create_client().await?;
    Ok(Arc::new(Database::new(edgedb_client)))
}
