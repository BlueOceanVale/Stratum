use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};
use sqlx;
use uuid::Uuid;

use crate::{
    models::models::{
        Claims, Client, CreateClientRequest, ErrorResponse, SuccessResponse, UpdateClientRequest,
    },
    state::AppState,
};

// 1. CREATE CLIENT
pub async fn add_client(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(workspace_id): Path<Uuid>,
    Json(payload): Json<CreateClientRequest>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub;

    // Verify workspace membership
    let is_member = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM workspace_members WHERE workspace_id = $1 AND user_id = $2)",
    )
    .bind(workspace_id)
    .bind(user_id)
    .fetch_one(&state.pool)
    .await
    .unwrap_or(false);

    if !is_member {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "Workspace access denied".to_string(),
            }),
        ));
    }

    let status = payload.status.unwrap_or_else(|| "active".to_string());

    let result = sqlx::query(
        "INSERT INTO clients (workspace_id, owner_id, name, email, company, status)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(workspace_id)
    .bind(user_id)
    .bind(payload.name)
    .bind(payload.email)
    .bind(payload.company)
    .bind(status)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(SuccessResponse {
                message: "Client added successfully".to_string(),
            }),
        )),
        Err(err) => {
            eprintln!("Failed to create client: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to create client".to_string(),
                }),
            ))
        }
    }
}

// 2. LIST CLIENTS
pub async fn list_clients(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(workspace_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Client>>), (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub;

    let result = sqlx::query_as::<_, Client>(
        "SELECT c.id, c.workspace_id, c.name, c.email, c.company, c.status
         FROM clients c
         INNER JOIN workspace_members wm ON c.workspace_id = wm.workspace_id
         WHERE c.workspace_id = $1 AND wm.user_id = $2",
    )
    .bind(workspace_id)
    .bind(user_id)
    .fetch_all(&state.pool)
    .await;

    match result {
        Ok(clients) => Ok((StatusCode::OK, Json(clients))),
        Err(err) => {
            eprintln!("Failed to fetch clients: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to fetch clients".to_string(),
                }),
            ))
        }
    }
}

// 3. UPDATE CLIENT
pub async fn update_client(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((workspace_id, client_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateClientRequest>,
) -> Result<(StatusCode, Json<Client>), (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub;

    let result = sqlx::query_as::<_, Client>(
        "UPDATE clients
         SET
            name = COALESCE($1, name),
            email = COALESCE($2, email),
            company = COALESCE($3, company),
            status = COALESCE($4, status)
         WHERE id = $5 AND workspace_id = $6
           AND workspace_id IN (
               SELECT workspace_id FROM workspace_members 
               WHERE workspace_id = $6 AND user_id = $7 AND role IN ('owner', 'admin')
           )
         RETURNING id, workspace_id, name, email, company, status",
    )
    .bind(payload.name)
    .bind(payload.email)
    .bind(payload.company)
    .bind(payload.status)
    .bind(client_id)
    .bind(workspace_id)
    .bind(user_id)
    .fetch_optional(&state.pool)
    .await;

    match result {
        Ok(Some(client)) => Ok((StatusCode::OK, Json(client))),
        Ok(None) => Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "Client not found or insufficient permissions".to_string(),
            }),
        )),
        Err(err) => {
            eprintln!("Failed to update client: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to update client".to_string(),
                }),
            ))
        }
    }
}

// 4. DELETE CLIENT
pub async fn delete_client(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((workspace_id, client_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub;

    let result = sqlx::query(
        "DELETE FROM clients
         WHERE id = $1 AND workspace_id = $2
           AND workspace_id IN (
               SELECT workspace_id FROM workspace_members 
               WHERE workspace_id = $2 AND user_id = $3 AND role IN ('owner', 'admin')
           )",
    )
    .bind(client_id)
    .bind(workspace_id)
    .bind(user_id)
    .execute(&state.pool)
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => Ok((
            StatusCode::OK,
            Json(SuccessResponse {
                message: "Client deleted".to_string(),
            }),
        )),
        _ => Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "Client not found or insufficient permissions".to_string(),
            }),
        )),
    }
}