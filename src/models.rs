use axum::response::Json;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub age: u64,
    pub password: String,
}

impl User {
    pub fn new(payload: Value) -> Result<Self> {
        let user = serde_json::from_str(payload.to_string().as_str());
        return user;
    }
    pub fn copy(&self) -> User {
        User {
            username: self.username.to_owned(),
            id: self.id,
            age: self.age,
            password: self.password.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub content: String,
    pub release_date: DateTime,
    pub title: String,
    pub tags: Vec<String>,
}
