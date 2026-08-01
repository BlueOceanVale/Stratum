<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import {
		API,
		requireAuth,
		getWorkspaceId,
		type Project,
		type Task,
		type Comment
	} from '$lib/api';
	import KanbanBoard from '$lib/KanbanBoard.svelte';
	import Nav from '$lib/Nav.svelte';

	let workspaceId = $state('');
	let projectId = $state('');
	let project = $state<Project | null>(null);
	let errorMessage = $state('');

	// Task detail / comments drawer
	let activeTask = $state<Task | null>(null);
	let comments = $state<Comment[]>([]);
	let newComment = $state('');
	let commentsLoading = $state(false);

	async function loadProject() {
		const auth = requireAuth();
		const wsId = getWorkspaceId();
		if (!auth) return;
		if (!wsId) {
			window.location.href = '/workspaces';
			return;
		}
		workspaceId = wsId;
		projectId = page.params.id ?? '';

		try {
			const res = await fetch(`${API}/workspaces/${workspaceId}/projects/${projectId}`, {
				headers: auth
			});
			if (!res.ok) throw new Error('Failed to load project');
			project = await res.json();
		} catch (err) {
			console.error(err);
			errorMessage = 'Could not load this project.';
		}
	}

	async function openTask(task: Task) {
		activeTask = task;
		comments = [];
		newComment = '';
		await loadComments(task.id);
	}

	async function loadComments(taskId: string) {
		const auth = requireAuth();
		if (!auth) return;

		commentsLoading = true;
		try {
			const res = await fetch(`${API}/workspaces/${workspaceId}/tasks/${taskId}/comments`, {
				headers: auth
			});
			if (!res.ok) throw new Error('Failed to load comments');
			comments = await res.json();
		} catch (err) {
			console.error(err);
		} finally {
			commentsLoading = false;
		}
	}

	async function postComment(event: SubmitEvent) {
		event.preventDefault();
		const auth = requireAuth();
		if (!auth || !activeTask || !newComment.trim()) return;

		try {
			const res = await fetch(`${API}/workspaces/${workspaceId}/tasks/${activeTask.id}/comments`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json', ...auth },
				body: JSON.stringify({ content: newComment })
			});
			if (!res.ok) throw new Error('Failed to post comment');

			newComment = '';
			await loadComments(activeTask.id);
		} catch (err) {
			console.error(err);
		}
	}

	async function removeComment(commentId: string) {
		const auth = requireAuth();
		if (!auth || !activeTask) return;

		try {
			const res = await fetch(`${API}/workspaces/${workspaceId}/comments/${commentId}`, {
				method: 'DELETE',
				headers: auth
			});
			if (!res.ok) throw new Error('Failed to delete comment');
			comments = comments.filter((c) => c.id !== commentId);
		} catch (err) {
			console.error(err);
		}
	}

	function closeDrawer() {
		activeTask = null;
	}

	function formatDate(iso: string | null) {
		if (!iso) return '';
		return new Date(iso).toLocaleString();
	}

	onMount(loadProject);
</script>

<svelte:head>
	<title>{project ? project.title : 'Project'} — Stratum</title>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link
		href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<div
	class="font-display relative min-h-screen overflow-hidden bg-[#08090b] text-[#eef2f6] antialiased selection:bg-[#3fa9f5] selection:text-white"
