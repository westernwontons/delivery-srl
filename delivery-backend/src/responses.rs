use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
use mongodb::bson::Bson;
use mongodb::results::DeleteResult;
use mongodb::results::{InsertOneResult, UpdateResult};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InsertOneResultResponse {
    inserted_id: Bson
}

impl IntoResponse for InsertOneResultResponse {
    fn into_response(self) -> axum::response::Response {
        let inserted_id = match self.inserted_id {
            Bson::ObjectId(id) => {
                Json(serde_json::json!({ "inserted_id": id.to_string() }))
            }
            _ => unreachable!()
        };
        (StatusCode::CREATED, inserted_id).into_response()
    }
}

impl From<InsertOneResult> for InsertOneResultResponse {
    fn from(value: InsertOneResult) -> Self {
        Self {
            inserted_id: value.inserted_id
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateResultResponse {
    matched_count: u64,
    modified_count: u64,
    upserted_id: Option<Bson>
}

impl IntoResponse for UpdateResultResponse {
    fn into_response(self) -> axum::response::Response {
        let upserted_id = if let Some(Bson::ObjectId(oid)) = self.upserted_id {
            Some(oid.to_string())
        } else {
            None
        };
        (
            StatusCode::OK,
            Json(serde_json::json!({
                "matched_count": self.matched_count,
                "modified_count": self.modified_count,
                "upserted_id": upserted_id,
            }))
        )
            .into_response()
    }
}

impl From<UpdateResult> for UpdateResultResponse {
    fn from(value: UpdateResult) -> Self {
        Self {
            upserted_id: value.upserted_id,
            matched_count: value.matched_count,
            modified_count: value.matched_count
        }
    }
}

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
