use std::{str::FromStr, sync::Arc};

use axum::{
    extract::State,
    middleware::Next,
    response::{IntoResponse, Response},
};
use hyper::{http, Method, Request, StatusCode};
use mongodb::{bson::{doc, oid::ObjectId}, Collection, Database};
use serde_json::Value;

use crate::utils;

pub async fn check_owner<B>(
    State(state): State<Arc<Database>>,
    req: Request<B>,
    next: Next<B>,
) -> Response {
    let db = state;
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if req.method() == Method::PUT || req.method() == Method::DELETE {
        match auth_header {
            Some(auth_header)
                if is_object_owner(auth_header, &req.uri().to_string(), &db).await =>
            {
                return next.run(req).await
            }
            _ => return (StatusCode::UNAUTHORIZED).into_response(),
        }
    }
    next.run(req).await
}

/// Gets the user and uses a match to check on the correct model.
async fn is_object_owner(token: &str, uri: &str, db: &Arc<Database>) -> bool {
    let token = *token.split(' ').collect::<Vec<&str>>().get(1).unwrap_or(&"");
    let claims = utils::jwt::decode_jwt(token);
    let user_id = if let Ok(c) = claims {
        if let Some(id) = c.user.id {
            id.to_string()
        } else {
            return false;
        }
    } else {
        return false;
    };

    let search_info = get_model_and_id(uri);
    let (model_name, object_id) = if let (Some(model_name), Some(object_id)) = search_info {
        (model_name, object_id)
    } else {
        return false;
    };
    let collection = db.collection::<Value>(&model_name);
    db_lookup(&collection, &object_id, &user_id).await
}

fn get_model_and_id(uri: &str) -> (Option<String>, Option<String>) {
    let parts = uri.split('/').collect::<Vec<&str>>();
    let model_name = parts.get(1).map(|s| s.to_owned().to_owned());
    let object_id = parts.get(2).map(|s| s.to_owned().to_owned());
    (model_name, object_id)
}

async fn db_lookup(collection: &Collection<Value>, object_id: &str, user_id: &str) -> bool {
    let object_id = if let Ok(obj_id) = mongodb::bson::oid::ObjectId::from_str(object_id) {
        obj_id
    } else {
        return false;
    };
    let filter = doc! {"_id": object_id};
    let object = collection.find_one(filter, None).await;
    if let Ok(Some(object)) = object {
        match serde_json::from_value::<ObjectId>(object["author_id"].clone()) {
            Ok(id) => id.to_string() == user_id,
            Err(_) => false,
        }
    } else {
        false
    }
}
