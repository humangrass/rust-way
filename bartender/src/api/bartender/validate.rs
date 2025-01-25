use crate::api::entities::ErrorResponse;
use crate::app::AppState;
use auth::claims::Claims;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_extra::TypedHeader;
use headers::authorization::Bearer;
use headers::Authorization;
use jsonwebtoken::{decode, Algorithm, Validation};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

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
        state.token_manager.get_decoding_key(),
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
