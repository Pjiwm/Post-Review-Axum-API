use std::env;

use crate::models::PayloadConstructor;
use mongodb::{options::ClientOptions, Client, Collection};

pub async fn get_db() -> mongodb::Database {
    let connection_str = match env::var("DB_CONNECTIOn") {
        Ok(s) => s,
        Err(_) => panic!("Environment variable DB_CONNECTION was not set."),
    };
    let client_options = ClientOptions::parse(connection_str).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("sandbox");

    return db;
}

pub async fn collection<T: PayloadConstructor>() -> Collection<T> {
    let coll_name = T::name();
    get_db().await.collection::<T>(coll_name.as_str())
}
