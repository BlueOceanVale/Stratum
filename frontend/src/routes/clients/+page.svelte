<script lang="ts">
	import { onMount } from 'svelte';
	import { API, requireAuth, getWorkspaceId, type Client } from '$lib/api';
	import Nav from '$lib/Nav.svelte';

	let clients = $state<Client[]>([]);
	let workspaceId = $state('');
	let loading = $state(true);
	let errorMessage = $state('');

	let showCreate = $state(false);
	let showEdit = $state(false);
	let showDelete = $state(false);
	let selected = $state<Client | null>(null);

	let formName = $state('');
	let formEmail = $state('');
	let formCompany = $state('');
	let formStatus = $state('active');

	function resetForm() {
		formName = '';
		formEmail = '';
		formCompany = '';
		formStatus = 'active';
	}

	async function loadClients() {
		const auth = requireAuth();
		const wsId = getWorkspaceId();
		if (!auth) return;
		if (!wsId) {
			window.location.href = '/workspaces';
			return;
		}
		workspaceId = wsId;

		loading = true;
		try {
			const res = await fetch(`${API}/workspaces/${workspaceId}/clients`, { headers: auth });
			if (!res.ok) throw new Error('Failed to load clients');
			clients = await res.json();
		} catch (err) {
			console.error(err);
			errorMessage = 'Could not load clients.';
		} finally {
			loading = false;
		}
	}

	async function createClient(event: SubmitEvent) {
		event.preventDefault();
		const auth = requireAuth();
		if (!auth || !workspaceId || !formName.trim()) return;

		try {
			const res = await fetch(`${API}/workspaces/${workspaceId}/clients`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json', ...auth },
				body: JSON.stringify({
					name: formName,
					email: formEmail || null,
					company: formCompany || null,
					status: formStatus
				})
			});
			if (!res.ok) throw new Error('Failed to create client');

			resetForm();
			showCreate = false;
			await loadClients();
		} catch (err) {
			console.error(err);
			errorMessage = 'Could not create client.';
		}
	}

	function openEdit(client: Client) {
		selected = client;
		formName = client.name;
		formEmail = client.email ?? '';
		formCompany = client.company ?? '';
		formStatus = client.status;
		showEdit = true;
	}

	async function updateClient(event: SubmitEvent) {
		event.preventDefault();
		const auth = requireAuth();
		if (!auth || !selected || !workspaceId) return;

		try {
			const res = await fetch(`${API}/workspaces/${workspaceId}/clients/${selected.id}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json', ...auth },
				body: JSON.stringify({
					name: formName,
					email: formEmail || null,
					company: formCompany || null,
					status: formStatus
				})
			});
			if (!res.ok) throw new Error('Failed to update client');

			showEdit = false;
			selected = null;
			await loadClients();
		} catch (err) {
			console.error(err);
			errorMessage = 'Could not update client.';
		}
	}

	function openDelete(client: Client) {
		selected = client;
		showDelete = true;
	}

	async function deleteClient() {
		const auth = requireAuth();
		if (!auth || !selected || !workspaceId) return;

		try {
			const res = await fetch(`${API}/workspaces/${workspaceId}/clients/${selected.id}`, {
				method: 'DELETE',
				headers: auth
			});
			if (!res.ok) throw new Error('Failed to delete client');

			clients = clients.filter((c) => c.id !== selected?.id);
			showDelete = false;
			selected = null;
		} catch (err) {
			console.error(err);
			errorMessage = 'Could not delete client.';
		}
	}

	const statusColors: Record<string, string> = {
		active: 'text-[#42d6a4] bg-[#42d6a4]/10 border-[#42d6a4]/30',
		inactive: 'text-white/40 bg-white/[0.03] border-white/10',
		prospect: 'text-[#f7ad3d] bg-[#f7ad3d]/10 border-[#f7ad3d]/30'
	};

	onMount(loadClients);
</script>

<svelte:head>
	<title>Clients — Stratum</title>
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

	<Nav active="clients" />

	<div class="relative z-10 mx-auto max-w-6xl px-6 py-10">
		<p class="font-mono text-[11px] tracking-[0.25em] text-[#5db9f7] uppercase">Relationships</p>
		<div class="mt-2 flex items-center justify-between">
			<h1 class="text-3xl font-semibold tracking-tight sm:text-4xl">Clients</h1>
			<button
				onclick={() => {
					resetForm();
					showCreate = true;
				}}
				class="rounded-full bg-gradient-to-b from-[#4fb3f7] to-[#1c6ba3] px-5 py-2.5 text-sm font-medium text-white shadow-[0_4px_18px_-4px_rgba(63,169,245,0.6)] transition hover:shadow-[0_6px_26px_-4px_rgba(63,169,245,1)]"
			>
				+ New Client
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
				<p class="text-sm text-white/40">Loading clients…</p>
			{:else if clients.length === 0}
				<div
					class="flex flex-col items-center justify-center rounded-2xl border border-white/[0.09] bg-white/[0.02] py-16 text-center"
				>
					<p class="text-sm text-white/40">No clients yet. Add one to get started.</p>
				</div>
			{:else}
				<div class="overflow-x-auto rounded-2xl border border-white/[0.09] bg-white/[0.02]">
					<table class="w-full text-left text-sm text-white/80">
						<thead class="border-b border-white/[0.09] font-mono text-xs text-white/40 uppercase">
							<tr>
								<th class="px-6 py-4">Name</th>
								<th class="px-6 py-4">Company</th>
								<th class="px-6 py-4">Email</th>
								<th class="px-6 py-4">Status</th>
								<th class="px-6 py-4"></th>
							</tr>
						</thead>
						<tbody class="divide-y divide-white/[0.05]">
							{#each clients as client}
								<tr class="hover:bg-white/[0.02]">
									<td class="px-6 py-4 font-medium text-white">{client.name}</td>
									<td class="px-6 py-4 text-white/50">{client.company ?? '—'}</td>
									<td class="px-6 py-4 text-white/50">{client.email ?? '—'}</td>
									<td class="px-6 py-4">
										<span
											class="rounded-full border px-2.5 py-1 font-mono text-[11px] uppercase {statusColors[
												client.status
											] ?? 'border-white/10 bg-white/[0.03] text-white/50'}"
										>
											{client.status}
										</span>
									</td>
									<td class="px-6 py-4">
										<div class="flex justify-end gap-2">
											<button
												class="rounded-full border border-white/10 bg-white/[0.03] px-3.5 py-1.5 text-xs font-medium text-white/70 transition hover:border-white/25 hover:text-white"
												onclick={() => openEdit(client)}
											>
												Edit
											</button>
											<button
												class="rounded-full border border-[#ff3366]/30 bg-[#ff3366]/10 px-3.5 py-1.5 text-xs font-medium text-[#ff8fa8] transition hover:bg-[#ff3366]/20"
												onclick={() => openDelete(client)}
											>
												Delete
											</button>
										</div>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			{/if}
		</div>
	</div>
</div>

{#if showCreate || showEdit}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4">
		<div
			class="font-display w-full max-w-md rounded-2xl border border-white/[0.10] bg-[#0e1116] p-6"
		>
			<h2 class="mb-5 text-xl font-semibold text-white">
				{showEdit ? 'Update client' : 'New client'}
			</h2>
			<form onsubmit={showEdit ? updateClient : createClient} class="space-y-4">
				<div>
					<label
						for="c-name"
						class="mb-2 block font-mono text-[11px] tracking-[0.1em] text-white/50 uppercase"
						>Name</label
					>
					<input
						id="c-name"
						type="text"
						bind:value={formName}
						required
						placeholder="Jane Doe"
						class="w-full rounded-xl border border-white/10 bg-white/[0.03] px-4 py-3 text-white placeholder:text-white/25 focus:ring-2 focus:ring-[#3fa9f5]/60 focus:outline-none"
					/>
				</div>
				<div>
					<label
						for="c-company"
						class="mb-2 block font-mono text-[11px] tracking-[0.1em] text-white/50 uppercase"
						>Company</label
					>
					<input
						id="c-company"
						type="text"
						bind:value={formCompany}
						placeholder="Optional"
						class="w-full rounded-xl border border-white/10 bg-white/[0.03] px-4 py-3 text-white placeholder:text-white/25 focus:ring-2 focus:ring-[#3fa9f5]/60 focus:outline-none"
					/>
				</div>
				<div>
					<label
						for="c-email"
						class="mb-2 block font-mono text-[11px] tracking-[0.1em] text-white/50 uppercase"
						>Email</label
					>
					<input
						id="c-email"
						type="email"
						bind:value={formEmail}
						placeholder="Optional"
						class="w-full rounded-xl border border-white/10 bg-white/[0.03] px-4 py-3 text-white placeholder:text-white/25 focus:ring-2 focus:ring-[#3fa9f5]/60 focus:outline-none"
					/>
				</div>
				<div>
					<label
						for="c-status"
						class="mb-2 block font-mono text-[11px] tracking-[0.1em] text-white/50 uppercase"
						>Status</label
					>
					<select
						id="c-status"
						bind:value={formStatus}
						class="w-full rounded-xl border border-white/10 bg-white/[0.03] px-4 py-3 text-white focus:ring-2 focus:ring-[#3fa9f5]/60 focus:outline-none"
					>
						<option value="active">Active</option>
						<option value="prospect">Prospect</option>
						<option value="inactive">Inactive</option>
					</select>
				</div>
				<div class="flex justify-end gap-2 pt-2">
					<button
						type="button"
						onclick={() => {
							showCreate = false;
							showEdit = false;
						}}
						class="rounded-full border border-white/10 px-4 py-2 text-sm font-medium text-white/70"
					>
						Cancel
					</button>
					<button
						type="submit"
						class="rounded-full bg-gradient-to-b from-[#4fb3f7] to-[#1c6ba3] px-4 py-2 text-sm font-medium text-white"
					>
						{showEdit ? 'Save changes' : 'Create'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

{#if showDelete}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4">
		<div class="font-display rounded-2xl border border-white/[0.10] bg-[#0e1116] p-6">
			<h2 class="mb-2 text-xl font-semibold text-white">Delete client?</h2>
			<p class="mb-6 text-sm text-white/50">This action cannot be undone.</p>
			<div class="flex justify-end gap-2">
				<button
					class="rounded-full border border-white/10 px-4 py-2 text-sm font-medium text-white/70"
					onclick={() => (showDelete = false)}
				>
					Cancel
				</button>
				<button
					class="rounded-full bg-gradient-to-b from-[#ff5c7c] to-[#c21f45] px-4 py-2 text-sm font-medium text-white"
					onclick={deleteClient}
				>
					Confirm delete
				</button>
			</div>
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
