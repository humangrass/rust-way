use crate::api::entities::{AccessTokens, ErrorResponse};
use crate::api::helpers::{generate_tokens, validate_payload};
use crate::api::payload::LoginPayload;
use crate::app::AppState;
use axum::http::StatusCode;
use axum::{Extension, Json};
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

    let tokens = generate_tokens(&state, &user)?;

    Ok(Json(tokens))
}
