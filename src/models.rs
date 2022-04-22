use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;
use mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string;

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
    // pub age: u64,
    pub password: String,
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

impl Unpin for User {}
unsafe impl Sync for User{}


#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub content: String,
    // turn into proper type that uses serde?lld
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub release_date: DateTime,
    pub title: String,
    pub tags: Vec<String>,
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
