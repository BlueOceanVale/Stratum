use serde::Deserialize;
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password_hash: String
}


#[derive(Debug)]
pub struct Project {
    pub id: u32,
    pub title: String,
    pub description: String,
}