use super::OperationPerformed;

/// Represents some kind of [`Appliance`]
///
/// We don't know or care about the appliance (but they're mostly water heaters).
/// They have some operation performed on them for the [`DeliveryCustomer`], which we know from
/// the [`OperationPerformed`] field.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Appliance {
    pub manufacturer: String,
    pub year_of_manufacture: String,
    pub model: String,
    pub r#type: String,
    pub warranty: chrono::DateTime<chrono::FixedOffset>,
    pub operation_performed: OperationPerformed,
    pub number: String,
    pub date: chrono::DateTime<chrono::FixedOffset>,
    pub expiration_date: chrono::DateTime<chrono::FixedOffset>,
    pub observations: Option<String>
}

impl Appliance {
    /// Creates a new [`Appliance`].
    pub fn new(
        manufacturer: String,
        year_of_manufacture: String,
        model: String,
        r#type: String,
        warranty: chrono::DateTime<chrono::FixedOffset>,
        operation_performed: OperationPerformed,
        number: String,
        date: chrono::DateTime<chrono::FixedOffset>,
        expiration_date: chrono::DateTime<chrono::FixedOffset>,
        observations: Option<String>
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
            expiration_date,
            observations
        }
    }
}
