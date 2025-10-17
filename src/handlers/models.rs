use chrono::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
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
}
