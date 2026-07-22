<script lang="ts">
	import { API } from "$lib/api";

	let email = $state("");
	let password = $state("");
	let errorMessage = $state(""); // State to show errors to the user

	async function login(event: SubmitEvent) {
		event.preventDefault();
		errorMessage = ""; // Clear previous errors

		try {
			const response = await fetch(`${API}/login`, {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify({ email, password })
			});

			const resp = await response.json();

			if (!response.ok) {
				errorMessage = resp.message || "Invalid email or password.";
				console.log("Login failed");
				return;
			}

			// Check if token actually exists in response
			if (resp.token) {
				localStorage.setItem("token", resp.token);
				window.location.href = "/dashboard";
			} else {
				errorMessage = "Authentication failed. No token received.";
			}

		} catch (error) {
			console.error("Error during login:", error);
			errorMessage = "Something went wrong. Please try again later.";
		}
	}
</script>

<svelte:head>
	<title>Log in — Stratum</title>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="true" />
	<link
		href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<style>
	.font-display {
		font-family: 'Space Grotesk', system-ui, sans-serif;
	}
	.font-mono {
		font-family: 'JetBrains Mono', ui-monospace, monospace;
	}
	.noise {
		background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='180' height='180'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.85' numOctaves='2' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E");
	}
	.scanlines {
		background: repeating-linear-gradient(
			to bottom,
			rgba(255, 255, 255, 0.018) 0px,
			rgba(255, 255, 255, 0.018) 1px,
			transparent 1px,
			transparent 3px
		);
	}

	@media (prefers-reduced-motion: reduce) {
		* {
			animation-duration: 0.001ms !important;
			animation-iteration-count: 1 !important;
			transition-duration: 0.001ms !important;
		}
	}
</style>

<div class="font-display relative min-h-screen overflow-hidden bg-[#08090b] text-[#eef2f6] antialiased selection:bg-[#3fa9f5] selection:text-white flex items-center justify-center px-4">

	<!-- Scanlines + grain -->
	<div class="scanlines pointer-events-none fixed inset-0 z-40 opacity-50 mix-blend-overlay"></div>
	<div class="noise pointer-events-none fixed inset-0 z-40 opacity-[0.035] mix-blend-overlay"></div>

	<!-- Ambient gradient field -->
	<div class="pointer-events-none fixed inset-0">
		<div class="absolute left-1/2 top-[-14%] h-[760px] w-[1200px] -translate-x-1/2 rounded-full bg-[#3fa9f5] opacity-[0.14] blur-[190px]"></div>
		<div class="absolute bottom-[6%] right-[-8%] h-[420px] w-[420px] rounded-full bg-[#ff3366] opacity-[0.08] blur-[150px]"></div>
		<div class="absolute left-[-8%] top-[55%] h-[380px] w-[380px] rounded-full bg-[#3fa9f5] opacity-[0.07] blur-[140px]"></div>
	</div>

	<!-- Brand mark, top-left, echoes the landing page navbar -->
	<a href="/" class="absolute left-6 top-6 z-10 flex items-center gap-2.5">
		<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#5db9f7" stroke-width="1.8"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/></svg>
		<span class="text-[15px] font-semibold tracking-tight">Stratum</span>
	</a>

	<!-- Signature "linked work" style window, now framing the form -->
	<div class="relative z-10 w-full max-w-md" style="perspective: 1400px;">

		<div class="pointer-events-none absolute -inset-x-8 -inset-y-6 rounded-[32px] bg-[#3fa9f5] opacity-[0.16] blur-[80px]"></div>

		<div
			class="relative rounded-2xl border border-white/[0.10] bg-gradient-to-b from-white/[0.05] to-white/[0.01] shadow-[0_40px_100px_-20px_rgba(0,0,0,0.8)] backdrop-blur-2xl overflow-hidden">

			<!-- window chrome, matches the hero demo window -->
			<div class="flex items-center gap-1.5 border-b border-white/[0.09] px-5 py-3.5">
				<span class="h-2.5 w-2.5 rounded-full bg-white/15"></span>
				<span class="h-2.5 w-2.5 rounded-full bg-white/15"></span>
				<span class="h-2.5 w-2.5 rounded-full bg-white/15"></span>
				<span class="font-mono ml-3 text-[11px] text-white/30">stratum.app/login</span>
			</div>

			<div class="px-8 py-9">

				<div class="text-center mb-8">
					<p class="font-mono text-[11px] uppercase tracking-[0.25em] text-[#5db9f7]">
						Welcome back
					</p>
					<h2 class="mt-4 text-3xl font-semibold tracking-tight text-white">Sign in</h2>
					<p class="mt-2 text-sm text-white/50">Please enter your details to sign in.</p>
				</div>

				{#if errorMessage}
					<div class="mb-5 px-4 py-3 rounded-xl border border-[#ff3366]/30 bg-[#ff3366]/10 text-[#ff8fa8] text-sm">
						{errorMessage}
					</div>
				{/if}

				<form onsubmit={login} class="space-y-5">
					<div>
						<label for="login-email" class="block text-[11px] font-mono uppercase tracking-[0.1em] text-white/50 mb-2">Email address</label>
						<input
							id="login-email"
							type="email"
							placeholder="you@example.com"
							bind:value={email}
							required
							class="w-full px-4 py-3 rounded-xl border border-white/10 bg-white/[0.03] text-white placeholder:text-white/25 focus:outline-none focus:ring-2 focus:ring-[#3fa9f5]/60 focus:border-transparent transition-all"
						/>
					</div>

					<div>
						<label for="login-password" class="block text-[11px] font-mono uppercase tracking-[0.1em] text-white/50 mb-2">Password</label>
						<input
							id="login-password"
							type="password"
							placeholder="••••••••"
							bind:value={password}
							required
							class="w-full px-4 py-3 rounded-xl border border-white/10 bg-white/[0.03] text-white placeholder:text-white/25 focus:outline-none focus:ring-2 focus:ring-[#3fa9f5]/60 focus:border-transparent transition-all"
						/>
					</div>

					<button
						type="submit"
						class="w-full rounded-full bg-gradient-to-b from-[#4fb3f7] to-[#1c6ba3] px-4 py-3.5 text-sm font-medium text-white shadow-[0_0_0_1px_rgba(255,255,255,0.08),0_8px_30px_-6px_rgba(63,169,245,0.7)] transition hover:shadow-[0_0_0_1px_rgba(255,255,255,0.14),0_10px_40px_-6px_rgba(63,169,245,1)] mt-2">
						Sign in
					</button>
				</form>

				<p class="text-center text-sm text-white/50 mt-6">
					Don't have an account?
					<a href="/register" class="text-[#5db9f7] hover:text-white transition font-medium">Sign up</a>
				</p>

			</div>

			<div class="flex items-center justify-center gap-2 border-t border-white/[0.09] px-5 py-3">
				<span class="h-1 w-1 rounded-full bg-[#5db9f7]"></span>
				<span class="font-mono text-[10px] uppercase tracking-[0.14em] text-white/30">Everything linked, automatically</span>
				<span class="h-1 w-1 rounded-full bg-[#5db9f7]"></span>
			</div>

		</div>
	</div>

</div>