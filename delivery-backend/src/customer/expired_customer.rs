use axum::{http::StatusCode, response::IntoResponse, Json};

use super::delivery_customer::DeliveryCustomerOut;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct DeliveryCustomerList(Vec<DeliveryCustomerOut>);

impl DeliveryCustomerList {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl IntoResponse for DeliveryCustomerList {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl From<Vec<DeliveryCustomerOut>> for DeliveryCustomerList {
    fn from(value: Vec<DeliveryCustomerOut>) -> Self {
        Self(value)
    }
}
