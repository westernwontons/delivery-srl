use crate::error::AppError;
use mongodb::bson::{doc, Document};
use mongodb::options::ClientOptions;
use mongodb::Client as MongoClient;
use mongodb::IndexModel;
use std::env;
use std::sync::Arc;
use std::time::Duration;

use super::collection::{CustomerCollection, UserCollection};

/// Represents the connection to the database
///
/// Uses a [`MongoClient`] internally
#[derive(Debug)]
pub struct Database {
    client: Arc<MongoClient>
}

impl Database {
    /// Creates a new [`Database`].
    pub fn new(client: MongoClient) -> Self {
        Self { client: Arc::new(client) }
    }

    /// Return a [`CustomerCollection`] that allows operations to be
    /// done on the `customer` MongoDb collection
    pub fn customer(&self) -> CustomerCollection {
        CustomerCollection::new(Arc::clone(&self.client))
    }
    /// Return a [`UserCollection`] that allows operations to be
    /// done on the `user` MongoDb collection
    pub fn user(&self) -> UserCollection {
        UserCollection::new(Arc::clone(&self.client))
    }
}

//////////////////////////////////////////////////////////////////////////////////////////

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
                    options.connect_timeout = Some(Duration::from_secs(parsed_duration));
                }
                Err(error) => {
                    tracing::info!("Failed to parse MONGO_TIMEOUT_DURATION={duration} into integer: {error}. Using default");
                    options.connect_timeout =
                        Some(Duration::from_secs(mongo_timeout_duration_default));
                }
            };
        }

        Err(_) => {
            options.connect_timeout = Some(Duration::from_secs(mongo_timeout_duration_default));

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
        .create_index(IndexModel::builder().keys(doc! { "$**": "text" }).build(), None)
        .await?;

    tracing::info!("Index setup complete");

    Ok(Arc::new(Database::new(mongodb_client)))
}
