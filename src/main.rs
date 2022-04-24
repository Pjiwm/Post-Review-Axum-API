//! # axum-test:
//! Axum test is a sample project for testing how Rust can be used to make a RESTFUL Api that responds with JSON,
//! uses webtokens and stores data by using a database.
//! The goals set for this project were:
//! - Connect to a database
//! - CRUD requests
//! - several models
//! - encrypt passwords of users
//! - use jsonwebtokens for requests
//! - middleware
//! - a model with complex datatypes (time, lists e.g.)
//! - only objects can be deleted/edited by the author
//! 
//! Apart from these goals, this project also succeeded into create a working generic controller.
   


use std::{env, net::SocketAddr};
use axum::Router;
mod controllers;
mod middleware;
mod models;
mod mongo;
mod router;
mod utils;
 
/// #### main:
/// Starts up the app.
/// For the port it looks up the env var port and if it can not be found 3000 will be used.
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
/// Returns the entire app containing all the routes of the application
fn app() -> axum::Router {
    let app: axum::Router = Router::new().nest("/", router::root::root_router());
    return app;
}
