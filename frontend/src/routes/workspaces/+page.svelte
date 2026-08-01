<script lang="ts">
	import { onMount } from 'svelte';
	import { API, requireAuth, setWorkspaceId, type Workspace } from '$lib/api';
	import Nav from '$lib/Nav.svelte';

	let workspaces = $state<Workspace[]>([]);
	let loading = $state(true);
	let errorMessage = $state('');

	let showCreate = $state(false);
	let newTitle = $state('');
	let newDescription = $state('');
	let newTag = $state('');

	async function loadWorkspaces() {
		const auth = requireAuth();
		if (!auth) return;

		loading = true;
		try {
			const res = await fetch(`${API}/workspaces`, { headers: auth });
			if (!res.ok) throw new Error('Failed to load workspaces');
			workspaces = await res.json();
		} catch (err) {
			console.error(err);
			errorMessage = 'Could not load your workspaces.';
		} finally {
			loading = false;
		}
	}

	async function createWorkspace(event: SubmitEvent) {
		event.preventDefault();
		const auth = requireAuth();
		if (!auth || !newTitle.trim()) return;

		try {
			const res = await fetch(`${API}/workspaces`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json', ...auth },
				body: JSON.stringify({
					title: newTitle,
					description: newDescription || null,
					tag: newTag || null
				})
			});
			if (!res.ok) throw new Error('Failed to create workspace');

			newTitle = '';
			newDescription = '';
			newTag = '';
			showCreate = false;
			await loadWorkspaces();
		} catch (err) {
			console.error(err);
			errorMessage = 'Could not create workspace.';
		}
	}

	function selectWorkspace(ws: Workspace) {
		setWorkspaceId(ws.id);
		window.location.href = '/dashboard';
	}

	onMount(loadWorkspaces);
</script>

<svelte:head>
	<title>Workspaces — Stratum</title>
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

	<Nav active="workspaces" />

	<div class="relative z-10 mx-auto max-w-6xl px-6 py-10">
		<p class="font-mono text-[11px] tracking-[0.25em] text-[#5db9f7] uppercase">Your Spaces</p>
		<div class="mt-2 flex items-center justify-between">
			<h1 class="text-3xl font-semibold tracking-tight sm:text-4xl">Workspaces</h1>
			<button
				onclick={() => (showCreate = true)}
				class="rounded-full bg-gradient-to-b from-[#4fb3f7] to-[#1c6ba3] px-5 py-2.5 text-sm font-medium text-white shadow-[0_4px_18px_-4px_rgba(63,169,245,0.6)] transition hover:shadow-[0_6px_26px_-4px_rgba(63,169,245,1)]"
			>
				+ New Workspace
			</button>
		</div>

		{#if errorMessage}
			<div
				class="mt-6 rounded-xl border border-[#ff3366]/30 bg-[#ff3366]/10 px-4 py-3 text-sm text-[#ff8fa8]"
			>
				{errorMessage}
			</div>
		{/if}

		<div class="mt-8">
			{#if loading}
				<p class="text-sm text-white/40">Loading workspaces…</p>
			{:else if workspaces.length === 0}
				<div
					class="flex flex-col items-center justify-center rounded-2xl border border-white/[0.09] bg-white/[0.02] py-16 text-center"
				>
					<p class="text-sm text-white/40">No workspaces yet. Create one to get started.</p>
				</div>
			{:else}
				<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3">
					{#each workspaces as ws}
						<button
							onclick={() => selectWorkspace(ws)}
							class="group relative overflow-hidden rounded-2xl border border-white/[0.09] bg-white/[0.02] p-6 text-left transition hover:border-[#3fa9f5]/40"
						>
							{#if ws.tag}
								<span
									class="mb-3 inline-block rounded-full bg-white/[0.06] px-2.5 py-1 font-mono text-[10px] tracking-wider text-[#5db9f7] uppercase"
								>
									{ws.tag}
								</span>
							{/if}
							<div class="truncate text-lg font-medium text-white">{ws.title}</div>
							{#if ws.description}
								<p class="mt-1 line-clamp-2 text-sm text-white/50">{ws.description}</p>
							{/if}
							<div
								class="mt-4 font-mono text-[11px] tracking-wider text-white/30 uppercase group-hover:text-[#5db9f7]"
							>
								Enter workspace →
							</div>
						</button>
					{/each}
				</div>
			{/if}
		</div>
	</div>
</div>

{#if showCreate}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4">
		<div
			class="font-display w-full max-w-md rounded-2xl border border-white/[0.10] bg-[#0e1116] p-6"
		>
			<h2 class="mb-5 text-xl font-semibold text-white">Create workspace</h2>
			<form onsubmit={createWorkspace} class="space-y-4">
				<div>
					<label
						for="ws-title"
						class="mb-2 block font-mono text-[11px] tracking-[0.1em] text-white/50 uppercase"
						>Title</label
					>
					<input
						id="ws-title"
						type="text"
						bind:value={newTitle}
						required
						placeholder="Acme Engineering"
						class="w-full rounded-xl border border-white/10 bg-white/[0.03] px-4 py-3 text-white placeholder:text-white/25 focus:ring-2 focus:ring-[#3fa9f5]/60 focus:outline-none"
					/>
				</div>
				<div>
					<label
						for="ws-desc"
						class="mb-2 block font-mono text-[11px] tracking-[0.1em] text-white/50 uppercase"
						>Description</label
					>
					<input
						id="ws-desc"
						type="text"
						bind:value={newDescription}
						placeholder="Optional"
						class="w-full rounded-xl border border-white/10 bg-white/[0.03] px-4 py-3 text-white placeholder:text-white/25 focus:ring-2 focus:ring-[#3fa9f5]/60 focus:outline-none"
					/>
				</div>
				<div>
					<label
						for="ws-tag"
						class="mb-2 block font-mono text-[11px] tracking-[0.1em] text-white/50 uppercase"
						>Tag</label
					>
					<input
						id="ws-tag"
						type="text"
						bind:value={newTag}
						placeholder="Optional, e.g. client-work"
						class="w-full rounded-xl border border-white/10 bg-white/[0.03] px-4 py-3 text-white placeholder:text-white/25 focus:ring-2 focus:ring-[#3fa9f5]/60 focus:outline-none"
					/>
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
						Create
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
