<script lang="ts">
  import { onMount } from "svelte";
  import Modal from "$lib/Modal.svelte";
  import Pop from "$lib/Pop.svelte";
  import { API } from "$lib/api";

  type Project = {
    id: number;
    title: string;
  };

  let projects = $state<Project[]>([]);
  let show = $state(false);
  let selectedProject = $state<Project | null>(null);
  let showDeleteModal = $state(false);
  let showUpdateModal = $state(false);
  let updateTitle = $state("");

  function openDeleteModal(project: Project) {
    selectedProject = project;
    showDeleteModal = true;
  }

  function openUpdateModal(project: Project) {
    selectedProject = project;
    updateTitle = project.title;
    showUpdateModal = true;
  }

  function getAuthHeader() {
    const token = localStorage.getItem("token");
    if (!token) {
      window.location.href = "/login";
      return null;
    }
    return { "Authorization": `Bearer ${token}` };
  }

  async function deleteProject() {
    if (!selectedProject) return;
    const auth = getAuthHeader();
    if (!auth) return;

    try {
      const res = await fetch(`${API}/projects/${selectedProject.id}`, {
        method: "DELETE",
        headers: auth
      });
      if (!res.ok) throw new Error("Delete failed");

      projects = projects.filter(p => p.id !== selectedProject?.id);
      showDeleteModal = false;
      selectedProject = null;
    } catch (err) {
      console.error(err);
    }
  }

  async function updateProject() {
    if (!selectedProject) return;
    const auth = getAuthHeader();
    if (!auth) return;

    try {
      const response = await fetch(`${API}/projects/${selectedProject.id}`, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          ...auth
        },
        body: JSON.stringify({ title: updateTitle })
      });
      if (!response.ok) throw new Error("Update failed");

      projects = projects.map(p =>
        p.id === selectedProject?.id ? { ...p, title: updateTitle } : p
      );
      showUpdateModal = false;
      selectedProject = null;
    } catch (err) {
      console.error(err);
    }
  }

  async function getProjects() {
    const auth = getAuthHeader();
    if (!auth) return;

    try {
      const response = await fetch(`${API}/projects`, {
        method: "GET",
        headers: auth
      });
      if (!response.ok) throw new Error("Fetch failed");
      projects = await response.json();
    } catch (err) {
      console.error(err);
    }
  }

  onMount(() => {
    getProjects();
  });

  const open = () => (show = true);
  const close = () => (show = false);

  async function create(event: CustomEvent<{ title: string }>) {
    const auth = getAuthHeader();
    if (!auth) return;

    try {
      const response = await fetch(`${API}/project`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          ...auth
        },
        body: JSON.stringify(event.detail)
      });
      if (!response.ok) throw new Error("Creation failed");

      const p = await response.json();
      projects = [p, ...projects];
      show = false;
    } catch (err) {
      console.error(err);
    }
  }
</script>

<svelte:head>
  <title>Projects — Stratum</title>
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

