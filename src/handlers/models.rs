use chrono::DateTime;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

fn validate_custom_alias(alias: &str) -> Result<(), ValidationError> {
    Ok(())
    // let good = (3..=64).contains(&alias.len())
    //     && alias
    //         .chars()
    //         .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    // if good {
    //     Ok(())
    // } else {
    //     Err(ValidationError::new("bad_alias"))
    // }
}

fn validate_expiration_date(date: &DateTime<chrono::Utc>) -> Result<(), ValidationError> {
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
