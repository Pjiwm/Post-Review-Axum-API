use std::vec::Vec;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use serde_json::{json, Value};
use mongodb::bson::Document;
use mongodb::{
    bson::{doc, oid::ObjectId},
};
use futures::stream::TryStreamExt;
use crate::models;
use crate::mongo::users_coll;

pub async fn create(Json(payload): Json<Value>) -> impl IntoResponse {
    let user = models::User::new(Json(payload));
    if user.is_ok() {
    let result = users_coll().await.insert_one(user.unwrap(), None).await.unwrap();
    let json = Json(serde_json::to_value(&result).unwrap());
    (StatusCode::OK, json)
    } else {
        (StatusCode::BAD_REQUEST, Json(json!({
            "error": "bad request",
            "solution": "body should have, age: number and username: string"
        })))
    }
}

pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
    
    let filter = doc! {"_id": ObjectId::parse_str(id).unwrap()};
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

pub async fn update(Path(id): Path<String>, Json(payload): Json<Value>) -> impl IntoResponse {
    let filter = doc! {"_id": ObjectId::parse_str(id).unwrap()};
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

pub async fn remove(Path(id): Path<String>) -> impl IntoResponse {
    let filter = doc! {"_id": ObjectId::parse_str(id).unwrap()};
    let result = users_coll().await.delete_one(filter, None).await.ok();
    // this is similar as what was commented on update, we change the result to json so we can grab the value deletedCount
    // if it didn't delete anything we give a status of not found.
    let result_as_json = Json(serde_json::to_value(&result).unwrap());
    if result_as_json["deletedCount"].as_i64().unwrap() == 0 {
        return (StatusCode::NOT_FOUND, Json(json!({"status": "not_found"})))
    }
    return (StatusCode::OK, Json(json!({"status": result})));
}