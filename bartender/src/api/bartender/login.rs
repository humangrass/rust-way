use crate::api::helpers::validate_payload;
use crate::api::payload::LoginPayload;
use crate::app::AppState;
use crate::entities::access_tokens::AccessTokens;
use crate::entities::error_response::ErrorResponse;
use axum::http::StatusCode;
use axum::{Extension, Json};
use chrono::Duration;
use models::user::User;
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Successful login", body = AccessTokens),
        (status = 401, description = "Invalid username or password", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AccessTokens>, (StatusCode, Json<ErrorResponse>)> {
    validate_payload(&payload)?;

    let user = match state
        .auth_repository
        .find_by_username(&payload.username)
        .await
    {
        Ok(user) => User::from(user),
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    message: "Invalid username or password".to_string(),
                    details: None,
                }),
            ));
        }
    };

    if !bcrypt::verify(&payload.password, &user.password_hash).unwrap_or(false) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                message: "Invalid username or password".to_string(),
                details: None,
            }),
        ));
    }

    let access_token = state
        .token_manager
        .generate_access_token(
            &user,
            Duration::seconds(state.token_manager.access_token_expiration as i64),
        )
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Failed to generate access token".to_string(),
                    details: None,
                }),
            )
        })?;

    let refresh_token = state
        .token_manager
        .generate_refresh_token(
            &user,
            Duration::seconds(state.token_manager.refresh_token_expiration as i64),
        )
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Failed to generate refresh token".to_string(),
                    details: None,
                }),
            )
        })?;

    Ok(Json(AccessTokens {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.token_manager.access_token_expiration,
    }))
}
