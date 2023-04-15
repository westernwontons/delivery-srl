use crate::{state::AppState, user::JsonEncodedUser};
use axum::{extract::State, routing::post, Router};

#[allow(unused_variables)]
/// Login a [`User`]
#[axum_macros::debug_handler]
pub async fn login(State(_state): State<AppState>, user: JsonEncodedUser) {}

/// Logout a [`User`]
#[axum_macros::debug_handler]
pub async fn logout() {}

pub fn auth_router() -> Router<AppState> {
    Router::new().route("/login", post(login)).route("/logout", post(logout))
}
