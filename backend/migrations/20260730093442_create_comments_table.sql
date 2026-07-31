-- Add migration script here
CREATE TABLE IF NOT EXISTS comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    author_id UUID NOT NULL, -- User ID of the commenter
    project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
    task_id UUID REFERENCES tasks(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    -- Enforce that a comment belongs to either a task OR a project
    CONSTRAINT comment_target_check CHECK (
        (project_id IS NOT NULL AND task_id IS NULL) OR
        (project_id IS NULL AND task_id IS NOT NULL)
    )
);

CREATE INDEX IF NOT EXISTS idx_comments_task_id ON comments(task_id, created_at ASC);
CREATE INDEX IF NOT EXISTS idx_comments_project_id ON comments(project_id, created_at ASC);