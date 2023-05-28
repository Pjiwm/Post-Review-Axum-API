//! # Models
//! Contains all models used for mongoDB collections and
//! all traits required to get proper working logic.
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;
use std::fmt::Debug;

/// Makes it possible to use generics that implement this trait to be used inside a generic controller.
/// The name function is used so we can attach the struct type that implements this trait to a MongoDB collection.
pub trait PayloadConstructor {
    /// Makes it easy to get the name of a struct when a Generic is used.
    fn name() -> String;
    /// The payload constructor itself, makes it possible to immediately pass JSON values to a struct object.
    fn new(payload: Value) -> Result<Self>
    where
        Self: Sized;
}
/// User object can easily be changed to JSON or mongoDB Documents.
/// This is because of the traits from serde Deserialize and Serialize
/// the id has to be an Option<ObjectId> type, simply because if we make a new struct we don't know its ObjectId yet/
/// This value is only filled in when a search has been done from mongoDB.
/// The user struct is also used for authentication and webtokens.
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub password: String,
}

impl PayloadConstructor for User {
    fn new(payload: Value) -> Result<Self> {
        serde_json::from_str(&payload.to_string())
    }
    fn name() -> String {
        "users".to_string()
    }
}
/// User object can easily be changed to JSON or mongoDB Documents.
/// This is because of the traits from serde Deserialize and Serialize
/// the id has to be an Option<ObjectId> type, simply because if we make a new struct we don't know its ObjectId yet/
/// This value is only filled in when a search has been done from mongoDB.
/// The author_id is used to check the ownership.
#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub content: String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub release_date: DateTime,
    pub title: String,
    pub tags: Vec<String>,
    pub author_id: ObjectId,
}

impl PayloadConstructor for Post {
    fn new(payload: Value) -> Result<Self> {
        serde_json::from_str(&payload.to_string())
    }
    fn name() -> String {
        "posts".to_string()
    }
}
/// User object can easily be changed to JSON or mongoDB Documents.
/// This is because of the traits from serde Deserialize and Serialize
/// the id has to be an Option<ObjectId> type, simply because if we make a new struct we don't know its ObjectId yet/
/// This value is only filled in when a search has been done from mongoDB.
/// The author_id is used to check the ownership.
#[derive(Debug, Deserialize, Serialize)]
pub struct Review {
    pub post: ObjectId,
    pub title: String,
    pub review: String,
    pub movie_title: String,
    pub author_id: ObjectId,
}

impl PayloadConstructor for Review {
    fn new(payload: Value) -> Result<Self> {
        serde_json::from_str(&payload.to_string())
    }
    fn name() -> String {
        "reviews".to_string()
    }
}