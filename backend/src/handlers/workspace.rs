use axum::{{http::StatusCode}, extract::{{Extension, State}, Json}};
use serde::Deserialize;
use sqlx;
use crate::{models::models::Workspace, state::AppState};
use crate::models::models::{ErrorResponse, SuccessResponse, Claims};

#[derive(Deserialize)]
pub struct CreateWorkspaceRequest {
    pub title: String,
    pub description: Option<String>,
    pub tag: Option<String>,
}


pub async fn add_workspace(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(worksp): Json<CreateWorkspaceRequest>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let owner_id = claims.sub;

    let result = sqlx::query(
        "INSERT INTO workspaces (
            owner_id,
            title, 
            description, 
            tag
        )
        VALUES($1, $2, $3, $4)"
    )
        .bind(owner_id)
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

pub async fn list_workspace(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<(StatusCode, Json<Vec<Workspace>>), (StatusCode, ErrorResponse)> {
    let owner_id = claims.sub;

    let result = sqlx::query_as::<_, Workspace>(
        "SELECT id, title, description, tag
         FROM workspaces
         WHERE owner_id = $1"
    )
    .bind(owner_id)
    .fetch_all(&state.pool)
    .await;

    match result {
        Ok(workspaces) => Ok((
            StatusCode::OK,
            Json(workspaces),
        )),
        Err(err) => {
            eprintln!("workspace not found error: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error: "Failed to fetch workspaces from database".to_string(),
                },
            ))
        }
    }
}