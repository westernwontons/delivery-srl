/// TimeRange is a used to fetch expired customers falling within that range.
///
/// Users can provide either one or both fields to specify a lower and/or upper bound for the search.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TimeRange {
    pub start_date: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub end_date: Option<chrono::DateTime<chrono::FixedOffset>>
}
