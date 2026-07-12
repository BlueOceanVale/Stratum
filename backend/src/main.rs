use axum::{Json, Router, extract::State, routing::{get, post}};
use tokio::net::TcpListener;
use serde::Deserialize;
use sqlx::PgPool;
use dotenvy;
use rand_core;
use argon2::{
    Argon2, PasswordHasher, password_hash::{Error, SaltString},
};


#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[derive(Deserialize)]
struct RegisterRequest {
    name: String,
    email: String,
    password: String,
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

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let db_url = std::env::var("DATABASE_URL").unwrap();

    let pool = PgPool::connect(&db_url).await.unwrap();
    
    let state = AppState {
        pool,
    };

    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .route("/register", post(register))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Server starting on port 8080");
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "ok"
}

async fn home() -> &'static str {
    "Backend Working"
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> &'static str {
    println!("Username: {}", payload.name);
    println!("Email: {}", payload.email);

    let hashed_password = hash_password(&payload.password);

    let hashed_password = match hashed_password {
        Ok(value) => value,
        Err(err) => {
            println!("{err}");
            return "Failed to hash password"
        }
    };

    let result = sqlx::query("
        INSERT INTO users(name, email, password_hash)
        VALUES($1, $2, $3)
    ")  .bind(payload.name)
        .bind(payload.email)
        .bind(hashed_password)
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => "User created!",
        Err(err) => {
            println!("Database Error: {err}");
            "Failed to create user!"
        }
    }

} 