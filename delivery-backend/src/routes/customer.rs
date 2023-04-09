#![allow(
    unused_variables,
    unused_mut,
    unused_imports,
    dead_code,
    unused_assignments
)]

use crate::expiration::TimeRange;
use crate::state::AppState;
use crate::{app_error::AppError, customer::DeliveryCustomer};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router
};

/// Add a new [`DeliveryCustomer`]
///
/// Adds a new [`DeliveryCustomer`] to the database.
#[axum_macros::debug_handler]
async fn create_customer(
    State(state): State<AppState>,
    Json(customer): Json<DeliveryCustomer>
) -> Result<(), AppError> {
    todo!()
}

/// Edit a [`DeliveryCustomer`]
///
/// Edits an existing [`DeliveryCustomer`] in the database.
#[axum_macros::debug_handler]
async fn update_customer(
    State(state): State<AppState>,
    Json(customer): Json<DeliveryCustomer>
) {
    todo!()
}

/// Activate a [`DeliveryCustomer`]
///
/// Customers can become inactive when they stop being a customer
/// or for some other arbitrary reason.
/// It's a requirement to keep the data associated with them,
/// whether they're still an active customer or not.
#[axum_macros::debug_handler]
async fn activate_customer(
    State(state): State<AppState>,
    Path(customer_id): Path<String>
) {
    todo!()
}

/// Deactivate a [`DeliveryCustomer`]
///
/// Clients can become inactive when they stop being a customer.
/// or for some other arbitrary reason.
/// It's a requirement to keep the data associated with them,
/// whether they're still an active customer or not.
#[axum_macros::debug_handler]
async fn deactivate_customer(
    State(state): State<AppState>,
    Path(customer_id): Path<String>
) {
    todo!()
}

/// Retrieve expired [`DeliveryCustomer`]s
///
/// Retrieve [`DeliveryCustomer`]s that are expired (their appliance is due for a checkup).
#[axum_macros::debug_handler]
async fn expired_customers(
    State(state): State<AppState>,
    Query(expiration_range): Query<TimeRange>
) {
    todo!()
}

/// Router for client related operations.
///
/// Any action done on a client resource is registered here.
pub fn customer_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_customer))
        .route("/update", put(update_customer))
        .route("/activate/:customer_id", get(activate_customer))
        .route("/deactivate/:customer_id", get(deactivate_customer))
        .route("/expired", get(expired_customers))
}
