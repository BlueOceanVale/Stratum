use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub title: String,
    pub description: Option<String>,
    pub tag: Option<String>,
}

#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String
}




#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub exp: usize,
}

#[derive(Deserialize, Debug, FromRow)]
pub struct Workspace {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub tag: Option<String>,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub tag: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub project_id: Uuid,
    pub workspace_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Client {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub company: Option<String>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateClientRequest {
    pub name: String,
    pub email: Option<String>,
    pub company: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateClientRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub company: Option<String>,
    pub status: Option<String>,
}

// --- Dashboard Aggregation DTOs ---

#[derive(Debug, Serialize)]
pub struct ClientSummary {
    pub id: Uuid,
    pub name: String,
    pub company: Option<String>,
    pub total_projects: i64,
    pub total_tasks: i64,
}

#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub total_clients: i64,
    pub total_projects: i64,
    pub total_tasks: i64,
    pub pending_tasks: i64,
    pub completed_tasks: i64,
    pub clients: Vec<ClientSummary>,
}