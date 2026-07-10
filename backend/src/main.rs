use axum::{extract::State, {routing::get, post}, Router, Json};
use tokio::net::TcpListener;
use serde::Deserialize;
use sqlx::PgPool;
use dotenvy;

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

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let db_url = std::env::var("DB_URL").unwrap();

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


    sqlx::query("
        INSERT INTO users(name, email, password_hash)
        VALUES($1, $2, $3)
    ")  .bind(payload.name)
        .bind(payload.email)
        .bind(payload.password)
        .execute(&state.pool)
        .await
        .unwrap();

    "User created!"
}