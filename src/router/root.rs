use super::user_router;
use axum::Router;
pub fn root_router() -> axum::Router {
    let router: axum::Router = Router::new().nest("/users", user_router::routes());

    return router;
}
