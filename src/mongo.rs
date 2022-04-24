use std::env;

use crate::models::PayloadConstructor;
use mongodb::{options::ClientOptions, Client, Collection};
/// Obtains the mongo database
/// Looks for env variable DB_CONNECTION otherwise uses a default string as connection string.
/// If an incorrect string has been provided the program will simply panic.
pub async fn get_db() -> mongodb::Database {
    let connection_str = env::var("DB_CONNECTION").unwrap_or(String::from("mongodb://axum_mongo"));
    let client_options = ClientOptions::parse(connection_str).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("sandbox");

    return db;
}
/// Receives the collection based on the type that's passed in.
/// Because T implements the trait PayloadConstructor mongoDB can now know to what collection it should assign the typ.
pub async fn collection<T: PayloadConstructor>() -> Collection<T> {
    let coll_name = T::name();
    get_db().await.collection::<T>(coll_name.as_str())
}
