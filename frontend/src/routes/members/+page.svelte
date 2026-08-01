<script lang="ts">
	import { onMount } from 'svelte';
	import { API, requireAuth, getWorkspaceId } from '$lib/api';
	import Nav from '$lib/Nav.svelte';

	let workspaceId = $state('');
	let statusMessage = $state('');
	let errorMessage = $state('');

	// Locally-tracked roster: the backend doesn't expose a "list members" endpoint,
	// so this page can add/update/remove members but can't fetch who's already there.
	// Anyone added successfully during this session is appended here for reference.
	type TrackedMember = { userId: string; role: string };
	let tracked = $state<TrackedMember[]>([]);

	let addUserId = $state('');
	let addRole = $state('member');

	let roleTargetId = $state('');
	let roleValue = $state('member');

	let removeTargetId = $state('');

	function checkAuth() {
		const auth = requireAuth();
		const wsId = getWorkspaceId();
		if (!auth) return null;
		if (!wsId) {
			window.location.href = '/workspaces';
			return null;
		}
		workspaceId = wsId;
		return auth;
	}

	async function addMember(event: SubmitEvent) {
		event.preventDefault();
		errorMessage = '';
		statusMessage = '';
		const auth = checkAuth();
		if (!auth || !addUserId.trim()) return;

		try {
			const res = await fetch(`${API}/workspaces/${workspaceId}/members`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json', ...auth },
				body: JSON.stringify({ user_id: addUserId, role: addRole })
			});
			const body = await res.json().catch(() => ({}));
			if (!res.ok) throw new Error(body.error || 'Failed to add member');

			tracked = [
				...tracked.filter((m) => m.userId !== addUserId),
				{ userId: addUserId, role: addRole }
			];
			statusMessage = `Added member ${addUserId} as ${addRole}.`;
			addUserId = '';
			addRole = 'member';
		} catch (err) {
			errorMessage = err instanceof Error ? err.message : 'Could not add member.';
		}
	}

	async function updateRole(event: SubmitEvent) {
		event.preventDefault();
		errorMessage = '';
		statusMessage = '';
		const auth = checkAuth();
		if (!auth || !roleTargetId.trim()) return;

		try {
			const res = await fetch(`${API}/workspaces/${workspaceId}/members/${roleTargetId}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json', ...auth },
				body: JSON.stringify({ role: roleValue })
			});
			const body = await res.json().catch(() => ({}));
			if (!res.ok) throw new Error(body.error || 'Failed to update role');

			tracked = tracked.map((m) => (m.userId === roleTargetId ? { ...m, role: roleValue } : m));
			statusMessage = `Updated ${roleTargetId} to ${roleValue}.`;
			roleTargetId = '';
			roleValue = 'member';
		} catch (err) {
			errorMessage = err instanceof Error ? err.message : 'Could not update role.';
		}
	}

	async function removeMember(event: SubmitEvent) {
		event.preventDefault();
		errorMessage = '';
		statusMessage = '';
		const auth = checkAuth();
		if (!auth || !removeTargetId.trim()) return;

		try {
			const res = await fetch(`${API}/workspaces/${workspaceId}/members/${removeTargetId}`, {
				method: 'DELETE',
				headers: auth
			});
			const body = await res.json().catch(() => ({}));
			if (!res.ok) throw new Error(body.error || 'Failed to remove member');

			tracked = tracked.filter((m) => m.userId !== removeTargetId);
			statusMessage = `Removed ${removeTargetId} from the workspace.`;
			removeTargetId = '';
		} catch (err) {
			errorMessage = err instanceof Error ? err.message : 'Could not remove member.';
		}
	}

	onMount(checkAuth);
</script>

<svelte:head>
	<title>Members — Stratum</title>
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

	<Nav active="members" />

	<div class="relative z-10 mx-auto max-w-6xl px-6 py-10">
		<p class="font-mono text-[11px] tracking-[0.25em] text-[#5db9f7] uppercase">Access Control</p>
		<h1 class="mt-2 text-3xl font-semibold tracking-tight sm:text-4xl">Members</h1>
		<p class="mt-2 max-w-xl text-sm text-white/50">
			Add members by their user ID, adjust roles, or remove access. Only owners and admins can
			manage members.
		</p>

		{#if errorMessage}
			<div
				class="mt-6 rounded-xl border border-[#ff3366]/30 bg-[#ff3366]/10 px-4 py-3 text-sm text-[#ff8fa8]"
			>
				{errorMessage}
			</div>
		{/if}
		{#if statusMessage}
			<div
				class="mt-6 rounded-xl border border-[#42d6a4]/30 bg-[#42d6a4]/10 px-4 py-3 text-sm text-[#8ef0c9]"
			>
				{statusMessage}
			</div>
		{/if}

		<div class="mt-8 grid grid-cols-1 gap-6 lg:grid-cols-3">
			<!-- Add member -->
			<div class="rounded-2xl border border-white/[0.09] bg-white/[0.02] p-6">
				<h2 class="mb-4 text-sm font-semibold text-white">Add member</h2>
				<form onsubmit={addMember} class="space-y-3">
					<div>
						<label
							for="add-user-id"
							class="mb-1.5 block font-mono text-[11px] tracking-[0.1em] text-white/50 uppercase"
							>User ID</label
						>
						<input
							id="add-user-id"
							type="text"
							bind:value={addUserId}
							required
							placeholder="UUID of the user"
							class="w-full rounded-xl border border-white/10 bg-white/[0.03] px-4 py-2.5 text-sm text-white placeholder:text-white/25 focus:ring-2 focus:ring-[#3fa9f5]/60 focus:outline-none"
						/>
					</div>
					<div>
						<label
							for="add-role"
							class="mb-1.5 block font-mono text-[11px] tracking-[0.1em] text-white/50 uppercase"
							>Role</label
						>
						<select
							id="add-role"
							bind:value={addRole}
							class="w-full rounded-xl border border-white/10 bg-white/[0.03] px-4 py-2.5 text-sm text-white focus:ring-2 focus:ring-[#3fa9f5]/60 focus:outline-none"
						>
							<option value="member">Member</option>
							<option value="admin">Admin</option>
						</select>
					</div>
					<button
						type="submit"
						class="w-full rounded-full bg-gradient-to-b from-[#4fb3f7] to-[#1c6ba3] px-4 py-2.5 text-sm font-medium text-white"
					>
						Add to workspace
					</button>
				</form>
			</div>

			<!-- Update role -->
			<div class="rounded-2xl border border-white/[0.09] bg-white/[0.02] p-6">
				<h2 class="mb-4 text-sm font-semibold text-white">Update role</h2>
				<form onsubmit={updateRole} class="space-y-3">
					<div>
						<label
							for="role-user-id"
							class="mb-1.5 block font-mono text-[11px] tracking-[0.1em] text-white/50 uppercase"
							>User ID</label
						>
						<input
							id="role-user-id"
							type="text"
							bind:value={roleTargetId}
							required
							placeholder="UUID of the user"
							class="w-full rounded-xl border border-white/10 bg-white/[0.03] px-4 py-2.5 text-sm text-white placeholder:text-white/25 focus:ring-2 focus:ring-[#3fa9f5]/60 focus:outline-none"
						/>
					</div>
					<div>
						<label
							for="role-value"
							class="mb-1.5 block font-mono text-[11px] tracking-[0.1em] text-white/50 uppercase"
							>New role</label
						>
						<select
							id="role-value"
							bind:value={roleValue}
							class="w-full rounded-xl border border-white/10 bg-white/[0.03] px-4 py-2.5 text-sm text-white focus:ring-2 focus:ring-[#3fa9f5]/60 focus:outline-none"
						>
							<option value="member">Member</option>
							<option value="admin">Admin</option>
						</select>
					</div>
					<p class="text-[11px] text-white/30">Only the workspace owner can change roles.</p>
					<button
						type="submit"
						class="w-full rounded-full bg-gradient-to-b from-[#4fb3f7] to-[#1c6ba3] px-4 py-2.5 text-sm font-medium text-white"
					>
						Update role
					</button>
				</form>
			</div>

			<!-- Remove member -->
			<div class="rounded-2xl border border-white/[0.09] bg-white/[0.02] p-6">
				<h2 class="mb-4 text-sm font-semibold text-white">Remove member</h2>
				<form onsubmit={removeMember} class="space-y-3">
					<div>
						<label
							for="remove-user-id"
							class="mb-1.5 block font-mono text-[11px] tracking-[0.1em] text-white/50 uppercase"
							>User ID</label
						>
						<input
							id="remove-user-id"
							type="text"
							bind:value={removeTargetId}
							required
							placeholder="UUID of the user"
							class="w-full rounded-xl border border-white/10 bg-white/[0.03] px-4 py-2.5 text-sm text-white placeholder:text-white/25 focus:ring-2 focus:ring-[#3fa9f5]/60 focus:outline-none"
						/>
					</div>
					<p class="text-[11px] text-white/30">
						You can remove yourself, or remove others if you're an owner/admin. The workspace owner
						can't be removed.
					</p>
					<button
						type="submit"
						class="w-full rounded-full border border-[#ff3366]/30 bg-[#ff3366]/10 px-4 py-2.5 text-sm font-medium text-[#ff8fa8] transition hover:bg-[#ff3366]/20"
					>
						Remove from workspace
					</button>
				</form>
			</div>
		</div>

		{#if tracked.length > 0}
			<div class="mt-8">
				<h2 class="mb-4 text-sm font-semibold text-white">Changes made this session</h2>
				<div class="overflow-x-auto rounded-2xl border border-white/[0.09] bg-white/[0.02]">
					<table class="w-full text-left text-sm text-white/80">
						<thead class="border-b border-white/[0.09] font-mono text-xs text-white/40 uppercase">
							<tr>
								<th class="px-6 py-3">User ID</th>
								<th class="px-6 py-3">Role</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-white/[0.05]">
							{#each tracked as m}
								<tr>
									<td class="px-6 py-3 font-mono text-xs text-white/70">{m.userId}</td>
									<td class="px-6 py-3">{m.role}</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.font-display {
		font-family: 'Space Grotesk', system-ui, sans-serif;
	}
	.font-mono {
		font-family: 'JetBrains Mono', ui-monospace, monospace;
	}
</style>
