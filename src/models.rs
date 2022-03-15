use serde_derive::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub age: u64,
}

impl User {
    pub fn copy(&self) -> User {
        User {
            username: self.username.to_owned(),
            id: self.id,
            age: self.age,
        }
    }
}