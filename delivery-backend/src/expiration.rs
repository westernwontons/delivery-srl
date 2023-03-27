/// When specified, return expired users between this time range
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExpirationRange {
    pub start_date: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub end_date: Option<chrono::DateTime<chrono::FixedOffset>>
}
