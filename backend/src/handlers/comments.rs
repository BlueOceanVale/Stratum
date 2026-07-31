use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    models::models::{CommentWithAuthor, CreateCommentRequest, Claims, ErrorResponse, SuccessResponse},
    state::AppState,
};

// 1. ADD COMMENT TO A TASK
pub async fn add_task_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((workspace_id, task_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let author_id = claims.sub;

    if payload.content.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Comment content cannot be empty".to_string(),
            }),
        ));
    }

    let result = sqlx::query(
        "INSERT INTO comments (workspace_id, author_id, task_id, content)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(workspace_id)
    .bind(author_id)
    .bind(task_id)
    .bind(payload.content)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(SuccessResponse {
                message: "Comment added".to_string(),
            }),
        )),
        Err(err) => {
            eprintln!("Failed to add task comment: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to post comment".to_string(),
                }),
            ))
        }
    }
}

// 2. GET TASK COMMENTS
pub async fn get_task_comments(
    State(state): State<AppState>,
    Path((_workspace_id, task_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<Vec<CommentWithAuthor>>), (StatusCode, Json<ErrorResponse>)> {
    let comments = sqlx::query_as::<_, CommentWithAuthor>(
        "SELECT 
            c.id, 
            c.author_id, 
            u.name AS author_name, 
            u.email AS author_email, 
            c.content, 
            c.created_at
         FROM comments c
         JOIN users u ON c.author_id = u.id
         WHERE c.task_id = $1
         ORDER BY c.created_at ASC",
    )
    .bind(task_id)
    .fetch_all(&state.pool)
    .await;

    match comments {
        Ok(list) => Ok((StatusCode::OK, Json(list))),
        Err(err) => {
            eprintln!("Failed to fetch comments: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to load comments".to_string(),
                }),
            ))
        }
    }
}

// 3. DELETE COMMENT
pub async fn delete_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((_workspace_id, comment_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub;

    let result = sqlx::query(
        "DELETE FROM comments WHERE id = $1 AND author_id = $2",
    )
    .bind(comment_id)
    .bind(user_id)
    .execute(&state.pool)
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => Ok((
            StatusCode::OK,
            Json(SuccessResponse {
                message: "Comment deleted".to_string(),
            }),
        )),
        _ => Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "Unable to delete comment".to_string(),
            }),
        )),
    }
}