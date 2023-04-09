#![allow(unused_variables)]

use crate::state::AppState;
use crate::update::PartialCustomerUpdate;
use axum::{extract::State, Json};

/// Search for a [`DeliveryCustomer`] in the database.
///
/// Accepts a [`SearchQuery`] which contains all the possible fields to search for.
#[axum_macros::debug_handler]
pub async fn customer_search(
    State(state): State<AppState>,
    Json(search): Json<PartialCustomerUpdate>
) {
    todo!()
}
