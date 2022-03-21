use serde_derive::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use serde_json::Value;
use axum::response::Json;


#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub age: u64,
}

impl User {
    pub fn new(Json(payload): Json<Value>) -> Result<Self, ()> {
        if !payload["username"].is_string() || !payload["age"].is_number() {
            Err(())
        } else {
            Ok(User {
                    username: payload["username"].to_string(),
                    id: None,
                    age: payload["age"].to_string().parse::<u64>().unwrap(),
                })
        }
    }
    pub fn copy(&self) -> User {
        User {
            username: self.username.to_owned(),
            id: self.id,
            age: self.age,
        }
    }
}