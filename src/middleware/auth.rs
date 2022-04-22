use axum::{
    http::{self, Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::utils;

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

fn token_is_valid(token: &str) -> bool {
    let jwt = *token.split(" ").collect::<Vec<&str>>().get(1).unwrap();
    let user = utils::jwt::decode_jwt(jwt);
    println!("{:?}", user);
    true
}
