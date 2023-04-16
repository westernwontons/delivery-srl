use std::sync::Arc;

use mongodb::{
    bson::{doc, Document},
    options::FindOptions,
    Client as MongoClient, Collection as MongoCollection, Database as MongoDatabase
};

use crate::customer::{DeliveryCustomerIn, DeliveryCustomerList};
use crate::database::customer_list::try_customer_list;
use crate::error::AppError;
use crate::query::{ExpiredCustomersQuery, PartialDeliveryCustomer, SearchQuery};
use crate::responses::{DeleteResultResponse, InsertOneResultResponse, UpdateResultResponse};

pub struct CustomerCollection {
    client: Arc<MongoClient>
}

impl CustomerCollection {
    /// Creates a new [`CustomerCollection`].
    pub fn new(client: Arc<MongoClient>) -> Self {
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
        let cursor =
            self.customer_collection().aggregate(time_range.as_aggregation(), None).await?;

        let customer_list = try_customer_list(cursor).await;

        match customer_list {
            Ok(customer_list) => {
                tracing::info!("Found {} expired customers", &customer_list.len());
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
                FindOptions::builder().sort(doc! { "score": { "$meta": "textScore" } }).build()
            )
            .await?;

        let customer_list = try_customer_list(cursor).await;

        match customer_list {
            Ok(customer_list) => {
                tracing::info!("Found {} customers. Query used: {}", customer_list.len(), &search);
                Ok(customer_list)
            }
            Err(err) => {
                tracing::error!("{err}");
                Err(err)
            }
        }
    }
}
