use crate::api::entities::{AccessTokens, ErrorResponse};
use crate::api::helpers::{generate_tokens, validate_payload};
use crate::api::payload::RefreshPayload;
use crate::app::AppState;
use auth::claims::Claims;
use axum::http::StatusCode;
use axum::{Extension, Json};
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

    let tokens = generate_tokens(&state, &user)?;

    Ok(Json(tokens))
}
