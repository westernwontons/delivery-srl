use chrono::{DateTime, Utc};
use mongodb::bson::{self, Document};
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
    #[serde(rename = "type")]
    pub typ: String,
    pub warranty: DateTime<chrono::FixedOffset>,
    pub operation_performed: OperationPerformed,
    pub number: String,
    pub date: DateTime<chrono::FixedOffset>,
    pub expiration_date: DateTime<chrono::FixedOffset>,
    pub observations: Option<String>
}

impl ApplianceIn {
    /// Convert [`Self`] into a MongoDB [`Document`]
    pub fn into_document(self) -> Document {
        Document::from_iter(
            self.into_iter().map(|(key, value)| (key, value.into()))
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
            ("type".into(), self.typ.into()),
            ("warranty".into(), self.warranty.into()),
            (
                "operation_performed".into(),
                self.operation_performed.into()
            ),
            ("number".into(), self.number.into()),
            ("date".into(), self.date.into()),
            ("expiration_date".into(), self.expiration_date.into()),
            ("observations".into(), self.observations.into()),
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
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(deserialize_with = "deserialize_chrono_from_bson_datetime")]
    pub warranty: DateTime<Utc>,
    pub operation_performed: OperationPerformed,
    pub number: String,
    #[serde(deserialize_with = "deserialize_chrono_from_bson_datetime")]
    pub date: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_chrono_from_bson_datetime")]
    pub expiration_date: DateTime<Utc>,
    pub observations: Option<String>
}

/// Deserializes a [`chrono::DateTime`] from a [`mongodb::bson::DateTime`].
pub fn deserialize_chrono_from_bson_datetime<'de, D>(
    deserializer: D
) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>
{
    let datetime = bson::DateTime::deserialize(deserializer)?;
    Ok(datetime.to_chrono())
}
