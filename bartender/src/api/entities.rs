use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct AccessTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub message: String,
    pub details: Option<serde_json::Value>,
}
