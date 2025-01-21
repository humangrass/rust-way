use crate::entities::user::User;
use bcrypt::{hash, DEFAULT_COST};
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Deserialize, ToSchema, Validate)]
pub struct RegisterPayload {
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    username: String,

    #[validate(email(message = "Invalid email format"))]
    email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    password: String,
}

impl RegisterPayload {
    pub fn validation(&self) -> Result<(), ValidationErrors> {
        if let Err(errors) = self.validate() {
            return Err(errors);
        }

        // TODO: возможно абсолютно всё это можно сделать через validator макросы
        let special_char_regex = Regex::new(r"[!@#$%^&*(),.?:{}|<>']").unwrap();
        if !special_char_regex.is_match(&self.password) {
            let mut errors = ValidationErrors::new();
            let mut error = ValidationError::new("special_char");
            error.message = Some("Password must contain at least one special character".into());
            errors.add("password", error);
            return Err(errors);
        }

        Ok(())
    }
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
    pub username: String,
    pub password: String,
}

impl LoginPayload {
    pub fn validation(&self) -> Result<(), ValidationErrors> {
        // TODO: for validation use validator::{Validate, ValidationError, ValidationErrors};
        if let Err(errors) = self.validate() {
            return Err(errors);
        }

        Ok(())
    }
}

#[derive(Serialize, ToSchema)]
pub struct AccessTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct RefreshPayload {
    #[validate(length(min = 1, message = "Refresh token must be provider"))]
    pub refresh_token: String,
}
