use mongodb::bson::Document;

use super::{Address, Appliance};

#[derive(serde::Serialize, serde::Deserialize)]
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

/// Represents a customer
///
/// Customers get the appliances checked for certain things.
/// That name of the operation is carried by the `OperationPerformed` enum.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DeliveryCustomer {
    pub customer_id: String,
    pub name: String,
    pub active: bool,
    pub address: Address,
    pub appliance: Appliance
}

impl DeliveryCustomer {
    /// Creates a new [`DeliveryCustomer`].
    pub fn new(
        customer_id: String,
        name: String,
        active: bool,
        address: Address,
        appliance: Appliance
    ) -> Self {
        Self {
            customer_id,
            name,
            active,
            address,
            appliance
        }
    }

    /// Convert a [`DeliveryCustomer`] into a MongoDB [`Document`]
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
