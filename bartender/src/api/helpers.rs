use crate::api::entities::{AccessTokens, ErrorResponse};
use crate::app::AppState;
use axum::http::StatusCode;
use axum::Json;
use chrono::Duration;
use models::user::User;
use regex::Regex;
use std::sync::Arc;
use validator::{Validate, ValidationError, ValidationErrors};

pub fn validate_payload<T: Validate>(payload: &T) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    if let Err(validation_error) = payload.validate() {
        Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: "Validation failed".to_string(),
                details: Some(format_validation_errors(validation_error)),
            }),
        ))
    } else {
        Ok(())
    }
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

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    let special_char_regex = Regex::new(r"[!@#$%^&*(),.?:{}|<>']").unwrap();
    if !special_char_regex.is_match(password) {
        let mut error = ValidationError::new("special_char");
        error.message = Some("Password must contain at least one special character".into());
        return Err(error);
    }
    Ok(())
}

pub fn generate_tokens(
    state: &Arc<AppState>,
    user: &User,
) -> Result<AccessTokens, (StatusCode, Json<ErrorResponse>)> {
    let access_token = state
        .token_manager
        .generate_access_token(
            user,
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
            user,
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

    Ok(AccessTokens {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.token_manager.access_token_expiration,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use serde_json::Value;
    use validator::Validate;

    // ---------------------
    // 1. validate_payload
    // ---------------------

    #[derive(Validate)]
    struct TestPayload {
        #[validate(length(min = 5))]
        name: String,
    }

    #[test]
    fn test_validate_payload_ok() {
        let payload = TestPayload {
            name: "ValidName".to_string(),
        };
        let result = validate_payload(&payload);
        assert!(result.is_ok(), "Expected OK but returned error");
    }

    #[test]
    fn test_validate_payload_err() {
        let payload = TestPayload {
            name: "Ab".to_string(),
        };
        let result = validate_payload(&payload);

        match result {
            Ok(_) => panic!("Expected validation error but returned OK"),
            Err((status, json)) => {
                assert_eq!(status, StatusCode::BAD_REQUEST);
                assert_eq!(json.0.message, "Validation failed");
                let details = json.0.details.unwrap();
                if let Value::Object(map) = details {
                    assert!(map.contains_key("name"), "Error expected for field name");
                } else {
                    panic!("Expected JSON object");
                }
            }
        }
    }

    // ---------------------
    // 2. validate_password
    // ---------------------

    #[test]
    fn test_validate_password_ok() {
        let password = "Password123!";
        let result = validate_password(password);
        assert!(
            result.is_ok(),
            "Expected OK but validation failed with an error"
        );
    }

    #[test]
    fn test_validate_password_err() {
        let password = "Password123";
        let result = validate_password(password);
        assert!(result.is_err(), "Expected error but returned OK");

        if let Err(e) = result {
            assert_eq!(e.code, "special_char");
            let message = e.message.unwrap();
            assert!(
                message.contains("special character"),
                "Expected a message about the need for a special character"
            );
        }
    }
}
