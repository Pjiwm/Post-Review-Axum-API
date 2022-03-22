use crate::utils::jwt;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
pub async fn authenticate(claims: jwt::Claims) ->  Result<impl IntoResponse, jwt::AuthError> {
    let json = Json(serde_json::to_value(&claims).unwrap());
    Ok((StatusCode::OK, json))
}