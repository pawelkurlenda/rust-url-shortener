#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
    #[serde(default)]
    pub custom_alias: Option<String>,
    #[serde(default)]
    pub ttl_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortenResponse {
    pub id: String,
    pub short_url: String,
}
