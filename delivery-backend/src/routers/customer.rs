use crate::customer::DeliveryCustomerList;
use crate::query::ExpiredCustomersQuery;
use crate::query::PartialDeliveryCustomer;
use crate::responses::InsertOneResultResponse;
use crate::responses::{DeleteResultResponse, UpdateResultResponse};
use crate::state::AppState;
use crate::{customer::DeliveryCustomerIn, error::AppError};

use axum::extract::{Path, Query, State};
use axum::routing::{delete, patch};
use axum::routing::{get, post, put};
use axum::{Json, Router};

/// Add a new [`DeliveryCustomer`]
///
/// Adds a new [`DeliveryCustomer`] to the database.
#[tracing::instrument(skip(state))]
#[axum_macros::debug_handler]
async fn create_customer(
    State(state): State<AppState>,
    Json(customer): Json<DeliveryCustomerIn>
) -> Result<InsertOneResultResponse, AppError> {
    tracing::info!("Inserting customer with customer_id={}", &customer.customer_id);

    state.database().customer().insert_customer(customer).await
}

/// Edit a [`DeliveryCustomer`]
///
/// Edits an existing [`DeliveryCustomer`] in the database.
#[tracing::instrument(skip(state))]
#[axum_macros::debug_handler]
async fn update_customer(
    State(state): State<AppState>,
    Json(customer): Json<PartialDeliveryCustomer>
) -> Result<UpdateResultResponse, AppError> {
    tracing::info!("Updating customer with customer_id={}", &customer.customer_id);

    state.database().customer().update_customer(customer).await
}

/// Activate a [`DeliveryCustomer`]
///
/// Customers can become inactive when they stop being a customer
/// or for some other arbitrary reason.
/// It's a requirement to keep the data associated with them,
/// whether they're still an active customer or not.
#[tracing::instrument(skip(state))]
#[axum_macros::debug_handler]
async fn activate_customer(
    State(state): State<AppState>,
    Path(customer_id): Path<String>
) -> Result<UpdateResultResponse, AppError> {
    tracing::info!("Activating customer with customer_id={}", &customer_id);

    state.database().customer().activate_customer(customer_id).await
}

/// Deactivate a [`DeliveryCustomer`]
///
/// Clients can become inactive when they stop being a customer.
/// or for some other arbitrary reason.
/// It's a requirement to keep the data associated with them,
/// whether they're still an active customer or not.
#[tracing::instrument(skip(state))]
#[axum_macros::debug_handler]
async fn deactivate_customer(
    State(state): State<AppState>,
    Path(customer_id): Path<String>
) -> Result<UpdateResultResponse, AppError> {
    tracing::info!("Deactivating customer with customer_id={}", &customer_id);

    state.database().customer().deactivate_customer(customer_id).await
}

/// Delete a [`DeliveryCustomer`]
///
/// Removes a [`DeliveryCustomer`] with matching `customer_id` from MongoDb
#[tracing::instrument(skip(state))]
#[axum_macros::debug_handler]
async fn delete_customer(
    State(state): State<AppState>,
    Path(customer_id): Path<String>
) -> Result<DeleteResultResponse, AppError> {
    tracing::info!("Deleting customer with customer_id={}", &customer_id);

    state.database().customer().delete_customer(customer_id).await
}

/// Retrieve expired [`DeliveryCustomer`]s
///
/// Retrieve [`DeliveryCustomer`]s that are expired (their appliance is due for a checkup).
#[tracing::instrument(skip(state))]
#[axum_macros::debug_handler]
async fn expired_customers(
    State(state): State<AppState>,
    Query(query): Query<ExpiredCustomersQuery>
) -> Result<DeliveryCustomerList, AppError> {
    tracing::info!("Retrieving expired customers");

    state.database().customer().expired_customers(query).await
}

/// Router for client related operations.
///
/// Any action done on a client resource is registered here.
pub fn customer_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_customer))
        .route("/update", put(update_customer))
        .route("/activate/:customer_id", patch(activate_customer))
        .route("/deactivate/:customer_id", patch(deactivate_customer))
        .route("/delete/:customer_id", delete(delete_customer))
        .route("/expired", get(expired_customers))
}
