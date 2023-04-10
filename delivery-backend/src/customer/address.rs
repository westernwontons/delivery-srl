use mongodb::bson::{bson, Document};

/// Represents the [`Address`] of a [`DeliveryCustomer`]
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
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

    /// Convert [`Self`] into a [`Document`]
    pub fn into_document(self) -> Document {
        Document::from_iter(
            self.into_iter().map(|(key, value)| (key, bson!(value)))
        )
    }
}

impl IntoIterator for Address {
    type Item = (String, String);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("county".into(), self.county),
            ("street".into(), self.street),
            ("number".into(), self.number),
            ("additional".into(), self.additional),
        ]
        .into_iter()
    }
}
