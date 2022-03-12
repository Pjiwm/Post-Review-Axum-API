use crate::db::user_db;
use crate::models;
use crate::utils::validators;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use serde_json::{json, Value};
use std::collections::LinkedList;

pub async fn create_user(Json(payload): Json<Value>) -> impl IntoResponse {
    let mut values: LinkedList<String> = LinkedList::new();
    // checks if the payload is valid
    values.push_back("username".to_owned());
    values.push_back("id".to_owned());
    values.push_back("age".to_owned());
    let response = validators::check_fields(Json(payload.to_owned()), values);

    if response.0 == StatusCode::CREATED {
        let user = models::User {
            username: payload["username"].to_string(),
            id: payload["id"].to_string().parse::<u64>().unwrap(),
            age: payload["age"].to_string().parse::<u64>().unwrap(),
        };
        user_db::add(user);
    }
    return response;
}

pub async fn get_user(Path(id): Path<u64>) -> impl IntoResponse {
    let user = user_db::get_by_id(id);
    match &user {
        None => return (StatusCode::NOT_FOUND, Json(json!({"status": "not_found"}))),
        _ => (),
    }

    let data = user.unwrap();
    return (
        StatusCode::OK,
        Json(json!({
            "id": data.id,
            "username": data.username,
            "age": data.age,
        })),
    );
}

pub async fn get_users() -> impl IntoResponse {
    let users = user_db::get();
    let mut data: Vec<Value> = Vec::new();
    for user in users {
        data.push(json!({
            "id": user.id,
            "username": user.username,
            "age": user.age,
        }));
    }
    return (StatusCode::OK, Json(json!({ "users": data })));
}

pub async fn update_user(Path(id): Path<u64>, Json(payload): Json<Value>) -> impl IntoResponse {
    let user = user_db::get_by_id(id);
    match &user {
        None => return (StatusCode::NOT_FOUND, Json(json!({"status": "not_found"}))),
        _ => (),
    }

    let mut data = user.unwrap();
    match payload["username"].is_null() {
        true => (),
        false => data.username = payload["username"].to_string(),
    }

    match payload["age"].is_null() {
        true => (),
        false => data.age = payload["age"].to_string().parse::<u64>().unwrap(),
    }

    let update = user_db::update(data.copy(), id);
    if update {
        return (
            StatusCode::OK,
            Json(json!({
                "id": data.id,
                "username": data.username,
                "age": data.age,
            })),
        );
    }
    return (StatusCode::NOT_FOUND, Json(json!({"status": "not_found"})));
}
