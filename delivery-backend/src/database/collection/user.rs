use crate::{auth::password::verify_password, error::AppError, user::JsonEncodedUser};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client as MongoClient, Collection as MongoCollection
};
use std::sync::Arc;

/// The `user` that goes OUT
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DeliveryUserOut {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub password: String
}

//////////////////////////////////////////////////////////////////////////////////////////

/// The [`UserCollection`] holds a reference to the MongoClient
/// and does operations on the `user` collection
#[derive(Debug)]
pub struct UserCollection {
    client: Arc<MongoClient>
}

impl UserCollection {
    /// Creates a new [`UserCollection`].
    pub fn new(client: Arc<MongoClient>) -> Self {
        Self { client }
    }

    /// Get the `user` collection
    #[tracing::instrument(skip(self))]
    fn user_collection(&self) -> MongoCollection<DeliveryUserOut> {
        Arc::clone(&self.client).database("delivery_database").collection("user")
    }

    /// Fetch a user by it's username
    #[tracing::instrument(skip(self))]
    pub async fn get_user(&self, username: &str) -> Result<Option<DeliveryUserOut>, AppError> {
        let maybe_user =
            self.user_collection().find_one(doc! { "username": username }, None).await?;

        Ok(maybe_user)
    }

    /// Checks whether the user trying to log in has typed their password in correctly
    #[tracing::instrument]
    pub async fn validate_user_password(&self, user: &JsonEncodedUser) -> Result<bool, AppError> {
        let maybe_valid = self.get_user(&user.username).await?.is_some_and(|delivery_user_out| {
            match verify_password(&user.password, &delivery_user_out.password) {
                Ok(maybe_valid) => maybe_valid,
                Err(err) => {
                    tracing::error!("{}", err);
                    false
                }
            }
        });

        Ok(maybe_valid)
    }
}
