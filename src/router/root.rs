use super::{post_router, review_router, user_router};
use axum::Router;
pub fn root_router() -> axum::Router {
    let router: axum::Router = Router::new()
        .nest("/users", user_router::routes())
        .nest("/posts", post_router::routes())
        .nest("/reviews", review_router::routes());

    return router;
}
