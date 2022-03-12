use axum::response::Json;
use std::collections::LinkedList;
use serde_json::{json, Value};
use axum::http::StatusCode;

pub fn check_fields(Json(payload): Json<Value>, required_values: LinkedList<String>) -> (StatusCode, Json<Value>) {
    let mut errors: String = "The following fields are missing: \n".to_owned();
    let mut valid_request = true;
    for item in required_values {
        if payload[&item].is_null() {
            valid_request = false;
            errors.push_str(format!("[{}] ", item.to_owned()).as_str());
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
