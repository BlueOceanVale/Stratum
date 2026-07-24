use axum::{extract::{Extension, Path, State}, http::StatusCode, Json};
use serde::Deserialize;
use sqlx;
use uuid::Uuid;

use crate::{models::models::Task, state::AppState};
use crate::models::models::{Claims, ErrorResponse, SuccessResponse, UpdateTaskRequest};

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
}

pub async fn add_task(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((workspace_id, project_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let owner_id = claims.sub;

    // Verify parent project exists within the workspace and belongs to owner
    let project_exists = sqlx::query!(
        "SELECT id FROM projects WHERE id = $1 AND workspace_id = $2 AND owner_id = $3",
        project_id,
        workspace_id,
        owner_id
    )
    .fetch_optional(&state.pool)
    .await;

    match project_exists {
        Ok(None) => return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Project or Workspace not found".to_string(),
            }),
        )),
        Err(err) => {
            eprintln!("Project verification error: {}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to verify project context".to_string(),
                }),
            ));
        }
        Ok(Some(_)) => {}
    }

    let status = payload.status.unwrap_or_else(|| "todo".to_string());
    let priority = payload.priority.unwrap_or_else(|| "medium".to_string());

    let result = sqlx::query(
        "INSERT INTO tasks (
            project_id,
            workspace_id,
            owner_id,
            title, 
            description, 
            status,
            priority
        )
        VALUES($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(project_id)
    .bind(workspace_id)
    .bind(owner_id)
    .bind(payload.title)
    .bind(payload.description)
    .bind(status)
    .bind(priority)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(SuccessResponse {
                message: "Task created".to_string(),
            }),
        )),
        Err(err) => {
            eprintln!("Task creation error: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to create task".to_string(),
                }),
            ))
        }
    }
}

pub async fn list_tasks(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((workspace_id, project_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<Vec<Task>>), (StatusCode, Json<ErrorResponse>)> {
    let owner_id = claims.sub;

    let result = sqlx::query_as::<_, Task>(
        "SELECT id, project_id, workspace_id, title, description, status, priority
         FROM tasks
         WHERE project_id = $1 AND workspace_id = $2 AND owner_id = $3"
    )
    .bind(project_id)
    .bind(workspace_id)
    .bind(owner_id)
    .fetch_all(&state.pool)
    .await;

    match result {
        Ok(tasks) => Ok((StatusCode::OK, Json(tasks))),
        Err(err) => {
            eprintln!("Failed to fetch tasks: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to fetch tasks from database".to_string(),
                }),
            ))
        }
    }
}

pub async fn get_task(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((workspace_id, project_id, id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<(StatusCode, Json<Task>), (StatusCode, Json<ErrorResponse>)> {
    let owner_id = claims.sub;

    let result = sqlx::query_as::<_, Task>(
        "SELECT id, project_id, workspace_id, title, description, status, priority
         FROM tasks
         WHERE id = $1 AND project_id = $2 AND workspace_id = $3 AND owner_id = $4"
    )
    .bind(id)
    .bind(project_id)
    .bind(workspace_id)
    .bind(owner_id)
    .fetch_optional(&state.pool)
    .await;

    match result {
        Ok(Some(task)) => Ok((StatusCode::OK, Json(task))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Task not found".to_string(),
            }),
        )),
        Err(err) => {
            eprintln!("Failed to fetch task {}: {:?}", id, err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to fetch task from database".to_string(),
                }),
            ))
        }
    }
}

pub async fn update_task(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((workspace_id, project_id, id)): Path<(Uuid, Uuid, Uuid)>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<(StatusCode, Json<Task>), (StatusCode, Json<ErrorResponse>)> {
    let owner_id = claims.sub;

    let result = sqlx::query_as::<_, Task>(
        "
        UPDATE tasks
        SET
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            status = COALESCE($3, status),
            priority = COALESCE($4, priority)
        WHERE
            id = $5
            AND project_id = $6
            AND workspace_id = $7
            AND owner_id = $8
        RETURNING id, project_id, workspace_id, title, description, status, priority
        "
    )
    .bind(payload.title)
    .bind(payload.description)
    .bind(payload.status)
    .bind(payload.priority)
    .bind(id)
    .bind(project_id)
    .bind(workspace_id)
    .bind(owner_id)
    .fetch_optional(&state.pool)
    .await;

    match result {
        Ok(Some(task)) => Ok((StatusCode::OK, Json(task))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Task not found".to_string(),
            }),
        )),
        Err(err) => {
            eprintln!("Task update error: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to update task".to_string(),
                }),
            ))
        }
    }
}

pub async fn delete_task(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((workspace_id, project_id, id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let owner_id = claims.sub;

    let result = sqlx::query(
        "DELETE FROM tasks
         WHERE id = $1 AND project_id = $2 AND workspace_id = $3 AND owner_id = $4"
    )
    .bind(id)
    .bind(project_id)
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
                        error: "Task not found".to_string(),
                    }),
                ));
            }

            Ok((
                StatusCode::OK,
                Json(SuccessResponse {
                    message: "Task deleted".to_string(),
                }),
            ))
        }
        Err(err) => {
            eprintln!("Task deletion error: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to delete task".to_string(),
                }),
            ))
        }
    }
}