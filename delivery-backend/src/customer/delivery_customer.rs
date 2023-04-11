use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use mongodb::bson::{self, Document};
use mongodb::error::Error as MongoError;

use crate::error::AppError;

use super::{appliance::ApplianceOut, Address, ApplianceIn};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CustomerStatus {
    Active,
    Inactive
}

impl std::fmt::Display for CustomerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CustomerStatus::Active => write!(f, "active"),
            CustomerStatus::Inactive => write!(f, "inactive")
        }
    }
}

impl std::str::FromStr for CustomerStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            _ => anyhow::bail!(format!(
                "Cannot create CustomerStatus from {}",
                s
            ))
        }
    }
}

/////////////////////////////////////////////////////////////////////////////

/// Represents a customer that's going IN to the database.
///
/// Because of how `BSON` is serialized, this distinction is
/// necessary for a couple of reasons:
///
/// * The field with `ObjectId` has to be named `_id`.
/// * The `appliance` field has datetimes from `chrono`
/// that don't play well with `BSON`.
///
/// Customers get the appliances checked for certain things.
/// That name of the operation is carried by the `OperationPerformed` enum.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DeliveryCustomerIn {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub customer_id: String,
    pub name: String,
    pub active: bool,
    pub address: Address,
    pub appliance: ApplianceIn
}

impl DeliveryCustomerIn {
    /// Creates a new [`DeliveryCustomerIn`].
    pub fn new(
        customer_id: String,
        name: String,
        active: bool,
        address: Address,
        appliance: ApplianceIn
    ) -> Self {
        Self {
            id: ObjectId::new(),
            customer_id,
            name,
            active,
            address,
            appliance
        }
    }

    /// Convert a [`DeliveryCustomerIn`] into a MongoDB [`Document`]
    /// for update operations
    pub fn into_update_document(self) -> Document {
        let address_document = self.address.into_document();
        let appliance_document = self.appliance.into_document();
        let mut inner_document = Document::default();
        let mut document = Document::default();

        inner_document.insert("name", self.name);
        inner_document.insert("active", self.active);
        inner_document.insert("address", address_document);
        inner_document.insert("appliance", appliance_document);

        document.insert("$set", inner_document);

        document
    }
}

/////////////////////////////////////////////////////////////////////////////

/// [`DeliveryCustomerOut`] is the version of `DeliveryCustomer`
/// that is returned to the client. This separation is necessary,
/// because serialized BSON is ugly.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DeliveryCustomerOut {
    #[serde(
        serialize_with = "serialize_object_id_as_hex_string",
        rename = "_id"
    )]
    pub id: ObjectId,
    pub customer_id: String,
    pub name: String,
    pub active: bool,
    pub address: Address,
    pub appliance: ApplianceOut
}

impl TryFrom<Document> for DeliveryCustomerOut {
    type Error = AppError;

    fn try_from(value: Document) -> Result<Self, Self::Error> {
        bson::from_document(value).map_err(AppError::from)
    }
}

impl TryFrom<Result<Document, MongoError>> for DeliveryCustomerOut {
    type Error = AppError;

    fn try_from(
        value: Result<Document, MongoError>
    ) -> Result<Self, Self::Error> {
        match value {
            Ok(d) => bson::from_document(d).map_err(AppError::from),
            Err(e) => Err(e.into())
        }
    }
}
