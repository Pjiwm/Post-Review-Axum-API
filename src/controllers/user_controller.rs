use crate::models;
use crate::mongo::users_coll;
use crate::utils::{encryption, jwt};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use futures::stream::TryStreamExt;
use mongodb::bson::ser;
use mongodb::bson::{doc, oid::ObjectId};
use serde_json::{json, Value};
use std::vec::Vec;

pub async fn create(Json(payload): Json<Value>) -> impl IntoResponse {
    let user = models::User::new(payload);
    match user {
        Ok(u) => {
            let result = users_coll().await.insert_one(u, None).await.unwrap();
            let json = Json(serde_json::to_value(&result).unwrap());
            return (StatusCode::OK, json);
        }
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
        }
    }
}

pub async fn login(Json(payload): Json<Value>) -> impl IntoResponse {
    if !payload["username"].is_string() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"status": "no username was inserted"})),
        );
    }
    let filter = doc! {"username": payload["username"].to_string()};
    let user = users_coll().await.find_one(filter, None).await.unwrap();
    match &user {
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"status": "user not found"})),
            )
        }
        _ => (),
    }
    if encryption::validate(
        &user.as_ref().unwrap().password,
        &payload["password"].to_string(),
    ) {
        let jwt = jwt::encode_user(user.unwrap());
        return (
            StatusCode::OK,
            Json(json!({
                "status": "logged in",
                "token": jwt
            })),
        );
    }
    return (
        StatusCode::BAD_REQUEST,
        Json(json!({"status": "incorrect password"})),
    );
}

pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let mongo_id = ObjectId::parse_str(id);
    if mongo_id.is_err() {
        return (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"})));
    }
    let filter = doc! {"_id": mongo_id.unwrap()};
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
    let mongo_id = ObjectId::parse_str(id);
    if mongo_id.is_err() {
        return (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"})));
    }
    let filter = doc! {"_id": mongo_id.unwrap()};
    // remove if we make generic controller
    let doc = ser::to_bson(&payload);
    match doc {
        Ok(d) => {
            let doc = d.as_document().unwrap();
            let result = users_coll()
                .await
                .update_one(filter, doc! {"$set":doc}, None)
                .await
                .ok();
            return (StatusCode::OK, Json(json!({ "status": result })));
        }
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
        }
    }
}

pub async fn remove(Path(id): Path<String>) -> impl IntoResponse {
    let mongo_id = ObjectId::parse_str(id);
    if mongo_id.is_err() {
        return (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"})));
    }
    let filter = doc! {"_id": mongo_id.unwrap()};
    let result = users_coll().await.delete_one(filter, None).await.ok();
    // this is similar as what was commented on update, we change the result to json so we can grab the value deletedCount
    // if it didn't delete anything we give a status of not found.
    let result_as_json = Json(serde_json::to_value(&result).unwrap());
    if result_as_json["deletedCount"].as_i64().unwrap() == 0 {
        return (StatusCode::NOT_FOUND, Json(json!({"status": "not_found"})));
    }
    return (StatusCode::OK, Json(json!({ "status": result })));
}
