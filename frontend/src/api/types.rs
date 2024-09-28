use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono<Utc>,
    pub updated_at: chrono<Utc>,
}

pub struct AppState {
    pub users: Option<User>,
}