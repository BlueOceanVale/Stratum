pub mod state;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

