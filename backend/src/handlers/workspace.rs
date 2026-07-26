use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use sqlx;
use uuid::Uuid;

use crate::{
    models::models::{Claims, ErrorResponse, SuccessResponse, UpdateWorkspaceRequest, Workspace},
    state::AppState,
};

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
    let user_id = claims.sub;

    // Use a transaction so both workspace creation and member assignment succeed or fail together
    let mut tx = state.pool.begin().await.map_err(|err| {
        eprintln!("Failed to start transaction: {}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to create workspace".to_string(),
            }),
        )
    })?;

    // 1. Insert the workspace
    let workspace_id = match sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO workspaces (owner_id, title, description, tag)
         VALUES ($1, $2, $3, $4)
         RETURNING id",
    )
    .bind(user_id)
    .bind(worksp.title)
    .bind(worksp.description)
    .bind(worksp.tag)
    .fetch_one(&mut *tx)
    .await
    {
        Ok(id) => id,
        Err(err) => {
            eprintln!("Workspace insertion error: {}", err);
            let _ = tx.rollback().await;
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to create workspace".to_string(),
                }),
            ));
        }
    };

    // 2. Add creator as 'owner' in workspace_members
    if let Err(err) = sqlx::query(
        "INSERT INTO workspace_members (workspace_id, user_id, role)
         VALUES ($1, $2, 'owner')",
    )
    .bind(workspace_id)
    .bind(user_id)
    .execute(&mut *tx)
    .await
    {
        eprintln!("Workspace member insertion error: {}", err);
        let _ = tx.rollback().await;
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to assign workspace ownership".to_string(),
            }),
        ));
    }

    tx.commit().await.map_err(|err| {
        eprintln!("Failed to commit transaction: {}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to finalize workspace creation".to_string(),
            }),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        Json(SuccessResponse {
            message: "Workspace Created".to_string(),
        }),
    ))
}

pub async fn list_workspace(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<(StatusCode, Json<Vec<Workspace>>), (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub;

    // Fetch all workspaces where the user is a registered member
    let result = sqlx::query_as::<_, Workspace>(
        "SELECT w.id, w.title, w.description, w.tag
         FROM workspaces w
         INNER JOIN workspace_members wm ON w.id = wm.workspace_id
         WHERE wm.user_id = $1",
    )
    .bind(user_id)
    .fetch_all(&state.pool)
    .await;

    match result {
        Ok(workspaces) => Ok((StatusCode::OK, Json(workspaces))),
        Err(err) => {
            eprintln!("Workspace listing error: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to fetch workspaces from database".to_string(),
                }),
            ))
        }
    }
}

pub async fn get_workspace(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Workspace>), (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub;

    let result = sqlx::query_as::<_, Workspace>(
        "SELECT w.id, w.title, w.description, w.tag
         FROM workspaces w
         INNER JOIN workspace_members wm ON w.id = wm.workspace_id
         WHERE w.id = $1 AND wm.user_id = $2",
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(&state.pool)
    .await;

    match result {
        Ok(Some(workspace)) => Ok((StatusCode::OK, Json(workspace))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Workspace not found".to_string(),
            }),
        )),
        Err(err) => {
            eprintln!("Failed to fetch workspace {}: {:?}", id, err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to fetch workspace from database".to_string(),
                }),
            ))
        }
    }
}

pub async fn update_workspace(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateWorkspaceRequest>,
) -> Result<(StatusCode, Json<Workspace>), (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub;

    // Only allow update if the user's role is 'owner' or 'admin'
    let result = sqlx::query_as::<_, Workspace>(
        "UPDATE workspaces
         SET
             title = COALESCE($1, title),
             description = COALESCE($2, description),
             tag = COALESCE($3, tag)
         WHERE id = $4 
           AND id IN (
               SELECT workspace_id 
               FROM workspace_members 
               WHERE workspace_id = $4 AND user_id = $5 AND role IN ('owner', 'admin')
           )
         RETURNING id, title, description, tag",
    )
    .bind(payload.title)
    .bind(payload.description)
    .bind(payload.tag)
    .bind(id)
    .bind(user_id)
    .fetch_optional(&state.pool)
    .await;

    match result {
        Ok(Some(workspace)) => Ok((StatusCode::OK, Json(workspace))),
        Ok(None) => Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "Workspace not found or insufficient permissions".to_string(),
            }),
        )),
        Err(err) => {
            eprintln!("Workspace update error: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to update workspace".to_string(),
                }),
            ))
        }
    }
}

pub async fn delete_workspace(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub;

    // Strictly require the 'owner' role to delete the workspace
    let result = sqlx::query(
        "DELETE FROM workspaces
         WHERE id = $1 
           AND id IN (
               SELECT workspace_id 
               FROM workspace_members 
               WHERE workspace_id = $1 AND user_id = $2 AND role = 'owner'
           )",
    )
    .bind(id)
    .bind(user_id)
    .execute(&state.pool)
    .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(ErrorResponse {
                        error: "Workspace not found or only the workspace owner can delete it"
                            .to_string(),
                    }),
                ));
            }

            Ok((
                StatusCode::OK,
                Json(SuccessResponse {
                    message: "Workspace deleted".to_string(),
                }),
            ))
        }
        Err(err) => {
            eprintln!("Workspace deletion error: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to delete workspace".to_string(),
                }),
            ))
        }
    }
}