use axum::{Json, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use argon2::{
    Argon2, PasswordHasher, password_hash::{Error, SaltString, rand_core},
};

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

fn hash_password(password: &str) -> Result<String, Error> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut rand_core::OsRng);
    let password_hash = argon2.hash_password(password.as_bytes(), &salt);
    match password_hash {
        Ok(hash) => Ok(hash.to_string()),
        Err(err) => Err(err),
    }
}


fn verify_password() { 

}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> (StatusCode, Json<Response>) {
    let hashed_password = hash_password(&payload.password);

    let hashed_password = match hashed_password {
        Ok(value) => value,
        Err(err) => {
            println!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(Response {message: "Failed to create user".to_string()}))
        }
    };

    let result = sqlx::query("
        INSERT INTO users(name, email, password_hash)
        VALUES($1, $2, $3)
    ")
        .bind(payload.name)
        .bind(payload.email)
        .bind(hashed_password)
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => (StatusCode::CREATED, Json(Response{ message:"User created!".to_string() })),
        Err(_err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Response{message:"Failed to create user!".to_string()}))
        }
    }
}

#[allow(dead_code)]
pub async fn login() {
}
