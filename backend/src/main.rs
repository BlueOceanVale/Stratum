use axum::{Router, routing::{get, post}};
use tokio::net::TcpListener;
use sqlx::PgPool;
use dotenvy;

mod state;
use state::AppState;

mod handlers;
use handlers::{register, home, login, logout, health};

pub mod models;



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
        .route("/logout", post(logout))
        .route("/login", post(login))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Server starting on port 8080");
    axum::serve(listener, app).await.unwrap();
}