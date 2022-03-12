use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use serde_json::{json, Value};
use std::collections::HashMap;
use mongodb::{
    bson::doc,
};
use crate::db::user_db;
use crate::models;
use crate::utils::validators;
use crate::utils::Types;
use crate::mongo::users_coll;

pub async fn create(Json(payload): Json<Value>) -> impl IntoResponse {
    // checks if the payload is valid
    let mut values: HashMap<String, Types> = HashMap::new();
    values.insert("username".to_owned(), Types::String);
    values.insert("id".to_owned(), Types::U64);
    values.insert("age".to_owned(), Types::U64);
    let response = validators::check_fields(Json(payload.to_owned()), values);
    // response.0 is the StatusCode and response.1 is the Json<Value>, Response is a tuple.
    if response.0 == StatusCode::CREATED {
        let user = models::User {
            username: payload["username"].to_string(),
            id: payload["id"].to_string().parse::<i64>().unwrap(),
            age: payload["age"].to_string().parse::<u64>().unwrap(),
        };
        users_coll().await.insert_one(user, None).await.expect("Something went wrong.");
    }
    return response;
}

pub async fn get_by_id(Path(id): Path<i64>) -> impl IntoResponse {
    // let user = user_db::get_by_id(id);
    let filter = doc! {"id": id};
    let user = users_coll().await.find_one(filter, None).await.unwrap();
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

pub async fn get_all() -> impl IntoResponse {
    let users = user_db::get();
    // let users = users_coll().await.find(None, None).await.unwrap();
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

pub async fn update(Path(id): Path<i64>, Json(payload): Json<Value>) -> impl IntoResponse {
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

pub async fn remove(Path(id): Path<i64>) -> impl IntoResponse {
    let user = user_db::get_by_id(id);
    match &user {
        None => return (StatusCode::NOT_FOUND, Json(json!({"status": "not_found"}))),
        _ => (),
    }

    user_db::remove_by_id(id);
    return (StatusCode::OK, Json(json!({"status": "ok"})));
}
