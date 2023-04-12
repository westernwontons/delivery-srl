use crate::{
    customer::DeliveryCustomerList, error::AppError, query::SearchQuery,
    state::AppState
};
use axum::extract::{Query, State};

/// Search for a `DeliveryCustomer` in the database.
///
/// Accepts a [`String`] which contains all the possible fields to search for.
#[tracing::instrument(skip(state))]
#[axum_macros::debug_handler]
pub async fn customer_search(
    State(state): State<AppState>,
    Query(search): Query<SearchQuery>
) -> Result<DeliveryCustomerList, AppError> {
    state.database().search_customers(search).await
}
