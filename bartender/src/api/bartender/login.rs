use crate::api::helpers::validate_payload;
use crate::api::payload::LoginPayload;
use crate::app::AppState;
use crate::entities::access_tokens::AccessTokens;
use crate::entities::error_response::ErrorResponse;
use crate::entities::user::User;
use axum::http::StatusCode;
use axum::{Extension, Json};
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

    let access_token = match user.generate_access_token(
        &state.jwt_state.jwt_secret,
        state.jwt_state.access_token_expiration,
    ) {
        Ok(access_token) => access_token,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Internal server error".to_string(),
                    details: None,
                }),
            ));
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
                    message: "Internal server error".to_string(),
                    details: None,
                }),
            ));
        }
    };

    Ok(Json(AccessTokens {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.jwt_state.access_token_expiration,
    }))
}
