pub mod health;
pub mod auth;
pub mod workspace;
pub mod projects;
pub mod tasks;
pub mod clients;
pub mod members;
pub mod dashboard;
pub mod comments;

pub use health::{home, health};
pub use auth::{register, login, logout};
pub use workspace::{add_workspace, list_workspace, update_workspace, delete_workspace};
pub use projects::{add_project, list_projects, update_project, delete_project, get_project};
pub use tasks::{add_task, list_tasks, update_task, delete_task, get_task};
pub use clients::{add_client, list_clients, update_client, delete_client};
pub use members::{add_workspace_member, update_member_role, delete_workspace_member};
pub use dashboard::get_workspace_dashboard;
pub use comments::{add_task_comment, get_task_comments, delete_comment};