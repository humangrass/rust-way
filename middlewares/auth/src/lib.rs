pub mod tokens;
pub mod claims;
mod jwt;

pub struct JWTState {
    pub secret: String,
    pub access_token_expiration: u64,
    pub refresh_token_expiration: u64,
}
