use crate::appliance_field::ApplianceField;
use crate::customer::{CustomerStatus, OperationPerformed};
use mongodb::bson::{doc, Document};

/// Represents a request for searching or update [`DeliveryCustomer`]s
///
/// It's fields are analogous to a flattened [`DeliveryCustomer`],
/// except that all fields are optional.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PartialDeliveryCustomer {
    pub customer_id: String,
    pub name: Option<String>,
    pub status: Option<CustomerStatus>,
    pub county: Option<String>,
    pub street: Option<String>,
    pub number: Option<String>,
    pub additional: Option<String>,
    pub manufacturer: Option<String>,
    pub year_of_manufacture: Option<u16>,
    pub model: Option<String>,
    #[serde(rename = "type")]
    pub typ: Option<String>,
    pub warranty: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub operation_performed: Option<OperationPerformed>,
    pub appliance_number: Option<String>,
    pub date: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub expiration_date: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub observations: Option<String>
}

impl PartialDeliveryCustomer {
    /// Converts a [`PartialDeliveryCustomer`] into a MongoDB [`Document`]
    ///
    /// Filters out all fields that are [`None`].
    pub fn into_update_document_no_none(self) -> Document {
        let all_some = self.into_iter().filter(|(_, value)| value.is_some());
        let mut inner_document = Document::default();
        let mut address_document = Document::default();
        let mut appliance_document = Document::default();
        let mut document = Document::default();

        for (key, value) in all_some {
            match key.as_str() {
                "county" | "street" | "number" | "additional" => {
                    address_document.insert(key, value);
                }
                "manufacturer"
                | "year_of_manufacture"
                | "model"
                | "typ"
                | "warranty"
                | "operation_performed"
                | "appliance_number"
                | "date"
                | "expiration_date"
                | "observations" => {
                    appliance_document.insert(key, value);
                }
                _ => {
                    inner_document.insert(key, value);
                }
            }
        }

        inner_document.insert("address", address_document);
        inner_document.insert("appliance", appliance_document);
        document.insert("$set", inner_document);

        document
    }
}

impl IntoIterator for PartialDeliveryCustomer {
    type Item = (String, Option<ApplianceField>);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("customer_id".into(), Some(self.customer_id.into())),
            ("name".into(), self.name.map(ApplianceField::from)),
            ("status".into(), self.status.map(ApplianceField::from)),
            ("county".into(), self.county.map(ApplianceField::from)),
            ("street".into(), self.street.map(ApplianceField::from)),
            ("number".into(), self.number.map(ApplianceField::from)),
            (
                "additional".into(),
                self.additional.map(ApplianceField::from)
            ),
            (
                "manufacturer".into(),
                self.manufacturer.map(ApplianceField::from)
            ),
            (
                "year_of_manufacture".into(),
                self.year_of_manufacture.map(ApplianceField::from)
            ),
            ("model".into(), self.model.map(ApplianceField::from)),
            ("type".into(), self.typ.map(ApplianceField::from)),
            ("warranty".into(), self.warranty.map(ApplianceField::from)),
            (
                "operation_performed".into(),
                self.operation_performed.map(ApplianceField::from)
            ),
            (
                "appliance_number".into(),
                self.appliance_number.map(ApplianceField::from)
            ),
            ("date".into(), self.date.map(ApplianceField::from)),
            (
                "expiration_date".into(),
                self.expiration_date.map(ApplianceField::from)
            ),
            (
                "observations".into(),
                self.observations.map(ApplianceField::from)
            ),
        ]
        .into_iter()
    }
}
