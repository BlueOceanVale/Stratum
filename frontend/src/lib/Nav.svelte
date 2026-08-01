<script lang="ts">
	import { logout } from '$lib/api';

	let { active = '' }: { active?: 'dashboard' | 'clients' | 'members' | 'workspaces' | '' } =
		$props();

	const links: { href: string; label: string; key: string }[] = [
		{ href: '/dashboard', label: 'Dashboard', key: 'dashboard' },
		{ href: '/clients', label: 'Clients', key: 'clients' },
		{ href: '/members', label: 'Members', key: 'members' },
		{ href: '/workspaces', label: 'Workspaces', key: 'workspaces' }
	];
</script>

<nav class="sticky top-0 z-30 border-b border-white/[0.09] bg-[#08090b]/55 backdrop-blur-xl">
	<div class="mx-auto flex max-w-6xl items-center justify-between px-6 py-[18px]">
		<a href="/dashboard" class="flex items-center gap-2.5">
			<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#5db9f7" stroke-width="1.8"
				><polygon points="12 2 2 7 12 12 22 7 12 2" /><polyline points="2 17 12 22 22 17" /><polyline
					points="2 12 12 17 22 12"
				/></svg
			>
			<span class="text-[15px] font-semibold tracking-tight">Stratum</span>
		</a>

		<div class="flex items-center gap-1">
			{#each links as link}
				<a
					href={link.href}
					class="rounded-full px-4 py-2 font-mono text-xs uppercase tracking-wider transition {active ===
					link.key
						? 'bg-white/[0.06] text-white'
						: 'text-white/50 hover:text-white/80'}"
				>
					{link.label}
				</a>
			{/each}
			<button
				onclick={logout}
				class="ml-2 rounded-full border border-white/10 bg-white/[0.03] px-4 py-2 font-mono text-xs uppercase tracking-wider text-white/50 transition hover:border-[#ff3366]/30 hover:text-[#ff8fa8]"
			>
				Log out
			</button>
		</div>
	</div>
</nav>
