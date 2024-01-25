use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]

pub struct User {
    id: String,
    username: String,
    password: String,
}

impl User {
    pub fn new(id: String, username: String, password: String) -> Self {
        Self {
            id,
            username,
            password,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
}
