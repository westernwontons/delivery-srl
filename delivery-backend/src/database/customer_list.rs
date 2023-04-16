use mongodb::{bson::Document, Cursor};

use crate::{customer::DeliveryCustomerList, error::AppError};

/// Try to drive the cursor to yield `Document`s
pub async fn try_customer_list(
    mut cursor: Cursor<Document>
) -> Result<DeliveryCustomerList, AppError> {
    let mut buffer = Vec::with_capacity(10);

    while cursor.advance().await? {
        let deserialized = cursor.deserialize_current().try_into()?;
        buffer.push(deserialized)
    }

    Ok(buffer.into())
}
