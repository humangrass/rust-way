use anyhow::Context;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Serialize;
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

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn generate_access_token(&self, jwt_secret: &str) -> anyhow::Result<String> {
        let claims = Claims {
            sub: self.id.to_string(),
            exp: (Utc::now() + Duration::minutes(15)).timestamp() as usize, // Access token available 15min
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )
        .context("Failed to generate access token")
    }

    pub fn generate_refresh_token(&self, jwt_secret: &str) -> anyhow::Result<String> {
        let claims = Claims {
            sub: self.id.to_string(),
            exp: (Utc::now() + Duration::hours(24)).timestamp() as usize, // Refresh token available 24h
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
