#![allow(
    unused_variables,
    unused_mut,
    unused_imports,
    dead_code,
    unused_assignments
)]

use crate::app_error::AppError;
use crate::expiration::TimeRange;
use crate::{customer::DeliveryCustomer, update::PartialCustomerUpdate};

use axum::{http::StatusCode, response::IntoResponse};
use mongodb::{options::ClientOptions, Client as MongoClient};
use std::sync::Arc;
use uuid::Uuid;

pub struct Database {
    client: MongoClient
}

impl Database {
    /// Creates a new [`Database`].
    pub fn new(client: MongoClient) -> Self {
        Self { client }
    }

    /// Commit a [`DeliveryCustomer`] to the database
    pub async fn create_customer(&self, customer: DeliveryCustomer) {
        todo!()
    }

    /// Update a [`DeliveryCustomer`] in the database
    pub async fn update_customer(&self, customer: PartialCustomerUpdate) {
        todo!()
    }

    /// Activate a [`DeliveryCustomer`]
    pub async fn activate_customer(&self, customer_id: String) {
        todo!()
    }

    /// Deactivate a [`DeliveryCustomer`]
    pub async fn deactivate_customer(&self, customer_id: String) {
        todo!()
    }

    /// Fetch expired [`DeliveryCustomer`]s
    ///
    /// Optionally, an [`ExpirationRange`] can be provided, which will be used to
    /// only return [`DeliveryCustomer`]s between a given time range
    pub async fn expired_customers(&self, range: Option<TimeRange>) {
        todo!()
    }
}

/// Initialize the [`Database`] with a MongoDB [`MongoClient`]
pub async fn setup_database() -> Result<Arc<Database>, AppError> {
    let options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let mongodb_client = MongoClient::with_options(options)?;
    Ok(Arc::new(Database::new(mongodb_client)))
}
