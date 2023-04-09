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
    pub address: Address,
    pub appliance: Appliance
}

impl DeliveryCustomer {
    /// Creates a new [`DeliveryCustomer`].
    pub fn new(
        customer_id: String,
        name: String,
        address: Address,
        appliance: Appliance
    ) -> Self {
        Self {
            customer_id,
            name,
            address,
            appliance
        }
    }
}
