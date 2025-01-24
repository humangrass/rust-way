use async_trait::async_trait;
use axum::{
    extract::{Extension, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::future::Future;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Sync + Send,
{
    type Rejection = (StatusCode, &'static str);

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        Box::pin(async move {
            // Получение Authorization: Bearer ...
            let TypedHeader(Authorization(bearer)) =
                TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                    .await
                    .map_err(|_| {
                        (
                            StatusCode::UNAUTHORIZED,
                            "Missing or invalid Authorization header",
                        )
                    })?;

            let token = bearer.token();

            // Получение TokenManager из Extension
            let Extension(token_manager) =
                Extension::<Arc<crate::tokens::TokenManager>>::from_request_parts(parts, state)
                    .await
                    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "TokenManager not found"))?;

            // Валидация токена
            let claims = token_manager
                .validate_token(token)
                .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;

            // Сбор информации о пользователе
            let user_id = claims.sub;

            // TODO: Можно сходить в бд, проверив что user существует

            Ok(AuthenticatedUser { id: user_id })
        })
    }
}
