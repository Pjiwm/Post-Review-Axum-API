use std::sync::Arc;

use crate::controllers::generic_controller;
use crate::middleware::auth;
use crate::models;
use axum::middleware;
use axum::{body, routing::get, Router};
use mongodb::Database;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
// Router for Post objects
pub fn routes() -> axum::Router<Arc<Database>> {
    Router::new()
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
                .layer(middleware::from_fn(auth::auth)),
        )
}
