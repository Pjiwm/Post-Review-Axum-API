use std::vec::Vec;
use std::collections::HashMap;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use serde_json::{json, Value};
use mongodb::bson::Document;
use mongodb::{
    bson::doc,
};
use futures::stream::TryStreamExt;
use crate::models;
use crate::utils::validators;
use crate::utils::Types;
use crate::mongo::users_coll;

pub async fn create(Json(payload): Json<Value>) -> impl IntoResponse {
    // checks if the payload is valid
    let response = validators::check_fields(Json(payload.to_owned()), body_validators());
    // response.0 is the StatusCode and response.1 is the Json<Value>, Response is a tuple.
    if response.0 == StatusCode::CREATED {
        let user = user_create(Json(payload));
        users_coll().await.insert_one(user, None).await.unwrap();
    }
    return response;
}

pub async fn get_by_id(Path(id): Path<i64>) -> impl IntoResponse {
    let filter = doc! {"id": id};
    let user = users_coll().await.find_one(filter, None).await.unwrap();
    let json = Json(serde_json::to_value(&user).unwrap());
    match &user {
        None => return (StatusCode::NOT_FOUND, Json(json!({"status": "not_found"}))),
        _ => (),
    }
    return (StatusCode::OK, json);
}

pub async fn get_all() -> impl IntoResponse {
    let mut users = users_coll().await.find(None, None).await.unwrap();
    let mut json: Vec<Value> = Vec::new();

    while let Some(user) = users.try_next().await.expect(r#"something went wrong"#) {
        json.push(serde_json::to_value(&user).unwrap());
    }
    return (StatusCode::OK, Json(json!({ "users": json })));
}

pub async fn update(Path(id): Path<i64>, Json(payload): Json<Value>) -> impl IntoResponse {
    let filter = doc! {"id": id};
    // We convert the json payload into a Document so we can pass it in the update_one function.
    let mut changes_doc: Document = Document::new();
    let changes = payload.as_object().unwrap();
    for (k, v) in changes {
        if v.is_number() {
            changes_doc.insert(k, v.as_i64());
        } else {
            changes_doc.insert(k, v.as_str());
        }
    }
    let changes = doc!{"$set": changes_doc};
    let result = users_coll().await.update_one(filter,changes, None).await.ok();
    // The result contains the value matchedCount which shows how many values got changed, 
    // by changing this type to JSON we can check if the value is higher than 0, otherwise it's a 404.
    let result_as_json = Json(serde_json::to_value(&result).unwrap());
    if result_as_json["matchedCount"].as_i64().unwrap() == 0 {
        return (StatusCode::NOT_FOUND, Json(json!({"status": "not_found"})))
    }
    return (StatusCode::OK, Json(json!({"status": result})))
}

pub async fn remove(Path(id): Path<i64>) -> impl IntoResponse {
    let result = users_coll().await.delete_one(doc!{"id": id}, None).await.ok();
    // this is similar as what was commented on update, we change the result to json so we can grab the value deletedCount
    // if it didn't delete anything we give a status of not found.
    let result_as_json = Json(serde_json::to_value(&result).unwrap());
    if result_as_json["deletedCount"].as_i64().unwrap() == 0 {
        return (StatusCode::NOT_FOUND, Json(json!({"status": "not_found"})))
    }

    return (StatusCode::OK, Json(json!({"status": result})));
}

fn body_validators() -> HashMap<String, Types> {
    let mut values: HashMap<String, Types> = HashMap::new();
    values.insert("username".to_owned(), Types::String);
    values.insert("id".to_owned(), Types::U64);
    values.insert("age".to_owned(), Types::U64);
    values
}

fn user_create(Json(payload): Json<Value>) -> models::User {
    let user = models::User {
        username: payload["username"].to_string(),
        id: payload["id"].to_string().parse::<i64>().unwrap(),
        age: payload["age"].to_string().parse::<u64>().unwrap(),
    };
    user
}