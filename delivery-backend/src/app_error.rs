use axum::{http::StatusCode, response::IntoResponse, Json};
use mongodb::error::Error as MongoError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("DatabaseError")]
    DatabaseError(#[from] MongoError)
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::DatabaseError(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "DatabaseError": error.to_string() }))
            )
                .into_response()
        }
    }
}
