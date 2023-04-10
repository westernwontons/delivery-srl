use crate::customer::{DeliveryCustomerIn, ExpiredCustomerList};
use crate::error::AppError;
use crate::query::ExpiredCustomersQuery;
use crate::responses::{DeleteResultResponse, UpdateResultResponse};
use crate::{customer::DeliveryCustomerOut, query::PartialDeliveryCustomer};

use mongodb::bson::{self, doc};
use mongodb::options::ClientOptions;
use mongodb::options::UpdateOptions;
use mongodb::Client as MongoClient;
use mongodb::Collection as MongoCollection;
use mongodb::Database as MongoDatabase;
use std::sync::Arc;

pub struct Database {
    client: MongoClient
}

impl Database {
    /// Creates a new [`Database`].
    pub fn new(client: MongoClient) -> Self {
        Self { client }
    }

    /// Returns the `delivery_database` from `MongoDB`.
    fn get_database(&self) -> MongoDatabase {
        self.client.database("delivery_database")
    }

    /// Returns the `customer` collection from `MongoDB`.
    fn customer_collection(&self) -> MongoCollection<DeliveryCustomerIn> {
        self.get_database().collection("customer")
    }

    /// Commit a [`DeliveryCustomerIn`] to the database
    ///
    /// If the customer already exists, update that customer with the new fields.
    pub async fn upsert_customer(
        &self,
        customer: DeliveryCustomerIn
    ) -> Result<UpdateResultResponse, AppError> {
        Ok(self
            .customer_collection()
            .update_one(
                doc! {
                    "customer_id": &customer.customer_id
                },
                customer.into_update_document(),
                UpdateOptions::builder().upsert(Some(true)).build()
            )
            .await?
            .into())
    }

    /// Update a [`DeliveryCustomer`] in the database
    pub async fn update_customer(
        &self,
        customer: PartialDeliveryCustomer
    ) -> Result<UpdateResultResponse, AppError> {
        Ok(self
            .customer_collection()
            .update_one(
                doc! {
                    "customer_id": &customer.customer_id
                },
                customer.into_update_document_no_none(),
                None
            )
            .await?
            .into())
    }

    /// Activate a [`DeliveryCustomer`]
    pub async fn activate_customer(
        &self,
        customer_id: String
    ) -> Result<UpdateResultResponse, AppError> {
        Ok(self
            .customer_collection()
            .update_one(
                doc! {
                    "customer_id": customer_id
                },
                doc! {
                    "$set": {
                        "active": true
                    }
                },
                None
            )
            .await?
            .into())
    }

    /// Deactivate a [`DeliveryCustomer`]
    pub async fn deactivate_customer(
        &self,
        customer_id: String
    ) -> Result<UpdateResultResponse, AppError> {
        Ok(self
            .customer_collection()
            .update_one(
                doc! {
                    "customer_id": customer_id
                },
                doc! {
                    "$set": {
                        "active": false
                    }
                },
                None
            )
            .await?
            .into())
    }

    /// Delete a [`DeliveryCustomer`]
    pub async fn delete_customer(
        &self,
        customer_id: String
    ) -> Result<DeleteResultResponse, AppError> {
        Ok(self
            .customer_collection()
            .delete_one(doc! {"customer_id": customer_id}, None)
            .await?
            .into())
    }

    /// Fetch expired customers
    ///
    /// An [`ExpiredCustomersQuery`] contains the possible query parameters.
    #[tracing::instrument(skip(self))]
    pub async fn expired_customers(
        &self,
        time_range: ExpiredCustomersQuery
    ) -> Result<ExpiredCustomerList, AppError> {
        let mut cursor = self
            .customer_collection()
            .aggregate(time_range.as_aggregation(), None)
            .await?;

        let mut buffer = Vec::<DeliveryCustomerOut>::with_capacity(10);

        while cursor.advance().await? {
            match cursor.deserialize_current() {
                Ok(document) => {
                    let serialized =
                        bson::from_document::<DeliveryCustomerOut>(document)?;
                    buffer.push(serialized);
                }
                Err(err) => return Err(err.into())
            };
        }

        Ok(buffer.into())
    }
}

/// Initialize the [`Database`] with a MongoDB [`MongoClient`]
#[tracing::instrument]
pub async fn setup_database() -> Result<Arc<Database>, AppError> {
    let mut options = ClientOptions::parse("mongodb://localhost:27017").await?;
    options.direct_connection = Some(true);

    let mongodb_client = MongoClient::with_options(options)?;
    Ok(Arc::new(Database::new(mongodb_client)))
}
