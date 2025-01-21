use anyhow::Context;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl From<User> for UserModel {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            password_hash: user.password_hash,
        }
    }
}

impl From<UserModel> for User {
    fn from(model: UserModel) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            password_hash: model.password_hash,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn generate_access_token(&self, jwt_secret: &str, expiration: u64) -> anyhow::Result<String> {
        let claims = Claims {
            sub: self.id.to_string(),
            exp: (Utc::now() + Duration::seconds(expiration as i64)).timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )
        .context("Failed to generate access token")
    }

    pub fn generate_refresh_token(&self, jwt_secret: &str, expiration: u64) -> anyhow::Result<String> {
        let claims = Claims {
            sub: self.id.to_string(),
            exp: (Utc::now() + Duration::seconds(expiration as i64)).timestamp() as usize,
        };

        encode(
            &Header {
                alg: Algorithm::HS256,
                ..Default::default()
            },
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )
        .context("Failed to generate refresh token")
    }
}
