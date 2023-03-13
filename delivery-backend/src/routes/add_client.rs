use axum::{
    routing::{get, post},
    Router
};

/// Add a new client
///
/// Adds a new client to the database.
pub async fn add_client() {
    todo!()
}

/// Edit a client
///
/// Edits an existing client in the database.
pub async fn edit_client() {
    todo!()
}

/// Activate a client
///
/// Clients can become inactive when they stop being a customer.
/// The requirement is to keep the data associated with them,
/// whether they're still an active client or not.
pub async fn activate_client() {
    todo!()
}

/// Deactivate a client
///
/// Clients can become inactive when they stop being a customer.
/// The requirement is to keep the data associated with them,
/// whether they're still an active client or not.
pub async fn deactivate_client() {
    todo!()
}

/// Retrieve expired clients
///
/// Clients that are expired, as in their boiler or whatever is due for a checkup
/// are retrieved using this endpoint.
pub async fn expired_clients() {
    todo!()
}

/// Router for client related operations.
///
/// Any action done on a client resource is registered here.
pub fn client_router() -> Router {
    Router::new()
        .route("/add", post(add_client))
        .route("/edit", post(edit_client))
        .route("/activate", post(activate_client))
        .route("/deactivate", post(deactivate_client))
        .route("/expired", get(expired_clients))
}
