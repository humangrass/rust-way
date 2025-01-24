use crate::api::helpers::validate_payload;
use crate::api::payload::RegisterPayload;
use crate::app::AppState;
use crate::entities::access_tokens::AccessTokens;
use crate::entities::error_response::ErrorResponse;
use axum::http::StatusCode;
use axum::{debug_handler, Extension, Json};
use models::user::{User, UserModel};
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterPayload,
    responses(
        (status = 200, description = "Successful login", body = AccessTokens),
        (status = 400, description = "Validation failed", body = ErrorResponse),
        (status = 409, description = "User already exists", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
#[debug_handler]
pub async fn register(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RegisterPayload>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    validate_payload(&payload)?;

    let user: User = match User::try_from(payload) {
        Ok(user) => user,
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

    match state.auth_repository.create(&UserModel::from(user)).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            if e.is_user_already_exists() {
                Err((
                    StatusCode::CONFLICT,
                    Json(ErrorResponse {
                        message: "User already exists".to_string(),
                        details: None,
                    }),
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        message: "Internal server error".to_string(),
                        details: None,
                    }),
                ))
            }
        }
    }
}
