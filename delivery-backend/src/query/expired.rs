use mongodb::bson::{doc, oid::ObjectId, Document};
use std::ops::Sub;

/// [`ExpiredCustomersQuery`] is a used to fetch expired customers falling within that range.
///
/// Users can provide either one or both fields to specify a lower and/or upper bound for the search.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ExpiredCustomersQuery {
    pub start_date: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub end_date: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub limit: Option<u32>,
    pub last_seen: Option<ObjectId>
}

impl ExpiredCustomersQuery {
    /// Convert [`Self`] into an aggregation pipeline
    ///
    /// If `start_date` and `end_date` are both optional,
    /// we want to match every document where the `expiration_date`
    /// is in the last year.
    pub fn as_aggregation(self) -> Vec<Document> {
        let mut aggregation = Document::new();

        let limit = doc! { "$limit": self.limit.unwrap_or(50)};

        let return_from = self
            .last_seen
            .map(|oid| doc! {"$match": {"_id": {"$gt": oid}}});

        if let (None, None) = (self.start_date.as_ref(), self.end_date.as_ref()) {
            let now_minus_one_year = chrono::Local::now().sub(chrono::Duration::days(365));

            aggregation.insert(
                "$match",
                doc! { "appliance.expiration_date": { "$gt": now_minus_one_year } }
            );

            if let Some(document) = return_from {
                return vec![document, aggregation, limit];
            }
            return vec![aggregation, limit];
        }

        let mut inner_doc = Document::new();

        if let Some(start_date) = self.start_date {
            inner_doc.insert("$gt", start_date);
        }

        if let Some(end_date) = self.end_date {
            inner_doc.insert("$lt", end_date);
        }

        aggregation.insert("$match", doc! { "appliance.expiration_date": inner_doc });

        if let Some(document) = return_from {
            return vec![document, aggregation, limit];
        }
        vec![aggregation, limit]
    }
}
