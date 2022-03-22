use crate::controllers::user_controller;
use axum::{routing::get, Router};
use crate::controllers::auth_controller;

pub fn routes() -> axum::Router {
    let router: axum::Router = Router::new()
        .route(
            "/",
            get(user_controller::get_all).post(user_controller::create),
        )
        .route(
            "/:id",
            get(user_controller::get_by_id)
                .put(user_controller::update)
                .delete(user_controller::remove),
        )
        .route(
            "/login",
            get(user_controller::login),
        )
        .route(
            "/auth",
            get(auth_controller::authenticate)
        );
    return router;
}
