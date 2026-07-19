use axum::{{http::StatusCode}, extract::State, Json};
use serde::Serialize;
use sqlx;
use crate::state::AppState;
use crate::models::models::{ErrorResponse, SuccessResponse};

#[derive(Serialize)]
pub struct CreateWorkspaceRequest {
    pub title: String,
    pub description: Option<String>,
    pub tag: Option<String>,
}

pub async fn add_workspace(
    State(state): State<AppState>,
    Json(worksp): Json<CreateWorkspaceRequest>,
) -> Result<(StatusCode,Json<SuccessResponse>) , (StatusCode, Json<ErrorResponse>)> {
    let result = sqlx::query(
        "INSERT INTO workspaces(title, description, tag)
        VALUES($1, $2, $3)"
    )
        .bind(worksp.title)
        .bind(worksp.description)
        .bind(worksp.tag)
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => Ok((
                StatusCode::CREATED,
                Json(SuccessResponse{
                    message: "Workspace Created".to_string()
                }),
            )
        ),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR, 
            Json(ErrorResponse{
                error: "Failed to create workspace".to_string(),
            }),
        )),
    }
}