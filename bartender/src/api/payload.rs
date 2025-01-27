use crate::api::helpers::validate_password;
use bcrypt::{hash, DEFAULT_COST};
use models::user::User;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, ToSchema, Validate)]
pub struct RegisterPayload {
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    username: String,

    #[validate(email(message = "Invalid email format"))]
    email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    #[validate(custom(function = "validate_password"))]
    password: String,
}

impl TryFrom<RegisterPayload> for User {
    type Error = String;

    fn try_from(payload: RegisterPayload) -> Result<Self, Self::Error> {
        let password_hash = hash(payload.password, DEFAULT_COST)
            .map_err(|e| format!("Failed to hash password: {}", e))?;

        Ok(Self {
            id: Uuid::new_v4(),
            username: payload.username,
            email: payload.email,
            password_hash,
        })
    }
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct LoginPayload {
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct RefreshPayload {
    #[validate(length(min = 1, message = "Refresh token must be provider"))]
    pub refresh_token: String,
}
