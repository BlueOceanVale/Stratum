use axum::{Router, routing::{get, post}};
use axum::middleware::from_fn;
use tokio::net::TcpListener;
use sqlx::PgPool;
use dotenvy;

mod state;
use state::AppState;

mod handlers;
use handlers::{register, home, login, logout, health};

pub mod models;
pub mod middleware;
mod auth;
use crate::handlers::workspace::add_workspace;
use crate::middleware::auth::auth;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let db_url = std::env::var("DATABASE_URL").unwrap();

    let pool = PgPool::connect(&db_url).await.unwrap();
    
    let state = AppState {
        pool,
    };

    let protected = Router::new()
        .route("/logout", post(logout))
        .route("/workspace", post(add_workspace))
        .route_layer(from_fn(auth))
        .with_state(state.clone());

    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .route("/register", post(register))
        .route("/login", post(login))
        .merge(protected)
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Server starting on port 8080");
    axum::serve(listener, app).await.unwrap();
}