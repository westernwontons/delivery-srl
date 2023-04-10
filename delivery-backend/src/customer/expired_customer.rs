use axum::{http::StatusCode, response::IntoResponse, Json};

use super::delivery_customer::DeliveryCustomerOut;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExpiredCustomerList(Vec<DeliveryCustomerOut>);

impl IntoResponse for ExpiredCustomerList {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl From<Vec<DeliveryCustomerOut>> for ExpiredCustomerList {
    fn from(value: Vec<DeliveryCustomerOut>) -> Self {
        Self(value)
    }
}
