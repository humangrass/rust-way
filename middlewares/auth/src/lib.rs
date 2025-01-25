pub mod tokens;
pub mod claims;
pub mod extractor;

pub struct JWTState {
    pub secret: String,
    pub access_token_expiration: u64,
    pub refresh_token_expiration: u64,
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
}
