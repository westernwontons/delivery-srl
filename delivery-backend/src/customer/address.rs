/// Represents the [`Address`] of a [`DeliveryCustomer`]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Address {
    pub county: String,
    pub street: String,
    pub number: String,
    pub additional: String
}

impl Address {
    /// Creates a new [`Address`].
    pub fn new(
        county: String,
        street: String,
        number: String,
        additional: String
    ) -> Self {
        Self {
            county,
            street,
            number,
            additional
        }
    }
}
