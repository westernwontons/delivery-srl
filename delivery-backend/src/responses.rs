use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
use mongodb::bson::oid::ObjectId;
use mongodb::results::DeleteResult;
use mongodb::results::{InsertOneResult, UpdateResult};

/// The result of inserting a single document to MongoDb
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InsertOneResultResponse {
    inserted_id: ObjectId
}

impl IntoResponse for InsertOneResultResponse {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::CREATED,
            Json(serde_json::json!({ "inserted_id": self.inserted_id.to_string() }))
        )
            .into_response()
    }
}

impl From<InsertOneResult> for InsertOneResultResponse {
    fn from(value: InsertOneResult) -> Self {
        Self {
            inserted_id: value.inserted_id.as_object_id().unwrap()
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////

/// The result of updating a single document in MongoDb
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateResultResponse {
    matched_count: u64,
    modified_count: u64,
    upserted_id: Option<ObjectId>
}

impl IntoResponse for UpdateResultResponse {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::OK,
            Json(serde_json::json!({
                "matched_count": self.matched_count,
                "modified_count": self.modified_count,
                "upserted_id": self.upserted_id,
            }))
        )
            .into_response()
    }
}

impl From<UpdateResult> for UpdateResultResponse {
    fn from(value: UpdateResult) -> Self {
        Self {
            upserted_id: value.upserted_id.and_then(|i| i.as_object_id()),
            matched_count: value.matched_count,
            modified_count: value.matched_count
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////

/// The result of deleting a single document in MongoDb
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteResultResponse {
    deleted_count: u64
}

impl IntoResponse for DeleteResultResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl From<DeleteResult> for DeleteResultResponse {
    fn from(value: DeleteResult) -> Self {
        Self {
            deleted_count: value.deleted_count
        }
    }
}
