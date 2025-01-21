use crate::api::helpers::validate_payload;
use crate::api::payload::RefreshPayload;
use crate::app::AppState;
use crate::entities::access_tokens::AccessTokens;
use crate::entities::claims::Claims;
use crate::entities::error_response::ErrorResponse;
use crate::entities::user::User;
use axum::http::StatusCode;
use axum::{Extension, Json};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
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
        &DecodingKey::from_secret(state.jwt_state.jwt_secret.as_ref()),
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

    let access_token = match user.generate_access_token(
        &state.jwt_state.jwt_secret,
        state.jwt_state.access_token_expiration,
    ) {
        Ok(access_token) => access_token,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Failed to generate access token".to_string(),
                    details: None,
                }),
            ))
        }
    };

    let refresh_token = match user.generate_refresh_token(
        &state.jwt_state.jwt_secret,
        state.jwt_state.refresh_token_expiration,
    ) {
        Ok(refresh_token) => refresh_token,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Failed to generate refresh token".to_string(),
                    details: None,
                }),
            ))
        }
    };

    Ok(Json(AccessTokens {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.jwt_state.access_token_expiration,
    }))
}
