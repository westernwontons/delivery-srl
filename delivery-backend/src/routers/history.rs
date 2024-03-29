use crate::state::AppState;
use axum::extract::State;

/// Recent [`DeliveryCustomer`] history
///
/// Get the recently added, edited, activated or deactivated [`DeliveryCustomer`]s.
#[axum_macros::debug_handler]
#[allow(unused_variables)]
pub async fn customer_history(State(state): State<AppState>) {
    todo!()
}
