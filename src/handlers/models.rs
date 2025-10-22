use crate::app_settings::settings;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

fn validate_custom_alias(alias: &str) -> Result<(), ValidationError> {
    if alias.len() != settings::max_shortened_len() {
        return Err(ValidationError::new("bad_alias_length"));
    }

    Ok(())
}

fn validate_expiration_date(date: &DateTime<chrono::Utc>) -> Result<(), ValidationError> {
    if date <= &chrono::Utc::now() {
        return Err(ValidationError::new("expiration_in_past"));
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ShortenRequest {
    #[validate(url)]
    pub url: String,
    #[serde(default)]
    #[validate(custom(function = "validate_custom_alias"))]
    pub custom_alias: Option<String>,
    #[serde(default)]
    #[validate(custom(function = "validate_expiration_date"))]
    pub expires_at: Option<DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortenResponse {
    pub id: String,
}
