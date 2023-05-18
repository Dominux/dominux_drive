use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct InUser {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub password: String,
}

impl User {
    pub fn from_in_user(in_user: InUser, id: usize) -> Self {
        Self {
            id,
            name: in_user.name,
            password: in_user.password,
        }
    }
}
