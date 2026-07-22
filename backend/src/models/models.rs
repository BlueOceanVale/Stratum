use serde::{Deserialize, Serialize};
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i64,
    pub email: String,
    pub exp: usize,
}

#[derive(Deserialize, Debug)]
pub struct Workspace {
    pub title: String,
    pub description: Option<String>,
    pub tag: Option<String>,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    pub message: String,
}