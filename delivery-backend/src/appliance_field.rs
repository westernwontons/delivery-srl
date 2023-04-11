use chrono::{DateTime, FixedOffset};
use mongodb::bson::Bson;

use crate::customer::{CustomerStatus, OperationPerformed};

pub enum ApplianceField {
    String(String),
    DateTime(DateTime<FixedOffset>),
    Number(i32)
}

impl From<u16> for ApplianceField {
    fn from(value: u16) -> Self {
        Self::Number(value as i32)
    }
}

impl From<String> for ApplianceField {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<DateTime<FixedOffset>> for ApplianceField {
    fn from(value: DateTime<FixedOffset>) -> Self {
        Self::DateTime(value)
    }
}

impl From<ApplianceField> for Bson {
    fn from(value: ApplianceField) -> Self {
        match value {
            ApplianceField::String(s) => Bson::String(s),
            ApplianceField::DateTime(d) => Bson::DateTime(d.into()),
            ApplianceField::Number(n) => Bson::Int32(n)
        }
    }
}

impl From<Option<String>> for ApplianceField {
    fn from(value: Option<String>) -> Self {
        Self::String(value.unwrap_or_default())
    }
}

impl From<OperationPerformed> for ApplianceField {
    fn from(value: OperationPerformed) -> Self {
        Self::String(value.to_string())
    }
}

impl From<CustomerStatus> for ApplianceField {
    fn from(value: CustomerStatus) -> Self {
        Self::String(value.to_string())
    }
}