>
	<div class="pointer-events-none fixed inset-0">
		<div
			class="absolute top-[-14%] left-1/2 h-[760px] w-[1200px] -translate-x-1/2 rounded-full bg-[#3fa9f5] opacity-[0.14] blur-[190px]"
		></div>
	</div>

	<Nav active="dashboard" />

	<div class="relative z-10 mx-auto max-w-6xl px-6 py-10">
		<a href="/dashboard" class="font-mono text-[11px] tracking-[0.25em] text-[#5db9f7] uppercase">
			← Back to dashboard
		</a>

		{#if errorMessage}
			<div
				class="mt-6 rounded-xl border border-[#ff3366]/30 bg-[#ff3366]/10 px-4 py-3 text-sm text-[#ff8fa8]"
			>
				{errorMessage}
			</div>
		{/if}

		{#if project}
			<div class="mt-2 flex items-center gap-3">
				<h1 class="text-3xl font-semibold tracking-tight sm:text-4xl">{project.title}</h1>
				{#if project.tag}
					<span
						class="rounded-full bg-white/[0.06] px-2.5 py-1 font-mono text-[10px] tracking-wider text-[#5db9f7] uppercase"
					>
						{project.tag}
					</span>
				{/if}
			</div>
			{#if project.description}
				<p class="mt-2 max-w-2xl text-sm text-white/50">{project.description}</p>
			{/if}

			<div class="mt-10">
				<KanbanBoard {workspaceId} {projectId} onTaskClick={openTask} />
			</div>
		{:else if !errorMessage}
			<p class="mt-6 text-sm text-white/40">Loading project…</p>
		{/if}
	</div>
</div>

<!-- TASK DETAIL / COMMENTS DRAWER -->
{#if activeTask}
	<div class="fixed inset-0 z-50 flex justify-end bg-black/50">
		<div
			class="font-display flex h-full w-full max-w-md flex-col border-l border-white/[0.10] bg-[#0e1116]"
		>
			<div class="flex items-start justify-between border-b border-white/[0.09] px-6 py-5">
				<div>
					<p class="font-mono text-[11px] tracking-[0.2em] text-[#5db9f7] uppercase">Task</p>
					<h2 class="mt-1 text-lg font-semibold text-white">{activeTask.title}</h2>
				</div>
				<button onclick={closeDrawer} class="text-white/40 hover:text-white" aria-label="Close">
					✕
				</button>
			</div>

			<div class="flex-1 overflow-y-auto px-6 py-5">
				{#if activeTask.description}
					<p class="mb-6 text-sm text-white/60">{activeTask.description}</p>
				{/if}

				<h3 class="mb-3 font-mono text-xs tracking-wider text-white/40 uppercase">Comments</h3>

				{#if commentsLoading}
					<p class="text-sm text-white/30">Loading comments…</p>
				{:else if comments.length === 0}
					<p class="text-sm text-white/30">No comments yet.</p>
				{:else}
					<div class="space-y-4">
						{#each comments as comment}
							<div class="rounded-xl border border-white/10 bg-white/[0.02] p-4">
								<div class="flex items-start justify-between gap-2">
									<div>
										<p class="text-sm font-medium text-white">{comment.author_name}</p>
										<p class="font-mono text-[10px] text-white/30">
											{formatDate(comment.created_at)}
										</p>
									</div>
									<button
										onclick={() => removeComment(comment.id)}
										class="text-xs text-white/30 hover:text-[#ff8fa8]"
									>
										Delete
									</button>
								</div>
								<p class="mt-2 text-sm text-white/70">{comment.content}</p>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<form onsubmit={postComment} class="border-t border-white/[0.09] p-4">
				<div class="flex gap-2">
					<input
						type="text"
						bind:value={newComment}
						placeholder="Add a comment…"
						class="flex-1 rounded-xl border border-white/10 bg-white/[0.03] px-4 py-2.5 text-sm text-white placeholder:text-white/25 focus:ring-2 focus:ring-[#3fa9f5]/60 focus:outline-none"
					/>
					<button
						type="submit"
						class="rounded-full bg-gradient-to-b from-[#4fb3f7] to-[#1c6ba3] px-4 py-2.5 text-sm font-medium text-white"
					>
						Post
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	.font-display {
		font-family: 'Space Grotesk', system-ui, sans-serif;
	}
	.font-mono {
		font-family: 'JetBrains Mono', ui-monospace, monospace;
	}
</style>
