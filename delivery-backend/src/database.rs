use crate::customer::DeliveryCustomerOut;
use crate::customer::{DeliveryCustomerIn, DeliveryCustomerList};
use crate::error::AppError;
use crate::query::{ExpiredCustomersQuery, PartialDeliveryCustomer};
use crate::responses::{DeleteResultResponse, UpdateResultResponse};

use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::options::UpdateOptions;
use mongodb::Client as MongoClient;
use mongodb::Collection as MongoCollection;
use mongodb::Database as MongoDatabase;
use std::env;
use std::sync::Arc;
use std::time::Duration;

pub struct Database {
    client: MongoClient
}

impl Database {
    /// Creates a new [`Database`].
    pub fn new(client: MongoClient) -> Self {
        Self { client }
    }

    /// Returns the `delivery_database` from `MongoDB`.
    #[tracing::instrument(skip(self))]
    fn get_database(&self) -> MongoDatabase {
        self.client.database("delivery_database")
    }

    /// Returns the `customer` collection from `MongoDB`.
    #[tracing::instrument(skip(self))]
    fn customer_collection(&self) -> MongoCollection<DeliveryCustomerIn> {
        self.get_database().collection("customer")
    }

    /// Commit a [`DeliveryCustomerIn`] to the database
    ///
    /// If the customer already exists, update that customer with the new fields.
    #[tracing::instrument(skip(self))]
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
    #[tracing::instrument(skip(self))]
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
    #[tracing::instrument(skip(self))]
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
    #[tracing::instrument(skip(self))]
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
    #[tracing::instrument(skip(self))]
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
    ) -> Result<DeliveryCustomerList, AppError> {
        let mut cursor = self
            .customer_collection()
            .aggregate(time_range.as_aggregation(), None)
            .await?;

        let mut buffer = Vec::<DeliveryCustomerOut>::with_capacity(10);

        while cursor.advance().await? {
            buffer.push(cursor.deserialize_current().try_into()?)
        }

        Ok(buffer.into())
    }

    /// Use `MongoDB` full-text search
    ///
    /// Search by `query`, a space delimited string.
    /// All of them are `Option`s, but filtering is done to only search by fields
    /// that actually contain a value.
    #[tracing::instrument(skip(self))]
    pub async fn search_customers(
        &self,
        _query: String
    ) -> Result<DeliveryCustomerList, AppError> {
        todo!()
    }
}

/// Initialize the [`Database`] with a MongoDB [`MongoClient`]
#[tracing::instrument]
pub async fn setup_database() -> Result<Arc<Database>, AppError> {
    let mongo_url = env::var("MONGO_URL").unwrap_or_else(|_| {
        tracing::info!("MONGO_URL not set, using default");
        "mongodb://127.0.0.1:27017".into()
    });

    let mut options = ClientOptions::parse(mongo_url).await?;

    let mongo_timeout_duration_default = 3;
    match env::var("MONGO_TIMEOUT_DURATION") {
        Ok(duration) => {
            match u64::from_str_radix(&duration, 10) {
                Ok(number) => {
                    options.connect_timeout = Some(Duration::from_secs(number));
                }
                Err(error) => {
                    tracing::info!("Failed to parse MONGO_TIMEOUT_DURATION={duration} into integer: {error}. Using default");
                    options.connect_timeout = Some(Duration::from_secs(
                        mongo_timeout_duration_default
                    ));
                }
            };
        }

        Err(_) => {
            options.connect_timeout =
                Some(Duration::from_secs(mongo_timeout_duration_default));

            tracing::info!("MONGO_TIMEOUT_DURATION not set, using default");
        }
    };

    options.direct_connection = Some(true);

    tracing::info!("MongoDB connection type: {:?}", options.direct_connection);

    let mongodb_client = MongoClient::with_options(options)?;
    Ok(Arc::new(Database::new(mongodb_client)))
}
