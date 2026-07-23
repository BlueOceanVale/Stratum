use axum::{extract::{Extension, Path, State}, http::StatusCode, Json};
use serde::Deserialize;
use sqlx;
use uuid::Uuid;

use crate::{models::models::Project, state::AppState};
use crate::models::models::{Claims, ErrorResponse, SuccessResponse, UpdateProjectRequest};

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub workspace_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub tag: Option<String>,
}

pub async fn add_project(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let owner_id = claims.sub;

    // Optional check: Ensure the workspace actually belongs to the user before adding a project
    let workspace_exists = sqlx::query!(
        "SELECT id FROM workspaces WHERE id = $1 AND owner_id = $2",
        payload.workspace_id,
        owner_id
    )
    .fetch_optional(&state.pool)
    .await;

    match workspace_exists {
        Ok(None) => return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Workspace not found".to_string(),
            }),
        )),
        Err(err) => {
            eprintln!("Workspace verification error: {}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to verify workspace".to_string(),
                }),
            ));
        }
        Ok(Some(_)) => {}
    }

    let result = sqlx::query(
        "INSERT INTO projects (
            workspace_id,
            owner_id,
            title, 
            description, 
            tag
        )
        VALUES($1, $2, $3, $4, $5)"
    )
    .bind(payload.workspace_id)
    .bind(owner_id)
    .bind(payload.title)
    .bind(payload.description)
    .bind(payload.tag)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(SuccessResponse {
                message: "Project created".to_string(),
            }),
        )),
        Err(err) => {
            eprintln!("Project creation error: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to create project".to_string(),
                }),
            ))
        }
    }
}

pub async fn list_projects(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(workspace_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Project>>), (StatusCode, Json<ErrorResponse>)> {
    let owner_id = claims.sub;

    let result = sqlx::query_as::<_, Project>(
        "SELECT id, workspace_id, title, description, tag
         FROM projects
         WHERE workspace_id = $1 AND owner_id = $2"
    )
    .bind(workspace_id)
    .bind(owner_id)
    .fetch_all(&state.pool)
    .await;

    match result {
        Ok(projects) => Ok((StatusCode::OK, Json(projects))),
        Err(err) => {
            eprintln!("Failed to fetch projects: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to fetch projects from database".to_string(),
                }),
            ))
        }
    }
}

pub async fn get_project(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((workspace_id, id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<Project>), (StatusCode, Json<ErrorResponse>)> {
    let owner_id = claims.sub;

    let result = sqlx::query_as::<_, Project>(
        "SELECT id, workspace_id, title, description, tag
         FROM projects
         WHERE id = $1 AND workspace_id = $2 AND owner_id = $3"
    )
    .bind(id)
    .bind(workspace_id)
    .bind(owner_id)
    .fetch_optional(&state.pool)
    .await;

    match result {
        Ok(Some(project)) => Ok((StatusCode::OK, Json(project))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Project not found".to_string(),
            }),
        )),
        Err(err) => {
            eprintln!("Failed to fetch project {}: {:?}", id, err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to fetch project from database".to_string(),
                }),
            ))
        }
    }
}

pub async fn update_project(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((workspace_id, id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateProjectRequest>,
) -> Result<(StatusCode, Json<Project>), (StatusCode, Json<ErrorResponse>)> {
    let owner_id = claims.sub;

    let result = sqlx::query_as::<_, Project>(
        "
        UPDATE projects
        SET
            title = $1,
            description = $2,
            tag = $3
        WHERE
            id = $4
            AND workspace_id = $5
            AND owner_id = $6
        RETURNING id, workspace_id, title, description, tag
        "
    )
    .bind(payload.title)
    .bind(payload.description)
    .bind(payload.tag)
    .bind(id)
    .bind(workspace_id)
    .bind(owner_id)
    .fetch_optional(&state.pool)
    .await;

    match result {
        Ok(Some(project)) => Ok((StatusCode::OK, Json(project))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Project not found".to_string(),
            }),
        )),
        Err(err) => {
            eprintln!("Project update error: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to update project".to_string(),
                }),
            ))
        }
    }
}

pub async fn delete_project(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((workspace_id, id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let owner_id = claims.sub;

    let result = sqlx::query(
        "DELETE FROM projects
         WHERE id = $1 AND workspace_id = $2 AND owner_id = $3"
    )
    .bind(id)
    .bind(workspace_id)
    .bind(owner_id)
    .execute(&state.pool)
    .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse {
                        error: "Project not found".to_string(),
                    }),
                ));
            }

            Ok((
                StatusCode::OK,
                Json(SuccessResponse {
                    message: "Project deleted".to_string(),
                }),
            ))
        }
        Err(err) => {
            eprintln!("Project deletion error: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to delete project".to_string(),
                }),
            ))
        }
    }
}