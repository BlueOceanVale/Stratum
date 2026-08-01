const fallbackApiUrl = 'http://localhost:3000/api';
const rawApiUrl = import.meta.env.VITE_API_URL?.trim();
const configuredApiUrl = rawApiUrl && rawApiUrl !== 'undefined' ? rawApiUrl : fallbackApiUrl;
export const API = configuredApiUrl.replace(/\/$/, '');

/** Reads the JWT from localStorage and returns an Authorization header, or null if missing. */
export function getAuthHeader(): { Authorization: string } | null {
	const token = localStorage.getItem('token');
	return token ? { Authorization: `Bearer ${token}` } : null;
}

/** Same as getAuthHeader, but redirects to /login if there's no token. Use in onMount guards. */
export function requireAuth(): { Authorization: string } | null {
	const auth = getAuthHeader();
	if (!auth) {
		window.location.href = '/login';
		return null;
	}
	return auth;
}

/** Reads the active workspace id from localStorage, falling back to a ?workspace_id= query param. */
export function getWorkspaceId(): string | null {
	const stored = localStorage.getItem('current_workspace_id');
	if (stored) return stored;
	const urlParams = new URLSearchParams(window.location.search);
	return urlParams.get('workspace_id');
}

export function setWorkspaceId(id: string) {
	localStorage.setItem('current_workspace_id', id);
}

export function logout() {
	localStorage.removeItem('token');
	localStorage.removeItem('current_workspace_id');
	window.location.href = '/login';
}

export type Workspace = {
	id: string;
	title: string;
	description: string | null;
	tag: string | null;
};

export type Project = {
	id: string;
	workspace_id: string;
	title: string;
	description: string | null;
	tag: string | null;
};

export type TaskStatus = 'todo' | 'in_progress' | 'done';
export type TaskPriority = 'low' | 'medium' | 'high';

export type Task = {
	id: string;
	project_id: string;
	workspace_id: string;
	title: string;
	description: string | null;
	status: TaskStatus;
	priority: TaskPriority;
};

export type Client = {
	id: string;
	workspace_id: string;
	name: string;
	email: string | null;
	company: string | null;
	status: string;
};

export type Comment = {
	id: string;
	author_id: string;
	author_name: string;
	author_email: string;
	content: string;
	created_at: string | null;
};

export type WorkspaceMember = {
	user_id: string;
	role: string;
};
