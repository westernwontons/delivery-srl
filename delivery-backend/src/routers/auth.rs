use crate::{
    auth::{
        jwt::{AuthBody, AuthError},
        token::generate_token
    },
    error::AppError,
    state::AppState,
    user::JsonEncodedUser
};
use axum::{extract::State, routing::post, Router};

/// Login a [`User`]
#[allow(unused_variables)]
#[axum_macros::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    user: JsonEncodedUser
) -> Result<AuthBody, AppError> {
    match state.database().user().validate_user_password(&user).await? {
        true => {
            tracing::info!(username = &user.username, auth = "successful");
            Ok(AuthBody::new_bearer(generate_token(&user.username)))
        }
        false => {
            tracing::error!(
                username = &user.username,
                auth = "unsuccessful",
                reason = "WrongCredentials"
            );
            Err(AppError::AuthError(AuthError::WrongCredentials))
        }
    }
}

/// Logout a [`User`]
#[axum_macros::debug_handler]
pub async fn logout() {}

/// [`Router`] that is concerned about authentication
pub fn auth_router() -> Router<AppState> {
    Router::new().route("/login", post(login)).route("/logout", post(logout))
}
