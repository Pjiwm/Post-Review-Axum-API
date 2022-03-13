use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub id: i64,
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