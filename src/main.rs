use axum::Router;

// our modules
mod controllers;
mod db;
mod models;
mod router;
mod utils;

#[tokio::main]
async fn main() {
    // run server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> axum::Router {
    let app: axum::Router = Router::new().nest("/", router::root::root_router());
    return app;
}
