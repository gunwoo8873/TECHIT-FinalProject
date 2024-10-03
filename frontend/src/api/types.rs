use chrono::prelude::*;

pub struct User {
    pub id: i32,
    pub user_id: String,
    pub user_email: String,
    pub user_password: String,
    pub password_check: String,
    pub user_address: String,
    pub user_phone: String,
    pub role: String,
    pub update_at: DataTime<Utc>,
    pub create_at: DataTime<Utc>,
}

pub struct User_Register {
    pub id: i32,
    pub user_id: String,
    pub user_email: String,
    pub user_password: String,
    pub password_check: String,
    pub user_address: String,
    pub user_phone: String,
    pub role: String,
}

pub struct User_Signin {
    pub id: i32,
    pub user_id: String,
    pub user_email: String,
    pub user_password: String,
}

pub struct User {
    pub id: i32,
    pub user_id: String,
    pub token: String,
}