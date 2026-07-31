use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handlers::{
        // Health & Auth
        health, home, login, logout, register,
        // Workspace & Members
        add_workspace, delete_workspace, list_workspace, update_workspace,
        add_workspace_member, delete_workspace_member, update_member_role,
        // Clients
        add_client, delete_client, list_clients, update_client,
        // Projects
        add_project, delete_project, get_project, list_projects, update_project,
        // Tasks
        add_task, delete_task, get_task, list_tasks, update_task,
        // Comments & Dashboard
        add_task_comment, delete_comment, get_task_comments, get_workspace_dashboard,
    },
    middleware::auth::auth, 
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    // 1. PUBLIC ROUTES (No JWT Token Required)
    let public_routes = Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login));

    // 2. PROTECTED ROUTES (Requires Active JWT Token)
    let protected_routes = Router::new()
        // Auth / Session
        .route("/auth/logout", post(logout))
        
        // Workspaces
        .route("/workspaces", post(add_workspace).get(list_workspace))
        .route(
            "/workspaces/:workspace_id",
            put(update_workspace).delete(delete_workspace),
        )
        
        // Workspace Members
        .route(
            "/workspaces/:workspace_id/members",
            post(add_workspace_member),
        )
        .route(
            "/workspaces/:workspace_id/members/:member_id",
            put(update_member_role).delete(delete_workspace_member),
        )
        
        // Clients
        .route(
            "/workspaces/:workspace_id/clients",
            post(add_client).get(list_clients),
        )
        .route(
            "/workspaces/:workspace_id/clients/:client_id",
            put(update_client).delete(delete_client),
        )
        
        // Projects
        .route(
            "/workspaces/:workspace_id/projects",
            post(add_project).get(list_projects),
        )
        .route(
            "/workspaces/:workspace_id/projects/:project_id",
            get(get_project).put(update_project).delete(delete_project),
        )
        
        // Tasks
        .route(
            "/workspaces/:workspace_id/projects/:project_id/tasks",
            post(add_task).get(list_tasks),
        )
        .route(
            "/workspaces/:workspace_id/tasks/:task_id",
            get(get_task).put(update_task).delete(delete_task),
        )
        
        // Task Comments
        .route(
            "/workspaces/:workspace_id/tasks/:task_id/comments",
            post(add_task_comment).get(get_task_comments),
        )
        .route(
            "/workspaces/:workspace_id/comments/:comment_id",
            delete(delete_comment),
        )
        
        // Workspace Dashboard Summary
        .route(
            "/workspaces/:workspace_id/dashboard",
            get(get_workspace_dashboard),
        )
        // Apply JWT authentication middleware across all protected routes
        .layer(middleware::from_fn(auth));

    // Combine public and protected routes under the `/api` prefix
    Router::new()
        .nest("/api", public_routes.merge(protected_routes))
        .with_state(state)
}