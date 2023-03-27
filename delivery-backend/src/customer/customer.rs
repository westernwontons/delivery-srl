use super::{Address, Appliance};
use edgedb_derive::Queryable;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CustomerStatus {
    Active,
    Inactive
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

impl TryFrom<String> for CustomerStatus {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            _ => anyhow::bail!(format!(
                "Cannot create CustomerStatus from {}",
                value
            ))
        }
    }
}

/// Represents a customer
///
/// Customers get the appliances checked for certain things.
/// That name of the operation is carried by the `OperationPerformed` enum.
#[derive(Queryable, serde::Serialize, serde::Deserialize)]
#[edgedb(json)]
pub struct DeliveryCustomer {
    document_id: String,
    name: String,
    status: CustomerStatus,
    address: Address,
    appliance: Appliance
}

impl DeliveryCustomer {
    /// Creates a new [`DeliveryCustomer`].
    pub fn new(
        document_id: String,
        name: String,
        status: CustomerStatus,
        address: Address,
        appliance: Appliance
    ) -> Self {
        Self {
            document_id,
            name,
            status,
            address,
            appliance
        }
    }
}
