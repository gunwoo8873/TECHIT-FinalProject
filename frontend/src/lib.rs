use serde::{Deserialize, Serialize};

pub mod views;
pub mod route;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserConfig {
    pub id: i32,
    pub user_name: String,
    pub user_id: String,
    pub user_email: String,
    pub user_password: String,
    pub password_check: String,
    pub user_phone: String,
    pub token: String,
}