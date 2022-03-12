use super::Types;
use axum::http::StatusCode;
use axum::response::Json;
use serde_json::{json, Value};
use std::collections::HashMap;

pub fn check_fields(
    Json(payload): Json<Value>,
    required_values: HashMap<String, Types>,
) -> (StatusCode, Json<Value>) {
    let mut errors: String = "errors:\n".to_owned();
    let mut valid_request = true;
    for (key, value) in required_values {
        if payload[&key].is_null() {
            valid_request = false;
            errors.push_str(format!("The value {} is missing.\n", key.to_owned()).as_str());
        }
        match value {
            Types::String => if !payload[&key].is_string() {
                valid_request = false;
                errors.push_str(format!("The value {} is not of type String.\n", key.to_owned()).as_str());
            }
            Types::U64 => {
                if !payload[&key].is_u64() {
                    valid_request = false;
                    errors.push_str(format!("The value {} is not of type u64.", key.to_owned()).as_str());
                }
            }
                
        }
    }

    if valid_request {
        return (StatusCode::CREATED, Json(payload));
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"status": "bad_request", "errors": errors.to_string()})),
        );
    }
}
