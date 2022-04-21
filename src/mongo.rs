use crate::models;
use mongodb::{options::ClientOptions, Client, Collection};

pub async fn get_db() -> mongodb::Database {
    let client_options = ClientOptions::parse("mongodb://axum_mongo").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("sandbox");

    return db;
}

pub async fn collection<T>(coll_name: &str) -> Collection<T> {
    get_db().await.collection::<T>(coll_name)
}
