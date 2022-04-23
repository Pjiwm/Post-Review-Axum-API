use crate::middleware::ownership;

use super::{post_router, review_router, user_router};
use axum::{Router, middleware};
pub fn root_router() -> axum::Router {
    let router: axum::Router = Router::new()
        .nest("/users", user_router::routes())
        .nest("/posts", post_router::routes())
        .nest("/reviews", review_router::routes())
        .layer(middleware::from_fn(ownership::ownership));

    router
}
