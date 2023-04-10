use axum::{http::StatusCode, response::IntoResponse, Json};
use mongodb::bson::de::Error as BsonDeError;
use mongodb::error::Error as MongoError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("DatabaseError: {0}")]
    DatabaseError(#[from] MongoError),
    #[error("BsonDeError: {0}")]
    BsonError(#[from] BsonDeError)
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::DatabaseError(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "DatabaseError": error.to_string() }))
            )
                .into_response(),

            AppError::BsonError(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"BsonDeError": error.to_string()}))
            )
                .into_response()
        }
    }
}
