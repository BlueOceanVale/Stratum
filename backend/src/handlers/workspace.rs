use axum::{{http::StatusCode}, extract::{{Extension, Path, State}, Json}};
use serde::Deserialize;
use sqlx;
use crate::{models::models::Workspace, state::AppState};
use crate::models::models::{ErrorResponse, SuccessResponse, Claims, UpdateWorkspaceRequest};
use uuid::Uuid;

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

pub async fn get_workspace(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Workspace>), (StatusCode, ErrorResponse)> {
    let owner_id = claims.sub;

    let result = sqlx::query_as::<_, Workspace>(
        "SELECT id, title, description, tag
         FROM workspaces
         WHERE id = $1 AND owner_id = $2"
    )
    .bind(id)
    .bind(owner_id)
    .fetch_optional(&state.pool)
    .await;

    match result {
        Ok(Some(workspace)) => Ok((StatusCode::OK, Json(workspace))),

        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            ErrorResponse {
                error: "Workspace not found".to_string(),
            },
        )),

        Err(err) => {
            eprintln!("Failed to fetch workspace {}: {:?}", id, err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error: "Failed to fetch workspace from database".to_string(),
                },
            ))
        }
    }
}

pub async fn delete_workspace(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let owner_id = claims.sub;

    let result = sqlx::query(
        "DELETE FROM workspaces
         WHERE owner_id = $1 AND id = $2"
    )
    .bind(owner_id)
    .bind(id)
    .execute(&state.pool)
    .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse {
                        error: "Workspace not found".to_string(),
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
            eprintln!("workspace deletion error: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to delete workspace".to_string(),
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
    let owner_id = claims.sub;

    let result = sqlx::query_as::<_, Workspace>(
        "
        UPDATE workspaces
        SET
            title = $1,
            description = $2,
            tag = $3
        WHERE
            owner_id = $4
            AND id = $5
        RETURNING id, title, description, tag
        "
    )
    .bind(payload.title)
    .bind(payload.description)
    .bind(payload.tag)
    .bind(owner_id)
    .bind(id)
    .fetch_optional(&state.pool)
    .await;

    match result {
        Ok(Some(workspace)) => Ok((
            StatusCode::OK,
            Json(workspace),
        )),

        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Workspace not found".to_string(),
            }),
        )),

        Err(err) => {
            eprintln!("workspace update error: {}", err);

            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to update workspace".to_string(),
                }),
            ))
        }
    }
}

