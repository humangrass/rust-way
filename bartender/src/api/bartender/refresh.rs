use crate::api::helpers::validate_payload;
use crate::api::payload::RefreshPayload;
use crate::app::AppState;
use crate::entities::access_tokens::AccessTokens;
use crate::entities::error_response::ErrorResponse;
use auth::claims::Claims;
use axum::http::StatusCode;
use axum::{Extension, Json};
use chrono::Duration;
use jsonwebtoken::{decode, Algorithm, Validation};
use models::user::User;
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    request_body = RefreshPayload,
    responses(
        (status = 200, description = "Tokens refreshed successfully", body = AccessTokens),
        (status = 401, description = "Invalid or expired refresh token", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Failed to generate access/refresh token", body = ErrorResponse),
    )
)]
pub async fn refresh(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RefreshPayload>,
) -> Result<Json<AccessTokens>, (StatusCode, Json<ErrorResponse>)> {
    validate_payload(&payload)?;

    let token_data = match decode::<Claims>(
        &payload.refresh_token,
        state.token_manager.get_decoding_key(),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(data) => data,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    message: "Invalid or expired refresh token".to_string(),
                    details: None,
                }),
            ));
        }
    };

    let user_id = match Uuid::parse_str(&token_data.claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    message: "Invalid refresh token".to_string(),
                    details: None,
                }),
            ));
        }
    };

    let user = match state.auth_repository.find_by_id(user_id).await {
        Ok(user_model) => User::from(user_model),
        Err(_) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    message: "User not found".to_string(),
                    details: None,
                }),
            ));
        }
    };

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
