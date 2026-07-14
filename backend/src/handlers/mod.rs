pub mod auth;

pub use auth::{register, login, logout};
pub mod health;

pub use health::{home, health};
