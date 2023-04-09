use mongodb::bson::{Bson, Document};

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

    /// Convert [`Self`] into a MongoDB [`Document`]
    pub fn into_document(self) -> Document {
        let convert = |key: String, value: ApplianceField| {
            (
                key,
                match value {
                    ApplianceField::String(string) => Bson::String(string),
                    ApplianceField::DateTime(dt) => Bson::DateTime(dt.into())
                }
            )
        };
        Document::from_iter(
            self.into_iter().map(|(key, value)| convert(key, value))
        )
    }
}

pub enum ApplianceField {
    String(String),
    DateTime(chrono::DateTime<chrono::FixedOffset>)
}

impl From<String> for ApplianceField {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<chrono::DateTime<chrono::FixedOffset>> for ApplianceField {
    fn from(value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        Self::DateTime(value)
    }
}

impl IntoIterator for Appliance {
    type Item = (String, ApplianceField);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("manufacturer".into(), self.manufacturer.into()),
            (
                "year_of_manufacture".into(),
                self.year_of_manufacture.into()
            ),
            ("model".into(), self.model.into()),
            ("type".into(), self.r#type.into()),
            ("warranty".into(), self.warranty.into()),
            (
                "operation_performed".into(),
                self.operation_performed.to_string().into()
            ),
            ("number".into(), self.number.into()),
            ("date".into(), self.date.into()),
            ("expiration_date".into(), self.expiration_date.into()),
            (
                "observations".into(),
                self.observations.unwrap_or_default().into()
            ),
        ]
        .into_iter()
    }
}
