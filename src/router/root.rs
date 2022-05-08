use crate::middleware::{logging, ownership};

use super::{post_router, review_router, user_router};
use axum::{middleware, Router};
// Router for all the other sub routers
// It just nests the sub routers into itself.
pub fn root_router() -> axum::Router {
    let router: axum::Router = Router::new()
        .nest("/users", user_router::routes())
        .nest("/posts", post_router::routes())
        .nest("/reviews", review_router::routes())
        .layer(middleware::from_fn(ownership::ownership))
        .layer(middleware::from_fn(logging::logger));

    router
}
