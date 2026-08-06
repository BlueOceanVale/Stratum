use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::state::AppState;
use crate::auth::jwt::create_token;
use argon2::{
    Argon2, PasswordHasher, password_hash::{PasswordHash, PasswordVerifier, Error, SaltString, rand_core},
};
use crate::models::models::{LoginRequest, User, LoginResponse, ErrorResponse};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

fn hash_password(password: &str) -> Result<String, Error> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut rand_core::OsRng);
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

fn verify_password(password: &str, hash: &str) -> bool { 
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), (StatusCode, Json<ErrorResponse>)> {
    let hashed_password = match hash_password(&payload.password) {
        Ok(value) => value,
        Err(err) => {
            println!("{err}");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: "Failed to process password".to_string() }),
            ));
        }
    };

    // Insert user and return the auto-generated ID directly
    let user_id = match sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO users (name, email, password_hash)
         VALUES ($1, $2, $3)
         RETURNING id"
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&hashed_password)
    .fetch_one(&state.pool)
    .await {
        Ok(id) => id,
        Err(err) => {
            println!("{err}");
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { error: "User registration failed (email may already exist)".to_string() }),
            ));
        }
    };

    // Construct user model for token generation
    let new_user = User {
        id: user_id,
        name: payload.name,
        email: payload.email,
        password_hash: hashed_password,
    };

    // Generate JWT token
    let token = match create_token(&new_user) {
        Ok(token) => token,
        Err(_) => return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: "Failed to create authentication token".to_string() }),
        )),
    };

    Ok((
        StatusCode::CREATED,
        Json(LoginResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
        }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), (StatusCode, Json<ErrorResponse>)> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password_hash
        FROM users
        WHERE email=$1"
    )
    .bind(&payload.email)
    .fetch_one(&state.pool)
    .await;

    let user = match user {
        Ok(user) => user,
        Err(_) => return Err((
            StatusCode::UNAUTHORIZED,
            Json(
                ErrorResponse{
                    error: "Invalid email or password".to_string(),
                }
            ),
        ))
    };

    if !verify_password(&payload.password, &user.password_hash) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(
                ErrorResponse { error: "Invalid email or password".to_string() }
            ),
        ));
    }

    let token = match create_token(&user) {
        Ok(token) => token,
        Err(_) => return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: "Failed to create authentication token".to_string() }),
        )),
    };

    Ok((
        StatusCode::OK,
        Json(LoginResponse { access_token: token, token_type: "Bearer".to_string() }),
    ))
}

pub async fn logout() -> StatusCode {
    StatusCode::NO_CONTENT  
}