use std::sync::Arc;

use super::{post_router, review_router, user_router};
use crate::middleware::{logging, ownership};
use axum::{middleware, Router};
use mongodb::Database;

// Router for all the other sub routers
// It just nests the sub routers into itself.
pub fn root_router(db: Database) -> axum::Router {
    let state = Arc::new(db);
    Router::new()
        .nest("/users", user_router::routes())
        .nest("/posts", post_router::routes())
        .nest("/reviews", review_router::routes())
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            ownership::check_owner,
        ))
        .layer(middleware::from_fn(logging::logger))
        .with_state(state)
}
