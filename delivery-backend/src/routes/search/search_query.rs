use crate::customer::{
    CustomerStatus, FixedOffsetDateTime, OperationPerformed
};

/// Represents a request for searching [`DeliveryCustomer`]s
///
/// It's fields are analogous to a flattened [`DeliveryCustomer`],
/// except that all fields are optional.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SearchQuery {
    document_id: Option<String>,
    name: Option<String>,
    status: Option<CustomerStatus>,
    county: Option<String>,
    street: Option<String>,
    number: Option<String>,
    additional: Option<String>,
    manufacturer: Option<String>,
    year_of_manufacture: Option<String>,
    model: Option<String>,
    r#type: Option<String>,
    warranty: Option<String>,
    operation_performed: Option<OperationPerformed>,
    appliance_number: Option<String>,
    date: Option<String>,
    expiry_date: Option<String>,
    observations: Option<String>,
    last_updated: Option<FixedOffsetDateTime>
}
