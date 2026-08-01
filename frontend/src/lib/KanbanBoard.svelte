<script lang="ts">
	import { onMount } from 'svelte';
	import { API, requireAuth, type Task, type TaskStatus, type TaskPriority } from '$lib/api';

	// Reusable Props
	let {
		workspaceId,
		projectId,
		onTaskClick
	}: { workspaceId: string; projectId: string; onTaskClick?: (task: Task) => void } = $props();

	let tasks = $state<Task[]>([]);
	let draggedTaskId = $state<string | null>(null);

	let showCreate = $state(false);
	let createStatus = $state<TaskStatus>('todo');
	let formTitle = $state('');
	let formDescription = $state('');
	let formPriority = $state<TaskPriority>('medium');

	const columns: { label: string; key: TaskStatus }[] = [
		{ label: 'To Do', key: 'todo' },
		{ label: 'In Progress', key: 'in_progress' },
		{ label: 'Done', key: 'done' }
	];

	const priorityColors: Record<TaskPriority, string> = {
		low: 'text-white/40 border-white/10 bg-white/[0.03]',
		medium: 'text-[#f7ad3d] border-[#f7ad3d]/30 bg-[#f7ad3d]/10',
		high: 'text-[#ff8fa8] border-[#ff3366]/30 bg-[#ff3366]/10'
	};

	// Fetch tasks scoped to specific workspace and project
	async function fetchTasks() {
		const auth = requireAuth();
		if (!auth || !workspaceId || !projectId) return;

		try {
			const res = await fetch(`${API}/workspaces/${workspaceId}/projects/${projectId}/tasks`, {
				headers: auth
			});
			if (res.ok) {
				tasks = await res.json();
			}
		} catch (err) {
			console.error('Failed to load tasks', err);
		}
	}

	// Update task status backend call.
	// Note: the only registered route for a single task is
	// PUT /workspaces/{workspace_id}/tasks/{task_id} (no project_id segment).
	async function updateTaskStatus(taskId: string, newStatus: TaskStatus) {
		const auth = requireAuth();
		if (!auth) return;

		// Optimistic UI Update
		const previous = tasks;
		tasks = tasks.map((t) => (t.id === taskId ? { ...t, status: newStatus } : t));

		try {
			const res = await fetch(`${API}/workspaces/${workspaceId}/tasks/${taskId}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json', ...auth },
				body: JSON.stringify({ status: newStatus })
			});
			if (!res.ok) throw new Error('Failed to update task');
		} catch (err) {
			console.error('Failed to update status', err);
			tasks = previous; // Rollback on failure
		}
	}

	async function createTask(event: SubmitEvent) {
		event.preventDefault();
		const auth = requireAuth();
		if (!auth || !formTitle.trim()) return;

		try {
			const res = await fetch(`${API}/workspaces/${workspaceId}/projects/${projectId}/tasks`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json', ...auth },
				body: JSON.stringify({
					title: formTitle,
					description: formDescription || null,
					status: createStatus,
					priority: formPriority
				})
			});
			if (!res.ok) throw new Error('Failed to create task');

			formTitle = '';
			formDescription = '';
			formPriority = 'medium';
			showCreate = false;
			await fetchTasks();
		} catch (err) {
			console.error('Failed to create task', err);
		}
	}

	function openCreate(status: TaskStatus) {
		createStatus = status;
		showCreate = true;
	}

	// Drag and Drop Event Handlers
	function handleDragStart(id: string) {
		draggedTaskId = id;
	}

	function handleDragOver(e: DragEvent) {
		e.preventDefault(); // Required to allow drop
	}

	function handleDrop(targetStatus: TaskStatus) {
		if (draggedTaskId) {
			updateTaskStatus(draggedTaskId, targetStatus);
			draggedTaskId = null;
		}
	}

	// Reload tasks when workspace or project prop changes
	$effect(() => {
		if (workspaceId && projectId) {
			fetchTasks();
		}
	});

	export function refresh() {
		fetchTasks();
	}
</script>

<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
	{#each columns as col}
		<div
			class="flex min-h-[400px] flex-col rounded-2xl border border-white/[0.09] bg-white/[0.02] p-4"
			ondragover={handleDragOver}
			ondrop={() => handleDrop(col.key)}
		>
			<!-- Column Header -->
			<div class="mb-4 flex items-center justify-between">
				<h3 class="font-mono text-xs tracking-wider text-white/60 uppercase">
					{col.label}
				</h3>
				<div class="flex items-center gap-2">
					<span class="rounded-full bg-white/10 px-2 py-0.5 font-mono text-[11px] text-white/50">
						{tasks.filter((t) => t.status === col.key).length}
					</span>
					<button
						onclick={() => openCreate(col.key)}
						class="rounded-full border border-white/10 bg-white/[0.03] px-2 py-0.5 text-xs text-white/50 transition hover:border-[#3fa9f5]/50 hover:text-white"
						title="Add task"
					>
						+
					</button>
				</div>
			</div>

			<!-- Task Cards Stack -->
			<div class="flex flex-1 flex-col gap-3">
				{#each tasks.filter((t) => t.status === col.key) as task (task.id)}
					<button
						draggable="true"
						ondragstart={() => handleDragStart(task.id)}
						onclick={() => onTaskClick?.(task)}
						class="group cursor-grab rounded-xl border border-white/10 bg-[#0e1116] p-4 text-left shadow-sm transition hover:border-[#3fa9f5]/50 active:cursor-grabbing"
					>
						<div class="flex items-start justify-between gap-2">
							<p class="text-sm font-medium text-white">{task.title}</p>
							<span
								class="shrink-0 rounded-full border px-2 py-0.5 font-mono text-[10px] uppercase {priorityColors[
									task.priority
								]}"
							>
								{task.priority}
							</span>
						</div>
						{#if task.description}
							<p class="mt-1 line-clamp-2 text-xs text-white/50">{task.description}</p>
						{/if}
					</button>
				{/each}
			</div>
		</div>
	{/each}
</div>

{#if showCreate}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4">
		<div class="w-full max-w-md rounded-2xl border border-white/[0.10] bg-[#0e1116] p-6">
			<h2 class="mb-5 text-xl font-semibold text-white">New task</h2>
			<form onsubmit={createTask} class="space-y-4">
				<div>
					<label
						for="task-title"
						class="mb-2 block text-[11px] font-mono uppercase tracking-[0.1em] text-white/50"
						>Title</label
					>
					<input
						id="task-title"
						type="text"
						bind:value={formTitle}
						required
						placeholder="What needs to be done?"
						class="w-full rounded-xl border border-white/10 bg-white/[0.03] px-4 py-3 text-white placeholder:text-white/25 focus:outline-none focus:ring-2 focus:ring-[#3fa9f5]/60"
					/>
				</div>
				<div>
					<label
						for="task-desc"
						class="mb-2 block text-[11px] font-mono uppercase tracking-[0.1em] text-white/50"
						>Description</label
					>
					<textarea
						id="task-desc"
						bind:value={formDescription}
						rows="3"
						placeholder="Optional"
						class="w-full resize-none rounded-xl border border-white/10 bg-white/[0.03] px-4 py-3 text-white placeholder:text-white/25 focus:outline-none focus:ring-2 focus:ring-[#3fa9f5]/60"
					></textarea>
				</div>
				<div class="grid grid-cols-2 gap-4">
					<div>
						<label
							for="task-status"
							class="mb-2 block text-[11px] font-mono uppercase tracking-[0.1em] text-white/50"
							>Column</label
						>
						<select
							id="task-status"
							bind:value={createStatus}
							class="w-full rounded-xl border border-white/10 bg-white/[0.03] px-4 py-3 text-white focus:outline-none focus:ring-2 focus:ring-[#3fa9f5]/60"
						>
							<option value="todo">To Do</option>
							<option value="in_progress">In Progress</option>
							<option value="done">Done</option>
						</select>
					</div>
					<div>
						<label
							for="task-priority"
							class="mb-2 block text-[11px] font-mono uppercase tracking-[0.1em] text-white/50"
							>Priority</label
						>
						<select
							id="task-priority"
							bind:value={formPriority}
							class="w-full rounded-xl border border-white/10 bg-white/[0.03] px-4 py-3 text-white focus:outline-none focus:ring-2 focus:ring-[#3fa9f5]/60"
						>
							<option value="low">Low</option>
							<option value="medium">Medium</option>
							<option value="high">High</option>
						</select>
					</div>
				</div>
				<div class="flex justify-end gap-2 pt-2">
					<button
						type="button"
						onclick={() => (showCreate = false)}
						class="rounded-full border border-white/10 px-4 py-2 text-sm font-medium text-white/70"
					>
						Cancel
					</button>
					<button
						type="submit"
						class="rounded-full bg-gradient-to-b from-[#4fb3f7] to-[#1c6ba3] px-4 py-2 text-sm font-medium text-white"
					>
						Create task
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
