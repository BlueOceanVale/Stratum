use sqlx;
use crate::state::AppState;
use crate::models::models::Workspace;

pub async fn add_workspace(State(state): State<AppState>, space: &Workspace) {
    let workspace = sqlx::query(
        "INSERT INTO workspaces(title, description, tag)
        VALUES($1, $2, $3) 
        WHERE userid=$4 "
    )
        .bind(space.title)
        .bind(space.description)
        .bind(space.tag)
        .bind()
        .fetch_one(state)
        .await;
}