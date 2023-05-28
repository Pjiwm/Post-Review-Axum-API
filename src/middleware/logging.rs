use axum::{
    http::{HeaderValue, Request},
    middleware::Next,
    response::IntoResponse,
};
use colored::*;
use mongodb::bson::DateTime;

/// Simple logger for requests
pub async fn logger<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let method = req.method().to_string();
    let uri = req.uri().to_string();
    let agent = header_map(
        req.headers().get("user-agent"),
        "Unknown User Agent".to_string(),
    );
    let host = header_map(req.headers().get("host"), "Unknown Host".to_string());
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

fn header_map(header: Option<&HeaderValue>, msg: String) -> String {
    header
        .map(|x| x.to_str().unwrap_or(&msg).to_string())
        .unwrap_or(msg)
}
