use crate::controllers::auth_controller;
use crate::controllers::user_controller;
use crate::middleware::auth;
use crate::models;
use axum::middleware;
use axum::{
    body::{self, BoxBody, Bytes, Full},
    routing::get,
    Router,
};
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;

pub fn routes() -> axum::Router {
    let router: axum::Router = Router::new()
        .route(
            "/",
            get(user_controller::get_all::<models::User>)
                .post(user_controller::create::<models::User>),
        )
        .route(
            "/:id",
            get(user_controller::get_by_id::<models::User>)
                .put(user_controller::update::<models::User>)
                .delete(user_controller::remove::<models::User>),
        )
        .route("/login", get(auth_controller::login))
        .route("/auth", get(auth_controller::authenticate))
        .layer(
            ServiceBuilder::new()
                .map_request_body(body::boxed)
                .layer(middleware::from_fn(auth::print_request_body)),
        );

    return router;
}
