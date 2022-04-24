use std::{fmt::Debug, str::FromStr};

use axum::{
    http::{self, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use hyper::Method;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{de::DeserializeOwned, Serialize};

use crate::{models, utils};
use crate::{models::PayloadConstructor, mongo::collection};

pub async fn ownership<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if req.method() == Method::PUT || req.method() == Method::DELETE {
        match auth_header {
            Some(auth_header)
                if is_object_owner(auth_header, req.uri().to_string().as_str()).await =>
            {
                return Ok(next.run(req).await)
            }
            _ => return Err(StatusCode::UNAUTHORIZED),
        }
    }
    Ok(next.run(req).await)
}

async fn is_object_owner(token: &str, uri: &str) -> bool {
    let token = *token.split(" ").collect::<Vec<&str>>().get(1).unwrap();
    let claims = utils::jwt::decode_jwt(token);
    let user = claims.map(|c| c.user);
    if !user.is_ok() {
        return false;
    }
    let user = user.unwrap();
    let user_id = user.id.unwrap().to_string();

    let search_info = get_model_and_id(uri);
    if search_info.0.is_none() || search_info.1.is_none() {
        return false;
    }
    // check the model

    match search_info.0.unwrap().as_str() {
        "posts" => {
            return check_from_db::<models::Post>(&search_info.1.unwrap(), &user_id).await;
        }
        "reviews" => {
            return check_from_db::<models::Review>(&search_info.1.unwrap(), &user_id).await
        }
        _ => return false,
    }
}

fn get_model_and_id(uri: &str) -> (Option<String>, Option<String>) {
    let parts = uri.split('/').collect::<Vec<&str>>();
    let model_name = parts.get(1).map(|s| s.to_owned().to_owned());
    let object_id = parts.get(2).map(|s| s.to_owned().to_owned());
    return (model_name, object_id);
}

async fn check_from_db<
    T: PayloadConstructor + Serialize + Sync + Send + Unpin + DeserializeOwned + Debug,
>(
    object_id: &String,
    user_id: &String,
) -> bool {
    let filter = doc! {"_id": mongodb::bson::oid::ObjectId::from_str(object_id).unwrap()};
    let object = collection::<T>()
        .await
        .find_one(filter, None)
        .await
        .unwrap();

    if object.is_none() {
        return false;
    }
    let json = Json(serde_json::to_value(&object).unwrap());
    let author_obj_id = serde_json::from_value::<ObjectId>(json["author_id"].clone());

    author_obj_id.unwrap().to_string().as_str() == user_id.as_str()
}
