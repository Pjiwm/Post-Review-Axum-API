use mongodb::{Client, Database};
use std::env;

pub async fn database() -> Database {
    let connection_str = env::var("DB_CONNECTION").unwrap_or(String::from("mongodb://axum_mongo"));
    let client = Client::with_uri_str(connection_str)
        .await
        .expect("Could not connect to client!");
    client.database("sandbox")
}
