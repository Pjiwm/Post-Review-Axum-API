use crate::models::PayloadConstructor;
use mongodb::{Client, Collection};
use std::env;

/// Receives the collection based on the type that's passed in.
/// Because T implements the trait PayloadConstructor mongoDB can now know to what collection it should assign the typ.
pub async fn collection<T: PayloadConstructor>() -> Collection<T> {
    let connection_str = env::var("DB_CONNECTION").unwrap_or(String::from("mongodb://axum_mongo"));
    let client = Client::with_uri_str(connection_str).await.unwrap();
    let coll_name = T::name();
    client
        .database("sandbox")
        .collection::<T>(coll_name.as_str())
}
