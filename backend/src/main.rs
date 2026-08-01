mod auth;
mod handlers;
mod middleware;
mod models;
mod routes;
mod state;

use state::AppState;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:oceanvale@localhost:5432/stratum_db".to_string());


    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let state = AppState { pool };

    // Build the router with state
    let app = routes::create_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}