<div class="font-display relative min-h-screen overflow-hidden bg-[#08090b] text-[#eef2f6] antialiased selection:bg-[#3fa9f5] selection:text-white">

  <!-- Scanlines + grain -->
  <div class="scanlines pointer-events-none fixed inset-0 z-40 opacity-50 mix-blend-overlay"></div>
  <div class="noise pointer-events-none fixed inset-0 z-40 opacity-[0.035] mix-blend-overlay"></div>

  <!-- Ambient gradient field -->
  <div class="pointer-events-none fixed inset-0">
    <div class="absolute left-1/2 top-[-14%] h-[760px] w-[1200px] -translate-x-1/2 rounded-full bg-[#3fa9f5] opacity-[0.14] blur-[190px]"></div>
    <div class="absolute bottom-[6%] right-[-8%] h-[420px] w-[420px] rounded-full bg-[#ff3366] opacity-[0.08] blur-[150px]"></div>
    <div class="absolute left-[-8%] top-[55%] h-[380px] w-[380px] rounded-full bg-[#3fa9f5] opacity-[0.07] blur-[140px]"></div>
  </div>

  <!-- Top bar, echoes the landing page navbar -->
  <nav class="sticky top-0 z-30 border-b border-white/[0.09] bg-[#08090b]/55 backdrop-blur-xl">
    <div class="mx-auto flex max-w-6xl items-center justify-between px-6 py-[18px]">
      <div class="flex items-center gap-2.5">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#5db9f7" stroke-width="1.8"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/></svg>
        <span class="text-[15px] font-semibold tracking-tight">Stratum</span>
      </div>

      <button
        class="rounded-full bg-gradient-to-b from-[#4fb3f7] to-[#1c6ba3] px-5 py-2.5 text-sm font-medium text-white shadow-[0_0_0_1px_rgba(255,255,255,0.08),0_4px_18px_-4px_rgba(63,169,245,0.6)] transition hover:shadow-[0_0_0_1px_rgba(255,255,255,0.14),0_6px_26px_-4px_rgba(63,169,245,1)]"
        onclick={open}>
        + New Project
      </button>
    </div>
  </nav>

  <div class="relative z-10 mx-auto max-w-6xl px-6 py-14">

    <p class="font-mono text-[11px] uppercase tracking-[0.25em] text-[#5db9f7]">
      Workspace
    </p>
    <h1 class="mt-4 text-3xl font-semibold tracking-tight sm:text-4xl">Projects</h1>

    <div class="mt-10">
      {#if projects.length === 0}
        <div class="flex flex-col items-center justify-center rounded-2xl border border-white/[0.09] bg-white/[0.02] py-20 text-center">
          <div class="flex h-10 w-10 items-center justify-center rounded-lg border border-white/10 bg-white/[0.03] text-[#5db9f7] mb-4">
            <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/></svg>
          </div>
          <p class="text-sm text-white/40">No projects found. Create one to get started!</p>
        </div>
      {:else}
        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3">
          {#each projects as p}
            <div class="group relative overflow-hidden rounded-2xl border border-white/[0.09] bg-white/[0.02] p-6 transition hover:border-[#3fa9f5]/40 flex flex-col justify-between">
              <div class="pointer-events-none absolute -right-10 -top-10 h-32 w-32 rounded-full bg-[#3fa9f5] opacity-0 blur-3xl transition group-hover:opacity-20"></div>

              <div class="flex items-start justify-between mb-6">
                <div class="flex h-9 w-9 items-center justify-center rounded-lg border border-white/10 bg-white/[0.03] text-[#5db9f7]">
                  <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/></svg>
                </div>
              </div>

              <div class="font-medium text-white truncate mb-6">{p.title}</div>

              <div class="flex gap-2 justify-end">
                <button
                  class="rounded-full border border-white/10 bg-white/[0.03] px-3.5 py-1.5 text-xs font-medium text-white/70 transition hover:border-white/25 hover:text-white"
                  onclick={() => openUpdateModal(p)}>
                  Update
                </button>
                <button
                  class="rounded-full border border-[#ff3366]/30 bg-[#ff3366]/10 px-3.5 py-1.5 text-xs font-medium text-[#ff8fa8] transition hover:bg-[#ff3366]/20"
                  onclick={() => openDeleteModal(p)}>
                  Delete
                </button>
              </div>
            </div>
          {/each}
        </div>
      {/if}

      {#if show}
        <Modal oncreate={create} onclose={close}/>
      {/if}
    </div>
  </div>
</div>

{#if showDeleteModal}
  <Pop bind:open={showDeleteModal}>
    <div class="font-display rounded-2xl border border-white/[0.10] bg-[#0e1116] p-6">
      <h2 class="text-xl font-semibold text-white mb-2">Delete project?</h2>
      <p class="text-sm text-white/50 mb-6">This action cannot be undone. This will permanently remove the project parameters.</p>
      <div class="flex justify-end gap-2">
        <button
          class="rounded-full border border-white/10 px-4 py-2 text-sm font-medium text-white/70 transition hover:border-white/25 hover:text-white"
          onclick={() => showDeleteModal = false}>
          Cancel
        </button>
        <button
          class="rounded-full bg-gradient-to-b from-[#ff5c7c] to-[#c21f45] px-4 py-2 text-sm font-medium text-white shadow-[0_0_0_1px_rgba(255,255,255,0.08),0_8px_24px_-6px_rgba(255,51,102,0.6)] transition hover:shadow-[0_0_0_1px_rgba(255,255,255,0.14),0_10px_30px_-6px_rgba(255,51,102,0.9)]"
          onclick={deleteProject}>
          Confirm delete
        </button>
      </div>
    </div>
  </Pop>
{/if}

{#if showUpdateModal}
  <Pop bind:open={showUpdateModal}>
    <div class="font-display rounded-2xl border border-white/[0.10] bg-[#0e1116] p-6">
      <h2 class="text-xl font-semibold text-white mb-5">Update project</h2>
      <div class="mb-6">
        <label class="block text-[11px] font-mono uppercase tracking-[0.1em] text-white/50 mb-2" for="update-title">
          Project title
        </label>
        <input
          id="update-title"
          class="w-full px-4 py-3 rounded-xl border border-white/10 bg-white/[0.03] text-white placeholder:text-white/25 focus:outline-none focus:ring-2 focus:ring-[#3fa9f5]/60 focus:border-transparent transition-all"
          bind:value={updateTitle}
          type="text"
          placeholder="Enter new title"
        />
      </div>
      <div class="flex justify-end gap-2">
        <button
          class="rounded-full border border-white/10 px-4 py-2 text-sm font-medium text-white/70 transition hover:border-white/25 hover:text-white"
          onclick={() => showUpdateModal = false}>
          Cancel
        </button>
        <button
          class="rounded-full bg-gradient-to-b from-[#4fb3f7] to-[#1c6ba3] px-4 py-2 text-sm font-medium text-white shadow-[0_0_0_1px_rgba(255,255,255,0.08),0_8px_24px_-6px_rgba(63,169,245,0.6)] transition hover:shadow-[0_0_0_1px_rgba(255,255,255,0.14),0_10px_30px_-6px_rgba(63,169,245,1)]"
          onclick={updateProject}>
          Save changes
        </button>
      </div>
    </div>
  </Pop>
{/if}