use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::LinkedList;
use super::validators;

#[derive(Deserialize, Serialize)]
struct User {
    username: String,
    id: u64,
    age: u64,
}

pub async fn create_user(Json(payload): Json<Value>) -> impl IntoResponse {

    let mut values: LinkedList<String> = LinkedList::new();
    values.push_back("username".to_owned());
    values.push_back("id".to_owned());
    values.push_back("age".to_owned());
    return validators::valid_request(Json(payload), values);
}

pub async fn get_more() -> Json<Value> {
    Json(json!({
        "data": 595969,
        "test": "result",
        "nested": {
            "data": "nested",
            "value": 30
        },
        "array": [
            "one",
            "two",
            "three"
        ],
        "nested-nested": {
            "data": {"more": "data"},
            "value": 30,
            "array": [
                "one",
                "two",
                "three"
            ]
        }
    }))
}
