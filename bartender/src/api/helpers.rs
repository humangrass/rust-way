use crate::entities::error_response::ErrorResponse;
use axum::http::StatusCode;
use axum::Json;
use regex::Regex;
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
