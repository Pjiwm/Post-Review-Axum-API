use crate::models::PayloadConstructor;
use crate::mongo::collection;
use crate::utils::jwt::{self, Claims};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use futures::TryStreamExt;
use mongodb::bson::ser;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Database;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::Arc;
use std::vec::Vec;

/// The Bounds trait is only for readability. Many functions require a list of trait bounds,
/// this trait is simply a collection of all trait bounds. Instead of specifying each trait bound per function
/// The Bounds trait can just be used
pub trait Bounds: PayloadConstructor + Serialize + Sync + Send + Unpin + DeserializeOwned {}
impl<T> Bounds for T where T: PayloadConstructor + Serialize + Sync + Send + Unpin + DeserializeOwned
{}

/// Stores new object of the generic type given.
pub async fn create<T: Bounds>(
    State(db): State<Arc<Database>>,
    Claims { user, .. }: jwt::Claims,
    Json(mut payload): Json<Value>,
) -> impl IntoResponse {
    let collection = db.collection::<T>(&T::name());
    if let Some(user_id) = user.id {
        payload["author_id"] = serde_json::to_value(&user_id.to_string()).unwrap_or(Value::Null);
        let object = match T::new(payload) {
            Ok(o) => o,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": e.to_string()})),
                )
            }
        };
        match collection.insert_one(object, None).await {
            Ok(o) => (StatusCode::OK, Json(json!({ "result": o }))),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            ),
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Could not get author id"})),
        )
    }
}
/// Returns one object of the generic type given in JSON format.
pub async fn get_by_id<T: Bounds>(Path(id): Path<String>) -> impl IntoResponse {
    let mongo_id = ObjectId::parse_str(id);
    if mongo_id.is_err() {
        return (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"})));
    }
    let filter = doc! {"_id": mongo_id.unwrap()};
    let object = collection::<T>()
        .await
        .find_one(filter, None)
        .await
        .unwrap();
    let json = Json(serde_json::to_value(&object).unwrap());
    match &object {
        None => return (StatusCode::NOT_FOUND, Json(json!({"status": "not_found"}))),
        _ => (),
    }
    return (StatusCode::OK, json);
}
/// Returns all object of the generic type given in JSON format.
pub async fn get_all<T: Bounds>() -> impl IntoResponse {
    let mut objects = collection::<T>().await.find(None, None).await.unwrap();
    let mut json: Vec<Value> = Vec::new();

    while let Some(object) = objects.try_next().await.expect(r#"something went wrong"#) {
        json.push(serde_json::to_value(&object).unwrap());
    }
    return (StatusCode::OK, Json(json!({ "objects": json })));
}
/// Updates one object of the generic type.
pub async fn update<T: Bounds>(
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
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
            let result = collection::<T>()
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
/// Removes one object of the generic type.
pub async fn remove<T: Bounds>(Path(id): Path<String>) -> impl IntoResponse {
    let mongo_id = ObjectId::parse_str(id);
    if mongo_id.is_err() {
        return (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"})));
    }
    let filter = doc! {"_id": mongo_id.unwrap()};
    let result = collection::<T>().await.delete_one(filter, None).await.ok();
    // this is similar as what was commented on update, we change the result to json so we can grab the value deletedCount
    // if it didn't delete anything we give a status of not found.
    let result_as_json = Json(serde_json::to_value(&result).unwrap());
    if result_as_json["deletedCount"].as_i64().unwrap() == 0 {
        return (StatusCode::NOT_FOUND, Json(json!({"status": "not_found"})));
    }
    return (StatusCode::OK, Json(json!({ "status": result })));
}