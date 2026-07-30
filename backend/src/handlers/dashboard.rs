use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};
use sqlx;
use uuid::Uuid;

use crate::{
    models::models::{Claims, ClientSummary, DashboardStats, ErrorResponse},
    state::AppState,
};

pub async fn get_workspace_dashboard(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(workspace_id): Path<Uuid>,
) -> Result<(StatusCode, Json<DashboardStats>), (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub;

    // 1. Verify membership
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
                error: "Access denied to workspace".to_string(),
            }),
        ));
    }

    // 2. Fetch Workspace-Wide Totals
    let total_clients = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM clients WHERE workspace_id = $1",
    )
    .bind(workspace_id)
    .fetch_one(&state.pool)
    .await
    .unwrap_or(0);

    let total_projects = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM projects WHERE workspace_id = $1",
    )
    .bind(workspace_id)
    .fetch_one(&state.pool)
    .await
    .unwrap_or(0);

    let total_tasks = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM tasks WHERE workspace_id = $1",
    )
    .bind(workspace_id)
    .fetch_one(&state.pool)
    .await
    .unwrap_or(0);

    let completed_tasks = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM tasks WHERE workspace_id = $1 AND status = 'done'",
    )
    .bind(workspace_id)
    .fetch_one(&state.pool)
    .await
    .unwrap_or(0);

    let pending_tasks = total_tasks - completed_tasks;

    // 3. Fetch Clients with linked Project & Task counts via LEFT JOINs
    let clients_summary = sqlx::query_as::<_, (Uuid, String, Option<String>, i64, i64)>(
        "SELECT 
            c.id,
            c.name,
            c.company,
            COUNT(DISTINCT p.id) AS total_projects,
            COUNT(DISTINCT t.id) AS total_tasks
         FROM clients c
         LEFT JOIN projects p ON p.client_id = c.id
         LEFT JOIN tasks t ON t.project_id = p.id
         WHERE c.workspace_id = $1
         GROUP BY c.id, c.name, c.company
         ORDER BY c.name ASC",
    )
    .bind(workspace_id)
    .fetch_all(&state.pool)
    .await;

    let client_list = match clients_summary {
        Ok(rows) => rows
            .into_iter()
            .map(|(id, name, company, total_projects, total_tasks)| ClientSummary {
                id,
                name,
                company,
                total_projects,
                total_tasks,
            })
            .collect(),
        Err(err) => {
            eprintln!("Dashboard summary error: {}", err);
            Vec::new()
        }
    };

    Ok((
        StatusCode::OK,
        Json(DashboardStats {
            total_clients,
            total_projects,
            total_tasks,
            pending_tasks,
            completed_tasks,
            clients: client_list,
        }),
    ))
}