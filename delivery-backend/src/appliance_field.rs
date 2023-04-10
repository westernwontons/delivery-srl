use mongodb::bson::Bson;

pub enum ApplianceField {
    String(String),
    DateTime(chrono::DateTime<chrono::FixedOffset>),
    Number(u16)
}

impl From<u16> for ApplianceField {
    fn from(value: u16) -> Self {
        Self::Number(value)
    }
}

impl From<String> for ApplianceField {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<chrono::DateTime<chrono::FixedOffset>> for ApplianceField {
    fn from(value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        Self::DateTime(value)
    }
}

impl From<ApplianceField> for Bson {
    fn from(value: ApplianceField) -> Self {
        match value {
            ApplianceField::String(s) => Bson::String(s),
            ApplianceField::DateTime(d) => Bson::DateTime(d.into()),
            ApplianceField::Number(n) => Bson::Int32(n as i32)
        }
    }
}
