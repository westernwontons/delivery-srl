use axum::{http::StatusCode, response::IntoResponse, Json};

use super::delivery_customer::DeliveryCustomerOut;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct DeliveryCustomerList(Vec<DeliveryCustomerOut>);

impl DeliveryCustomerList {
    /// Return the number of [`DeliveryCustomerOut`]s held
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the underlying vector contains no elements
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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
