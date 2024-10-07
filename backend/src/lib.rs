use {
    std::env,
    dotenv::dotenv,
    serde::{Deserialize, Serialize},
    chrono::prelude::*,
};

pub mod server;
// pub mod utils;
pub mod api;

mod server_env {
    use super::*;
    pub fn server_host() -> String {
        dotenv().ok();
        env::var("SERVER_HOST").unwrap_or("127.0.0.1".to_string())
    }

    pub fn server_port() -> u16 {
        dotenv().ok();
        env::var("SERVER_PORT").unwrap_or("5000".to_string()).parse().unwrap()
    }
}
mod database_env {
    use super::*;

    pub fn database_host() -> String {
        dotenv().ok();
        env::var("DATABASE_HOST").unwrap_or("127.0.0.1".to_string())
    }

    pub fn database_port() -> u16 {
        dotenv().ok();
        env::var("DATABASE_PORT").unwrap_or("5433".to_string()).parse().unwrap()
    }

    pub fn database_user() -> String {
        dotenv().ok();
        env::var("DATABASE_USER").unwrap_or("gunwoo".to_string())
    }

    pub fn database_password() -> String {
        dotenv().ok();
        env::var("DATABASE_PASSWORD").unwrap_or("meat9563".to_string())
    }

    pub fn database_name() -> String {
        dotenv().ok();
        env::var("DATABASE_NAME").unwrap_or("postgres".to_string())
    }
}

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
