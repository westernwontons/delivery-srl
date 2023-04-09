use mongodb::bson::{doc, Document};

use crate::customer::{CustomerStatus, OperationPerformed};

/// Represents a request for searching or update [`DeliveryCustomer`]s
///
/// It's fields are analogous to a flattened [`DeliveryCustomer`],
/// except that all fields are optional.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PartialDeliveryCustomer {
    pub customer_id: String,
    pub name: Option<String>,
    pub status: Option<CustomerStatus>,
    pub county: Option<String>,
    pub street: Option<String>,
    pub number: Option<String>,
    pub additional: Option<String>,
    pub manufacturer: Option<String>,
    pub year_of_manufacture: Option<String>,
    pub model: Option<String>,
    pub r#type: Option<String>,
    pub warranty: Option<String>,
    pub operation_performed: Option<OperationPerformed>,
    pub appliance_number: Option<String>,
    pub date: Option<String>,
    pub expiration_date: Option<String>,
    pub observations: Option<String>
}

impl IntoIterator for PartialDeliveryCustomer {
    type Item = (String, Option<String>);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // NOTE: `customer_id` is intentionally omitted
        vec![
            ("name".into(), self.name),
            (
                "status".into(),
                self.status.map(|status| status.to_string())
            ),
            ("county".into(), self.county),
            ("street".into(), self.street),
            ("number".into(), self.number),
            ("additional".into(), self.additional),
            ("manufacturer".into(), self.manufacturer),
            ("year_of_manufacture".into(), self.year_of_manufacture),
            ("model".into(), self.model),
            ("type".into(), self.r#type),
            ("warranty".into(), self.warranty),
            (
                "operation_performed".into(),
                self.operation_performed.map(|op_perf| op_perf.to_string())
            ),
            ("appliance_number".into(), self.appliance_number),
            ("date".into(), self.date),
            ("expiration_date".into(), self.expiration_date),
            ("observations".into(), self.observations),
        ]
        .into_iter()
    }
}

impl PartialDeliveryCustomer {
    /// Converts a [`PartialDeliveryCustomer`] into a MongoDB [`Document`]
    ///
    /// Filters out all fields that are `None`. If the filtering is not desired,
    /// use [`into_document`].
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
                | "type"
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

    /// Converts a [`PartialDeliveryCustomer`] into a MongoDB [`Document`]
    ///
    /// There's no filtering done.
    pub fn into_update_document(self) -> Document {
        unimplemented!()
    }
}
