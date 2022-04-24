use crate::models::{self, PayloadConstructor};
use crate::mongo::collection;
use crate::utils::{self, encryption, jwt};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use mongodb::bson::doc;
use serde_json::{json, Value};
/// Sample controller function using claims. When claims are not available it will just send back unauthenticated.
pub async fn authenticate(claims: jwt::Claims) -> Result<impl IntoResponse, jwt::AuthError> {
    let json = Json(serde_json::to_value(&claims).unwrap());
    Ok((StatusCode::OK, json))
}
/// Gives back a jsonwebtoken if password and username in body are matching.
/// For both registrating and logging in the passwords will be encrypted.
pub async fn login(Json(payload): Json<Value>) -> impl IntoResponse {
    if !payload["username"].is_string() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"status": "no username was inserted"})),
        );
    }
    let filter = doc! {"username": payload["username"].to_string().replace("\"", "")};
    let user = collection::<models::User>()
        .await
        .find_one(filter, None)
        .await
        .unwrap();
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
        &payload["password"].to_string().replace("\"", ""),
    ) {
        let jwt = jwt::encode_user(user.unwrap().copy());
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
/// Stores a new account in the database if body has been filled in correctly.
/// The password given will be stored encrypted.
pub async fn register(Json(payload): Json<Value>) -> impl IntoResponse {
    let user = models::User::new(payload);
    if user.is_err() {
        let err = user.unwrap_err();
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": err.to_string()})),
        );
    }
    let mut user = user.unwrap();
    user.password = utils::encryption::encrypt(&user.password);
    
    collection::<models::User>()
    .await
    .insert_one(user, None)
    .await.unwrap();
    
    return (StatusCode::CREATED, Json(json!({ "result": "Account created, login to receive token"})));
}
