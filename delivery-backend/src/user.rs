use axum::extract::{rejection::JsonRejection, FromRequest};
use axum::http::HeaderValue;
use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
use serde_json::json;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Validate, serde::Serialize, serde::Deserialize)]
pub struct User {
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    pub username: String,
    pub password: String
}

/// An [`User`] that's encoded in JSON
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct JsonEncodedUser(pub User);

#[axum::async_trait]
impl<S, B> axum::extract::FromRequest<S, B> for JsonEncodedUser
where
    Json<User>: FromRequest<S, B, Rejection = JsonRejection>,
    B: Send + 'static,
    S: Send + Sync
{
    type Rejection = UserError;

    async fn from_request(req: axum::http::Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let request_id = req
            .headers()
            .get("x-request-id")
            .map(|r| r.to_owned())
            .unwrap_or(HeaderValue::from_static("NOTSET"));
        match Json::<User>::from_request(req, state).await {
            Ok(Json(user)) => {
                match user.validate() {
                    Ok(()) => Ok(()),
                    Err(err) => {
                        tracing::error!(
                            "Failed authentication attempt: username={} x-request-id={}",
                            user.username,
                            request_id.to_str().unwrap()
                        );
                        Err(UserError::from(err))
                    }
                }?;
                Ok(JsonEncodedUser(user))
            }
            Err(rejection) => Err(rejection.into())
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error(transparent)]
    InvalidFields(#[from] ValidationErrors),
    #[error(transparent)]
    JsonRejection(#[from] JsonRejection)
}

impl IntoResponse for UserError {
    fn into_response(self) -> axum::response::Response {
        match self {
            UserError::InvalidFields(errors) => {
                let errors_string = match serde_json::to_string(&errors) {
                    Ok(s) => s,
                    Err(err) => {
                        tracing::error!("failed to convert ValidationErrors to string: {err}");
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                };

                let json_value = match serde_json::from_str::<serde_json::Value>(&errors_string) {
                    Ok(v) => v,
                    Err(err) => {
                        tracing::error!("Failed to serialize ValidationErrors: {err}");
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                };

                (StatusCode::UNPROCESSABLE_ENTITY, Json(json_value)).into_response()
            }
            UserError::JsonRejection(rejection) => match rejection {
                JsonRejection::JsonDataError(error) => Err::<(), _>((
                    error.status(),
                    Json(json!({ "missing_field": error.body_text() }))
                )),
                other => Err::<(), _>((other.status(), Json(other.body_text().into())))
            }
            .into_response()
        }
    }
}
