use axum::{http::Request, middleware::Next, response::IntoResponse};
use colored::*;
use mongodb::bson::DateTime;

/// Simple logger for requests
pub async fn logger<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let method = req.method().to_string();
    let uri = req.uri().to_string();
    let agent = req.headers().get("user-agent").unwrap().to_str().unwrap();
    let host = req.headers().get("host").unwrap().to_str().unwrap();
    println!(
        "[{}] requested {}: {} via: {} [{}]",
        host.green(),
        method.yellow(),
        format!("'{}'", uri).as_str().green(),
        agent.blue(),
        DateTime::now()
    );
    next.run(req).await
}
