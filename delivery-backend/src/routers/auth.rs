use crate::{state::AppState, user::JsonEncodedUser};
use axum::{extract::State, routing::post, Router};

/// Login a [`User`]
pub async fn login(State(state): State<AppState>, user: JsonEncodedUser) {}

/// Logout a [`User`]
pub async fn logout() {}

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
}
