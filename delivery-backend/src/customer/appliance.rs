use mongodb::bson::{self, Bson, Document};
use serde::{Deserialize, Deserializer};

use crate::appliance_field::ApplianceField;

use super::OperationPerformed;

/// Represents some kind of [`ApplianceIn`]
///
/// We don't know or care about the appliance (but they're mostly water heaters).
/// They have some operation performed on them for the [`DeliveryCustomerIn`], which we know from
/// the [`OperationPerformed`] field.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApplianceIn {
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

impl ApplianceIn {
    /// Convert [`Self`] into a MongoDB [`Document`]
    pub fn into_document(self) -> Document {
        let convert = |key: String, value: ApplianceField| {
            (
                key,
                match value {
                    ApplianceField::String(string) => Bson::String(string),
                    ApplianceField::DateTime(dt) => Bson::DateTime(dt.into()),
                    _ => unimplemented!()
                }
            )
        };
        Document::from_iter(
            self.into_iter().map(|(key, value)| convert(key, value))
        )
    }
}

impl IntoIterator for ApplianceIn {
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

/// Represents some kind of `Appliance`
///
/// We don't know or care about the appliance (but they're mostly water heaters).
/// They have some operation performed on them for the [`DeliveryCustomerIn`], which we know from
/// the [`OperationPerformed`] field.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApplianceOut {
    pub manufacturer: String,
    pub year_of_manufacture: String,
    pub model: String,
    pub r#type: String,
    #[serde(deserialize_with = "deserialize_chrono_from_bson_datetime")]
    pub warranty: chrono::DateTime<chrono::Utc>,
    pub operation_performed: OperationPerformed,
    pub number: String,
    #[serde(deserialize_with = "deserialize_chrono_from_bson_datetime")]
    pub date: chrono::DateTime<chrono::Utc>,
    #[serde(deserialize_with = "deserialize_chrono_from_bson_datetime")]
    pub expiration_date: chrono::DateTime<chrono::Utc>,
    pub observations: Option<String>
}

/// Deserializes a [`chrono::DateTime`] from a [`mongodb::bson::DateTime`].
pub fn deserialize_chrono_from_bson_datetime<'de, D>(
    deserializer: D
) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
where
    D: Deserializer<'de>
{
    let datetime = bson::DateTime::deserialize(deserializer)?;
    Ok(datetime.to_chrono())
}
