use crate::controllers::generic_controller;
use crate::middleware::auth;
use crate::models;
use axum::middleware;
use axum::{
    body,
    routing::get,
    Router,
};
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;

pub fn routes() -> axum::Router {
    let router: axum::Router = Router::new()
        .route(
            "/",
            get(generic_controller::get_all::<models::Post>)
                .post(generic_controller::create::<models::Post>),
        )
        .route(
            "/:id",
            get(generic_controller::get_by_id::<models::Post>)
                .put(generic_controller::update::<models::Post>)
                .delete(generic_controller::remove::<models::Post>),
        )
        .layer(
            ServiceBuilder::new()
                .map_request_body(body::boxed)
                .layer(middleware::from_fn(auth::print_request_body)),
        );

    return router;
}