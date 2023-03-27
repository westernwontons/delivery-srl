use edgedb_derive::Queryable;

/// Represents the [`Address`] of a [`DeliveryCustomer`]
#[allow(dead_code)]
#[derive(Queryable, serde::Deserialize, serde::Serialize)]
#[edgedb(json)]
pub struct Address {
    county: String,
    street: String,
    number: String,
    additional: String
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
