use crate::api::payload::{AccessTokens, LoginPayload, RefreshPayload, RegisterPayload};
use crate::app::AppState;
use crate::entities::user::{Claims, User, UserModel};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{debug_handler, Extension, Json, Router};
use axum_extra::TypedHeader;
use headers::authorization::Bearer;
use headers::Authorization;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

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
    if let Err(validation_error) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: "Validation failed".to_string(),
                details: Some(format_validation_errors(validation_error)),
            }),
        ));
    }

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
    if let Err(validation_error) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: "Validation failed".to_string(),
                details: Some(format_validation_errors(validation_error)),
            }),
        ));
    }

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

#[derive(Serialize, ToSchema)]
pub struct ValidateResponse {
    pub user_id: String,
    pub message: String,
}

#[utoipa::path(
    get,
    path = "/api/auth/validate",
    responses(
        (status = 200, description = "Token is valid", body = ValidateResponse),
        (status = 401, description = "Invalid or expired token", body = ErrorResponse),
    )
)]
pub async fn validate(
    Extension(state): Extension<Arc<AppState>>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<ValidateResponse>, (StatusCode, Json<ErrorResponse>)> {
    let token = bearer.token();

    let token_data = match decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_state.jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(data) => data,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    message: "Invalid or expired token".to_string(),
                    details: None,
                }),
            ));
        }
    };

    Ok(Json(ValidateResponse {
        user_id: token_data.claims.sub,
        message: "Token is valid".to_string(),
    }))
}

pub fn router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/validate", get(validate))
}
