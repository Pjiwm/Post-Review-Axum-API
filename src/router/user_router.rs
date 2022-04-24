use crate::controllers::auth_controller;
use crate::controllers::generic_controller;
use crate::models;
use axum::routing::post;
use axum::{routing::get, Router};
// Router for authentication and user CRUD actions
pub fn routes() -> axum::Router {
    let router: axum::Router = Router::new()
        .route("/", get(generic_controller::get_all::<models::User>))
        .route("/:id", get(generic_controller::get_by_id::<models::User>))
        .route("/login", get(auth_controller::login))
        .route("/auth", get(auth_controller::authenticate))
        .route("/register", post(auth_controller::register));

    router
}
