use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plant {
    pub name: String,
    pub owner: String,
}
