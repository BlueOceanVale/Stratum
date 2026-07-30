use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use sqlx;
use uuid::Uuid;

use crate::{
    models::models::{Claims, ErrorResponse, SuccessResponse},
    state::AppState,
};

#[derive(Deserialize)]
pub struct AddMemberRequest {
    pub user_id: Uuid,
    pub role: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateRoleRequest {
    pub role: String,
}

// 1. ADD MEMBER
pub async fn add_workspace_member(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(workspace_id): Path<Uuid>,
    Json(payload): Json<AddMemberRequest>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let requester_id = claims.sub;
    let target_role = payload.role.unwrap_or_else(|| "member".to_string());

    // STEP 1: Verify caller is an 'owner' or 'admin' in this workspace
    let requester_role = sqlx::query_scalar::<_, String>(
        "SELECT role FROM workspace_members WHERE workspace_id = $1 AND user_id = $2",
    )
    .bind(workspace_id)
    .bind(requester_id)
    .fetch_optional(&state.pool)
    .await;

    match requester_role {
        Ok(Some(role)) if role == "owner" || role == "admin" => {}
        Ok(Some(_)) => {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    error: "Insufficient permissions to add members".to_string(),
                }),
            ))
        }
        _ => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "Workspace not found".to_string(),
                }),
            ))
        }
    }

    // STEP 2: Verify the target user exists in the database
    let user_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)",
    )
    .bind(payload.user_id)
    .fetch_one(&state.pool)
    .await;

    match user_exists {
        Ok(true) => {}
        _ => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "Target user does not exist".to_string(),
                }),
            ))
        }
    }

    // STEP 3: Add user to workspace_members
    let result = sqlx::query(
        "INSERT INTO workspace_members (workspace_id, user_id, role)
         VALUES ($1, $2, $3)
         ON CONFLICT (workspace_id, user_id) DO NOTHING",
    )
    .bind(workspace_id)
    .bind(payload.user_id)
    .bind(target_role)
    .execute(&state.pool)
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => Ok((
            StatusCode::CREATED,
            Json(SuccessResponse {
                message: "Member added successfully".to_string(),
            }),
        )),
        Ok(_) => Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "User is already a member of this workspace".to_string(),
            }),
        )),
        Err(err) => {
            eprintln!("Error adding member: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to add workspace member".to_string(),
                }),
            ))
        }
    }
}

// 2. REMOVE MEMBER
pub async fn delete_workspace_member(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((workspace_id, member_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let requester_id = claims.sub;

    // STEP 1: Check requester's role
    let requester_role = sqlx::query_scalar::<_, String>(
        "SELECT role FROM workspace_members WHERE workspace_id = $1 AND user_id = $2",
    )
    .bind(workspace_id)
    .bind(requester_id)
    .fetch_optional(&state.pool)
    .await;

    let is_admin_or_owner = match requester_role {
        Ok(Some(ref role)) if role == "owner" || role == "admin" => true,
        // Allow members to remove themselves (leave workspace)
        Ok(Some(_)) if requester_id == member_id => false,
        _ => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "Workspace not found".to_string(),
                }),
            ))
        }
    };

    if !is_admin_or_owner && requester_id != member_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "You can only remove yourself or require admin rights".to_string(),
            }),
        ));
    }

    // Protect workspace from deleting its owner
    let target_role = sqlx::query_scalar::<_, String>(
        "SELECT role FROM workspace_members WHERE workspace_id = $1 AND user_id = $2",
    )
    .bind(workspace_id)
    .bind(member_id)
    .fetch_optional(&state.pool)
    .await;

    if let Ok(Some(role)) = target_role {
        if role == "owner" {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    error: "Cannot remove the workspace owner".to_string(),
                }),
            ));
        }
    } else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Member not found in this workspace".to_string(),
            }),
        ));
    }

    // STEP 2 & 3: Delete the member
    let result = sqlx::query(
        "DELETE FROM workspace_members WHERE workspace_id = $1 AND user_id = $2",
    )
    .bind(workspace_id)
    .bind(member_id)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(SuccessResponse {
                message: "Member removed successfully".to_string(),
            }),
        )),
        Err(err) => {
            eprintln!("Error removing member: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to remove member".to_string(),
                }),
            ))
        }
    }
}

// 3. UPDATE MEMBER ROLE
pub async fn update_member_role(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((workspace_id, member_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let requester_id = claims.sub;

    // STEP 1: Only 'owner' can update roles
    let requester_role = sqlx::query_scalar::<_, String>(
        "SELECT role FROM workspace_members WHERE workspace_id = $1 AND user_id = $2",
    )
    .bind(workspace_id)
    .bind(requester_id)
    .fetch_optional(&state.pool)
    .await;

    match requester_role {
        Ok(Some(role)) if role == "owner" => {}
        _ => {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    error: "Only the workspace owner can update roles".to_string(),
                }),
            ))
        }
    }

    // STEP 2 & 3: Update role
    let result = sqlx::query(
        "UPDATE workspace_members SET role = $1 WHERE workspace_id = $2 AND user_id = $3",
    )
    .bind(payload.role)
    .bind(workspace_id)
    .bind(member_id)
    .execute(&state.pool)
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => Ok((
            StatusCode::OK,
            Json(SuccessResponse {
                message: "Role updated successfully".to_string(),
            }),
        )),
        Ok(_) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Member not found in workspace".to_string(),
            }),
        )),
        Err(err) => {
            eprintln!("Error updating role: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to update member role".to_string(),
                }),
            ))
        }
    }
}