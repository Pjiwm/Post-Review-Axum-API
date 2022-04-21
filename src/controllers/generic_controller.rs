use crate::models::PayloadConstructor;
use crate::mongo::collection;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use futures::TryStreamExt;
use mongodb::bson::ser;
use mongodb::bson::{doc, oid::ObjectId};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};
use std::vec::Vec;


pub async fn create<T: PayloadConstructor + Serialize>(
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let object = T::new(payload);
    match object {
        Ok(u) => {
            let result = collection::<T>()
                .await
                .insert_one(u, None)
                .await
                .unwrap();
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

pub async fn get_by_id<
    T: PayloadConstructor + Serialize + Sync + Send + Unpin + DeserializeOwned,
>(
    Path(id): Path<String>,
) -> impl IntoResponse {
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

pub async fn get_all<T: PayloadConstructor + Serialize + Sync + Send + Unpin + DeserializeOwned>(
) -> impl IntoResponse {
    let mut objects = collection::<T>()
        .await
        .find(None, None)
        .await
        .unwrap();
    let mut json: Vec<Value> = Vec::new();

    while let Some(object) = objects.try_next().await.expect(r#"something went wrong"#) {
        json.push(serde_json::to_value(&object).unwrap());
    }
    return (StatusCode::OK, Json(json!({ "objects": json })));
}

pub async fn update<T: PayloadConstructor + Serialize>(
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

pub async fn remove<T: PayloadConstructor + Serialize>(
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mongo_id = ObjectId::parse_str(id);
    if mongo_id.is_err() {
        return (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"})));
    }
    let filter = doc! {"_id": mongo_id.unwrap()};
    let result = collection::<T>()
        .await
        .delete_one(filter, None)
        .await
        .ok();
    // this is similar as what was commented on update, we change the result to json so we can grab the value deletedCount
    // if it didn't delete anything we give a status of not found.
    let result_as_json = Json(serde_json::to_value(&result).unwrap());
    if result_as_json["deletedCount"].as_i64().unwrap() == 0 {
        return (StatusCode::NOT_FOUND, Json(json!({"status": "not_found"})));
    }
    return (StatusCode::OK, Json(json!({ "status": result })));
}
