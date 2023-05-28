use axum::{
    http::{self, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use crate::utils;
/// Small middleware function that immediately gives back an unauthenticated response
/// This middleware is only used if a controller function uses a parameter using claims.
pub async fn auth<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    match auth_header {
        Some(auth_header) if token_is_valid(auth_header) => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
/// Checks wheter the jsonwebtoken is valid or not
fn token_is_valid(token: &str) -> bool {
    if let Some(token) = token.split(' ').collect::<Vec<&str>>().get(1) {
        utils::jwt::decode_jwt(token).is_ok()
    } else {
        false
    }
}
