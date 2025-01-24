use async_trait::async_trait;
use std::sync::Arc;
use axum::{
    extract::{Extension, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use crate::tokens::TokenManager;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct JwtAuth(pub AuthenticatedUser);

#[async_trait]
impl<S> FromRequestParts<S> for JwtAuth
where
    S: Sync + Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Extension(token_manager) = Extension::<Arc<TokenManager>>::from_request_parts(parts, state)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "TokenManager not found in request extensions".to_string(),
                )
            })?;

        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    (
                        StatusCode::UNAUTHORIZED,
                        "Missing or invalid Authorization header".to_string(),
                    )
                })?;

        let token = bearer.token();

        let claims = token_manager.validate_token(token).map_err(|err| {
            (
                StatusCode::UNAUTHORIZED,
                format!("Invalid token: {err}"),
            )
        })?;

        let user = AuthenticatedUser {
            id: claims.sub,
        };

        Ok(JwtAuth(user))
    }
}
