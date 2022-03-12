use axum::{routing::{get, post, put}, Router};

// our modules
mod controllers;
mod utils;
mod models;
mod db;

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
        .route("/users", post(controllers::user_controller::create_user))
        .route("/users/:id", get(controllers::user_controller::get_user))
        .route("/users", get(controllers::user_controller::get_users))
        .route("/users/:id", put(controllers::user_controller::update_user));
    return app;
}

