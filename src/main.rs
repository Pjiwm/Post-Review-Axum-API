use axum::extract::{Path, Query};
use axum::{response::Json, routing::*, Router};
use serde_json::{json, Value};
mod controllers;

#[tokio::main]
async fn main() {
    // run server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> axum::Router {
    let app: axum::Router = Router::new()
        .route("/", get(|| async { "hello world" }))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/bar", get(|| async { "bar" }))
        .route("/test", get(controllers::user_controller::get_more))
        .route("/users", post(controllers::user_controller::create_user));
    return app;
}

async fn root() {}

async fn get_foo() -> Json<Value> {
    Json(json!({
        "data": 42,
        "test": "result"
    }))
}

async fn post_foo() -> &'static str {
    return "test";
}

async fn get_bar() {}
