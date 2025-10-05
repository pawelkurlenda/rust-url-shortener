use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
    #[serde(default)]
    pub custom_alias: Option<String>,
    #[serde(default)]
    pub expires_at: Option<DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortenResponse {
    pub id: String,
    pub short_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkRecord {
    pub id: String,
    pub target: String,
    pub created_at: DateTime<chrono::Utc>,
    pub hits: u64,
    pub expires_at: Option<DateTime<chrono::Utc>>,
}
