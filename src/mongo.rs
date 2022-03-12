use mongodb::{Client, options::ClientOptions, Collection};
use crate::models;

pub async fn get_db() -> mongodb::Database{
    let client_options = ClientOptions::parse("mongodb://axum_mongo").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("sandbox");

    return db;
}

pub async fn users_coll() -> Collection<models::User> {
    get_db().await.collection::<models::User>("users")
}