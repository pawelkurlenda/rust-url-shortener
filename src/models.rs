use chrono::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkRecord {
    pub id: String,
    pub target: String,
    pub created_at: DateTime<chrono::Utc>,
    pub expires_at: Option<DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataResponse {
    pub id: String,
    pub target: String,
    pub created_at: DateTime<chrono::Utc>,
    pub expires_at: Option<DateTime<chrono::Utc>>,
    pub hits: u64,
}
