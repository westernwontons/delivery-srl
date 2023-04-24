use crate::auth::jwt::{
    generate_refresh_token, tokens_are_equal, AuthBody, AuthBodyWithRefreshToken, AuthError,
    RefreshToken
};
use crate::auth::verify::generate_token;
use crate::error::AppError;
use crate::state::AppState;
use crate::user::JsonEncodedUser;
use axum::routing::post;
use axum::Json;
use axum::{extract::State, Router};
use chrono::Utc;

/// Login a [`User`]
#[axum_macros::debug_handler]
#[tracing::instrument(skip(state))]
async fn login(
    State(state): State<AppState>,
    user: JsonEncodedUser
) -> Result<AuthBodyWithRefreshToken, AppError> {
    match state.database().user().validate_user_password(&user).await? {
        true => {
            tracing::info!(username = &user.username, auth = "successful");
            let refresh_token = generate_refresh_token();
            if state.store().insert(refresh_token.id.clone(), refresh_token.clone()).is_some() {
                tracing::info!("Replacing old refresh token");
            };
            let auth_body = AuthBody::new_bearer(generate_token(&user.username));

            Ok(AuthBodyWithRefreshToken::new(auth_body, refresh_token))
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

////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Logout a [`User`]
#[axum_macros::debug_handler]
#[tracing::instrument(skip(state))]
#[allow(unused_variables)]
async fn logout(State(state): State<AppState>) {}

////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Validate a refresh token and issue a new acces_token if it is valid
#[axum_macros::debug_handler]
#[tracing::instrument(skip(state))]
async fn refresh_token(
    State(state): State<AppState>,
    Json(token): Json<RefreshToken>
) -> Result<AuthBody, AppError> {
    match state.store().get(&token.id) {
        Some(stored_refresh_token) => {
            let now = Utc::now().timestamp() as usize;
            let stored_refresh_token = stored_refresh_token.value();
            match tokens_are_equal(&token, stored_refresh_token) {
                true => stored_refresh_token.exp > now,
                false => return Err(AppError::AuthError(AuthError::InvalidToken))
            }
        }
        None => {
            tracing::info!(authentication = "failed", reason = "WrongCredentials");
            return Err(AppError::AuthError(AuthError::WrongCredentials));
        }
    };

    tracing::info!(authentication = "successful");

    Ok(AuthBody::new_bearer(generate_token(&token.id)))
}

////////////////////////////////////////////////////////////////////////////////////////////////////////

/// [`Router`] that is concerned about authentication
pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh_token))
}
