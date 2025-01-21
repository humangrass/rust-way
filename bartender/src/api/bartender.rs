use crate::api::payload::{AccessTokens, LoginPayload, RegisterPayload};
use crate::app::AppState;
use crate::entities::user::{User, UserModel};
use axum::http::StatusCode;
use axum::routing::post;
use axum::{debug_handler, Extension, Json, Router};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;
use validator::ValidationErrors;

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub message: String,
    pub details: Option<serde_json::Value>,
}

fn format_validation_errors(errors: ValidationErrors) -> serde_json::Value {
    let mut error_map = serde_json::Map::new();

    for (field, field_errors) in errors.field_errors().iter() {
        let messages: Vec<String> = field_errors
            .iter()
            .filter_map(|e| e.message.as_ref().map(ToString::to_string))
            .collect();
        error_map.insert(
            field.to_string(),
            serde_json::Value::Array(
                messages
                    .into_iter()
                    .map(serde_json::Value::String)
                    .collect(),
            ),
        );
    }

    serde_json::Value::Object(error_map)
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterPayload,
    responses()
)]
#[debug_handler]
pub async fn register(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RegisterPayload>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    if let Err(validation_error) = payload.validation() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: "Validation failed".to_string(),
                details: Some(format_validation_errors(validation_error)),
            }),
        ));
    }

    let user: User = match User::try_from(payload) {
        Ok(user) => user,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Failed to process user".to_string(),
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

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Successful login", body = AccessTokens),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AccessTokens>, (StatusCode, Json<ErrorResponse>)> {
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

    let access_token = match user.generate_access_token(&state.jwt_secret) {
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
    let refresh_token = match user.generate_refresh_token(&state.jwt_secret) {
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
        expires_in: 60 * 60,
    }))
}

#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    // request_body = todo!(),
    responses()
)]
pub async fn refresh(
    Extension(state): Extension<Arc<AppState>>,
    // Json(payload): Json<>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    todo!()
}

#[utoipa::path(get, path = "/api/auth/validate", responses())]
pub async fn validate(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    todo!()
}

pub fn router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
    // .route("/refresh", post(refresh))
    // .route("/validate", get(validate))
}
