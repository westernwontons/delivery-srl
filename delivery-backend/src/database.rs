use crate::customer::DeliveryCustomerIn;
use crate::customer::DeliveryCustomerList;
use crate::error::AppError;
use crate::query::ExpiredCustomersQuery;
use crate::query::PartialDeliveryCustomer;
use crate::query::SearchQuery;
use crate::responses::DeleteResultResponse;
use crate::responses::InsertOneResultResponse;
use crate::responses::UpdateResultResponse;
use mongodb::bson::{doc, Document};
use mongodb::options::ClientOptions;
use mongodb::options::FindOptions;
use mongodb::Database as MongoDatabase;
use mongodb::{Client as MongoClient, Cursor};
use mongodb::{Collection as MongoCollection, IndexModel};
use std::env;
use std::sync::Arc;
use std::time::Duration;

/// Represents the connection to the database
///
/// Uses a [`MongoClient`] internally
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
    fn customer_collection(&self) -> MongoCollection<Document> {
        self.get_database().collection("customer")
    }

    /// Commit a [`DeliveryCustomerIn`] to the database
    ///
    /// Note that there will be no checks made whether the customer already exists.
    /// The [`DeliveryCustomerIn`] will be inserted as is.
    #[tracing::instrument(skip(self))]
    pub async fn insert_customer(
        &self,
        customer: DeliveryCustomerIn
    ) -> Result<InsertOneResultResponse, AppError> {
        Ok(self
            .customer_collection()
            .insert_one(customer.into_update_document(), None)
            .await?
            .into())
    }

    /// Update a [`DeliveryCustomer`] in the database
    ///
    /// Updates a document in the `customer` collection, based on a matching `customer_id`.
    /// The [`PartialDeliveryCustomer`] is purged of all [`None`] fields, because we don't want
    /// to overwrite any of the existing values.
    #[tracing::instrument(skip(self))]
    pub async fn update_customer(
        &self,
        customer: PartialDeliveryCustomer
    ) -> Result<UpdateResultResponse, AppError> {
        let update_result_response = self
            .customer_collection()
            .update_one(
                doc! { "customer_id": &customer.customer_id },
                customer.into_update_document_no_none(),
                None
            )
            .await?
            .into();

        Ok(update_result_response)
    }

    /// Activate a [`DeliveryCustomer`]
    #[tracing::instrument(skip(self))]
    pub async fn activate_customer(
        &self,
        customer_id: String
    ) -> Result<UpdateResultResponse, AppError> {
        let update_result_response = self
            .customer_collection()
            .update_one(
                doc! { "customer_id": customer_id },
                doc! { "$set": { "active": true } },
                None
            )
            .await?
            .into();

        Ok(update_result_response)
    }

    /// Deactivate a [`DeliveryCustomer`]
    #[tracing::instrument(skip(self))]
    pub async fn deactivate_customer(
        &self,
        customer_id: String
    ) -> Result<UpdateResultResponse, AppError> {
        let update_result_response = self
            .customer_collection()
            .update_one(
                doc! { "customer_id": customer_id },
                doc! { "$set": { "active": false } },
                None
            )
            .await?
            .into();

        Ok(update_result_response)
    }

    /// Delete a [`DeliveryCustomer`]
    #[tracing::instrument(skip(self))]
    pub async fn delete_customer(
        &self,
        customer_id: String
    ) -> Result<DeleteResultResponse, AppError> {
        let delete_result_response = self
            .customer_collection()
            .delete_one(doc! {"customer_id": customer_id}, None)
            .await?
            .into();

        Ok(delete_result_response)
    }

    /// Fetch expired customers
    ///
    /// An [`ExpiredCustomersQuery`] contains the possible query parameters.
    #[tracing::instrument(skip(self))]
    pub async fn expired_customers(
        &self,
        time_range: ExpiredCustomersQuery
    ) -> Result<DeliveryCustomerList, AppError> {
        let cursor = self
            .customer_collection()
            .aggregate(time_range.as_aggregation(), None)
            .await?;

        let customer_list = try_customer_list(cursor).await;

        match customer_list {
            Ok(customer_list) => {
                tracing::info!(
                    "Found {} expired customers",
                    &customer_list.len()
                );
                Ok(customer_list)
            }
            Err(err) => {
                tracing::error!("{err}");
                Err(err)
            }
        }
    }

    /// Use `MongoDB` full-text search
    ///
    /// Search by `query`, a space delimited string.
    /// All of them are `Option`s, but filtering is done to only search by fields
    /// that actually contain a value.
    #[tracing::instrument(skip(self))]
    pub async fn search_customers(
        &self,
        search: SearchQuery
    ) -> Result<DeliveryCustomerList, AppError> {
        let cursor = self
            .customer_collection()
            .find(
                doc! { "$text": { "$search": &search.query } },
                FindOptions::builder()
                    .sort(doc! { "score": { "$meta": "textScore" } })
                    .build()
            )
            .await?;

        let customer_list = try_customer_list(cursor).await;

        match customer_list {
            Ok(customer_list) => {
                tracing::info!(
                    "Found {} customers. Query used: {}",
                    customer_list.len(),
                    &search
                );
                Ok(customer_list)
            }
            Err(err) => {
                tracing::error!("{err}");
                Err(err)
            }
        }
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
            match duration.parse::<u64>() {
                Ok(parsed_duration) => {
                    options.connect_timeout =
                        Some(Duration::from_secs(parsed_duration));
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

    tracing::info!("Setting up indexes");

    mongodb_client
        .database("delivery_database")
        .collection::<Document>("customer")
        .create_index(
            IndexModel::builder().keys(doc! { "$**": "text" }).build(),
            None
        )
        .await?;

    tracing::info!("Index setup complete");

    Ok(Arc::new(Database::new(mongodb_client)))
}

/// Try to drive the cursor to yield `Document`s
pub async fn try_customer_list(
    mut cursor: Cursor<Document>
) -> Result<DeliveryCustomerList, AppError> {
    let mut buffer = Vec::with_capacity(10);

    while cursor.advance().await? {
        let deserialized = cursor.deserialize_current().try_into()?;
        buffer.push(deserialized)
    }

    Ok(buffer.into())
}
