use crate::claims::Claims;
use crate::JWTState;
use chrono::{Duration, Utc};
use jsonwebtoken::errors::Error as JwtError;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use models::user::User;

pub struct TokenManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    pub access_token_expiration: u64,
    pub refresh_token_expiration: u64,
}

impl TokenManager {
    pub fn new(jwt_state: JWTState) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(jwt_state.secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(jwt_state.secret.as_bytes()),
            access_token_expiration: jwt_state.access_token_expiration,
            refresh_token_expiration: jwt_state.refresh_token_expiration,
        }
    }

    pub fn get_decoding_key(&self) -> &DecodingKey {
        &self.decoding_key
    }

    pub fn get_encoding_key(&self) -> &EncodingKey {
        &self.encoding_key
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, JwtError> {
        decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::new(Algorithm::HS256),
        )
        .map(|data| data.claims)
    }

    pub fn generate_access_token(
        &self,
        user: &User,
        expiration: Duration,
    ) -> Result<String, JwtError> {
        let claims = Claims {
            sub: user.id.to_string(),
            exp: (Utc::now() + expiration).timestamp() as usize,
        };
        encode(&Header::default(), &claims, &self.encoding_key)
    }

    pub fn generate_refresh_token(
        &self,
        user: &User,
        expiration: Duration,
    ) -> Result<String, JwtError> {
        let claims = Claims {
            sub: user.id.to_string(),
            exp: (Utc::now() + expiration).timestamp() as usize,
        };
        encode(&Header::default(), &claims, &self.encoding_key)
    }

    pub fn decode_jwt(&self, token: &str) -> Result<AuthenticatedUser, JwtError> {
        let claims = self.validate_token(token)?;
        Ok(AuthenticatedUser {
            id: claims.sub,
        })
    }
}

pub struct AuthenticatedUser {
    pub id: String,
}
