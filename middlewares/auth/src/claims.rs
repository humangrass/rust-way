use chrono::{Duration, Utc};
use serde::{Serialize, Deserialize};
use models::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,            // User ID
    pub exp: usize,             // Expiration timestamp
    // pub iat: usize,          // Issued at timestamp
    // pub roles: Vec<String>,  // Роли пользователя
    // pub aud: String,         // Audience
    // pub iss: String,         // Issuer
}

impl Claims {
    pub fn from_user(user: &User, expiration: Duration) -> Self {
        Self {
            sub: String::from(user.id),
            exp: (Utc::now() + expiration).timestamp() as usize,
        }
    }
}
