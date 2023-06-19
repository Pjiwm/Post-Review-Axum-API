use crate::models::Schema;
use crate::utils::jwt::{self, Claims};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use futures::StreamExt;
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
pub trait Bounds: Schema + Serialize + Sync + Send + Unpin + DeserializeOwned {}
impl<T> Bounds for T where T: Schema + Serialize + Sync + Send + Unpin + DeserializeOwned {}

/// Stores new object of the generic type given.
pub async fn create<T: Bounds>(
    State(db): State<Arc<Database>>,
    Claims { user, .. }: jwt::Claims,
    Json(mut payload): Json<Value>,
) -> impl IntoResponse {
    let collection = db.collection::<T>(&T::name());
    if let Some(user_id) = user.id {
        payload["author_id"] = serde_json::to_value(user_id.to_string()).unwrap_or(Value::Null);
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
pub async fn get_by_id<T: Bounds>(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let collection = db.collection::<T>(&T::name());
    let filter = match ObjectId::parse_str(id) {
        Ok(id) => doc! {"_id": id},
        Err(_) => return (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"}))),
    };
    match collection.find_one(filter, None).await {
        Ok(Some(r)) => {
            let v = r.populate(&db).await;
            if v.is_null() {
                (StatusCode::NOT_FOUND, Json(json!({"status": "not_found"})))
            } else {
                (StatusCode::OK, Json(v))
            }
        }
        _ => (StatusCode::NOT_FOUND, Json(json!({"status": "not_found"}))),
    }
}
/// Returns all object of the generic type given in JSON format.
pub async fn get_all<T: Bounds>(State(db): State<Arc<Database>>) -> impl IntoResponse {
    match db.collection::<T>(&T::name()).find(None, None).await {
        Ok(v) => {
            let objects: Vec<T> = v
                .filter_map(|x| async move { x.ok() })
                .collect::<Vec<T>>()
                .await;

            let mut json: Vec<Value> = vec![];
            for o in objects {
                if let Ok(v) = serde_json::to_value(o.populate(&db).await) {
                    json.push(v);
                }
            }
            (StatusCode::OK, Json(json!({ "objects": json })))
        }
        Err(_) => (StatusCode::OK, Json(json!({ "objects": [] }))),
    }
}
/// Updates one object of the generic type.
pub async fn update<T: Bounds>(
    Path(id): Path<String>,
    State(db): State<Arc<Database>>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let collection = db.collection::<T>(&T::name());
    let filter = match ObjectId::parse_str(id) {
        Ok(id) => doc! {"_id": id},
        Err(_) => return (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"}))),
    };
    match ser::to_bson(&payload) {
        Ok(d) => {
            if let Some(doc) = d.as_document() {
                let result = collection
                    .update_one(filter, doc! {"$set":doc}, None)
                    .await
                    .ok();
                (StatusCode::OK, Json(json!({ "status": result })))
            } else {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Could not convert to document"})),
                )
            }
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": e.to_string()})),
        ),
    }
}
/// Removes one object of the generic type.
pub async fn remove<T: Bounds>(
    Path(id): Path<String>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let filter = match ObjectId::parse_str(id) {
        Ok(id) => doc! {"_id": id},
        Err(_) => return (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"}))),
    };
    let result = db
        .collection::<T>(&T::name())
        .delete_one(filter, None)
        .await
        .ok();
    (StatusCode::OK, Json(json!({ "status": result })))
}
