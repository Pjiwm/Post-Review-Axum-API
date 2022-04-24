use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;

pub trait PayloadConstructor {
    fn name() -> String;
    fn new(payload: Value) -> Result<Self>
    where
        Self: Sized;
}
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn copy(&self) -> User {
        User {
            username: self.username.to_owned(),
            id: self.id,
            password: self.password.to_owned(),
        }
    }
}

impl PayloadConstructor for User {
    fn new(payload: Value) -> Result<Self> {
        let user = serde_json::from_str(payload.to_string().as_str());
        return user;
    }
    fn name() -> String {
        "users".to_string()
    }
}

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
        let post = serde_json::from_str(payload.to_string().as_str());
        return post;
    }
    fn name() -> String {
        "posts".to_string()
    }
}

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
        let review = serde_json::from_str(payload.to_string().as_str());
        return review;
    }
    fn name() -> String {
        "reviews".to_string()
    }
}
