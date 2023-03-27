use super::OperationPerformed;
use edgedb_derive::Queryable;

pub type FixedOffsetDateTime = chrono::DateTime<chrono::FixedOffset>;

/// Represents some kind of [`Appliance`]
///
/// We don't know or care about the appliance (but they're mostly water heaters).
/// They have some operation performed on them for the [`DeliveryCustomer`], which we know from
/// the [`OperationPerformed`] field.
#[allow(dead_code)]
#[derive(Queryable, serde::Serialize, serde::Deserialize)]
#[edgedb(json)]
pub struct Appliance {
    manufacturer: String,
    year_of_manufacture: String,
    model: String,
    r#type: String,
    warranty: String,
    operation_performed: OperationPerformed,
    number: String,
    date: String,
    expiry_date: String,
    observations: Option<String>,
    last_updated: FixedOffsetDateTime
}

impl Appliance {
    /// Creates a new [`Appliance`].
    pub fn new(
        manufacturer: String,
        year_of_manufacture: String,
        model: String,
        r#type: String,
        warranty: String,
        operation_performed: OperationPerformed,
        number: String,
        date: String,
        expiry_date: String,
        observations: Option<String>,
        last_updated: FixedOffsetDateTime
    ) -> Self {
        Self {
            manufacturer,
            year_of_manufacture,
            model,
            r#type,
            warranty,
            operation_performed,
            number,
            date,
            expiry_date,
            observations,
            last_updated
        }
    }
}
