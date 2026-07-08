use axum::{{routing::get, post}, Router};
use tokio::net::TcpListener;
use serde::Deserialize;
use axum::Json; 

#[derive(Deserialize)]
struct User {
    name: String,
    age: i32,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .route("/user/", post(user));

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

async fn user(Json(user): Json<User>) -> String { 
    let printable = format!("Welcome {}, age {}!",user.name, user.age);
    printable
}