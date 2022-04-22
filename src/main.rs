use std::{env, net::SocketAddr};

use axum::Router;

mod controllers;
mod middleware;
mod models;
mod mongo;
mod router;
mod utils;
#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or(String::from("3000"));
    let addr = ["0.0.0.0:", port.clone().as_str()].concat();
    let server: SocketAddr = addr.parse().expect("Could not parse socket address");

    axum::Server::bind(&server.to_owned())
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> axum::Router {
    let app: axum::Router = Router::new().nest("/", router::root::root_router());
    return app;
}
