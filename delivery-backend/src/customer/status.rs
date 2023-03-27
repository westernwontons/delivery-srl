#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CustomerStatus {
    Active,
    Inactive
}
