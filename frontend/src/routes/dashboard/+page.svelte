<script lang="ts">
  import { onMount } from "svelte";
  import Modal from "$lib/Modal.svelte";
  import Pop from "$lib/Pop.svelte";
  import { API } from "$lib/api";

  type ClientSummary = {
    id: string;
    name: string;
    company: string | null;
    total_projects: number;
    total_tasks: number;
  };

  type DashboardStats = {
    total_clients: number;
    total_projects: number;
    total_tasks: number;
    pending_tasks: number;
    completed_tasks: number;
    clients: ClientSummary[];
  };

  type Project = {
    id: string;
    name: string;
    client_id?: string;
  };

  // State Management
  let dashboardStats = $state<DashboardStats | null>(null);
  let projects = $state<Project[]>([]);
  let workspaceId = $state<string>("");

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
    updateTitle = project.name;
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

  function getWorkspaceId(): string | null {
    // Dynamically retrieve current workspace ID from localStorage or URL parameter
    const storedWsId = localStorage.getItem("current_workspace_id");
    if (storedWsId) return storedWsId;

    const urlParams = new URLSearchParams(window.location.search);
    return urlParams.get("workspace_id");
  }

  // API Call: Fetch Workspace Dashboard Stats
  async function loadDashboard() {
    const auth = getAuthHeader();
    const wsId = getWorkspaceId();
    if (!auth || !wsId) return;

    workspaceId = wsId;

    try {
      // 1. Load Dashboard Aggregated Stats
      const dashRes = await fetch(`${API}/workspaces/${workspaceId}/dashboard`, {
        method: "GET",
        headers: auth
      });
      if (!dashRes.ok) throw new Error("Dashboard fetch failed");
      dashboardStats = await dashRes.json();

      // 2. Load Active Workspace Projects
      const projectsRes = await fetch(`${API}/workspaces/${workspaceId}/projects`, {
        method: "GET",
        headers: auth
      });
      if (!projectsRes.ok) throw new Error("Projects fetch failed");
      projects = await projectsRes.json();
    } catch (err) {
      console.error("Failed to load dashboard data:", err);
    }
  }

  // API Call: Delete Project
  async function deleteProject() {
    if (!selectedProject || !workspaceId) return;
    const auth = getAuthHeader();
    if (!auth) return;

    try {
      const res = await fetch(`${API}/workspaces/${workspaceId}/projects/${selectedProject.id}`, {
        method: "DELETE",
        headers: auth
      });
      if (!res.ok) throw new Error("Delete failed");

      projects = projects.filter(p => p.id !== selectedProject?.id);
      showDeleteModal = false;
      selectedProject = null;
      loadDashboard(); // Refresh aggregated metrics
    } catch (err) {
      console.error(err);
    }
  }

  // API Call: Update Project
  async function updateProject() {
    if (!selectedProject || !workspaceId) return;
    const auth = getAuthHeader();
    if (!auth) return;

    try {
      const response = await fetch(`${API}/workspaces/${workspaceId}/projects/${selectedProject.id}`, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          ...auth
        },
        body: JSON.stringify({ name: updateTitle })
      });
      if (!response.ok) throw new Error("Update failed");

      projects = projects.map(p =>
        p.id === selectedProject?.id ? { ...p, name: updateTitle } : p
      );
      showUpdateModal = false;
      selectedProject = null;
      loadDashboard();
    } catch (err) {
      console.error(err);
    }
  }

  // API Call: Create Project
  async function create(event: CustomEvent<{ name: string; client_id?: string }>) {
    const auth = getAuthHeader();
    if (!auth || !workspaceId) return;

    try {
      const response = await fetch(`${API}/workspaces/${workspaceId}/projects`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          ...auth
        },
        body: JSON.stringify(event.detail)
      });
      if (!response.ok) throw new Error("Creation failed");

      await loadDashboard(); // Re-fetch to synchronize state
      show = false;
    } catch (err) {
      console.error(err);
    }
  }

  onMount(() => {
    loadDashboard();
  });

  const open = () => (show = true);
  const close = () => (show = false);
</script>

<svelte:head>
  <title>Dashboard — Stratum</title>
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
  .scanlines {
    background: repeating-linear-gradient(
      to bottom,
      rgba(255, 255, 255, 0.018) 0px,
      rgba(255, 255, 255, 0.018) 1px,
      transparent 1px,
      transparent 3px
    );
  }
</style>

<div class="font-display relative min-h-screen overflow-hidden bg-[#08090b] text-[#eef2f6] antialiased selection:bg-[#3fa9f5] selection:text-white">

  <!-- Scanlines background effect -->
  <div class="scanlines pointer-events-none fixed inset-0 z-40 opacity-50 mix-blend-overlay"></div>

  <!-- Ambient gradient field -->
  <div class="pointer-events-none fixed inset-0">
    <div class="absolute left-1/2 top-[-14%] h-[760px] w-[1200px] -translate-x-1/2 rounded-full bg-[#3fa9f5] opacity-[0.14] blur-[190px]"></div>
  </div>

  <!-- Top bar -->
  <nav class="sticky top-0 z-30 border-b border-white/[0.09] bg-[#08090b]/55 backdrop-blur-xl">
    <div class="mx-auto flex max-w-6xl items-center justify-between px-6 py-[18px]">
      <div class="flex items-center gap-2.5">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#5db9f7" stroke-width="1.8"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/></svg>
        <span class="text-[15px] font-semibold tracking-tight">Stratum Dashboard</span>
      </div>

      <button
        class="rounded-full bg-gradient-to-b from-[#4fb3f7] to-[#1c6ba3] px-5 py-2.5 text-sm font-medium text-white shadow-[0_4px_18px_-4px_rgba(63,169,245,0.6)] transition hover:shadow-[0_6px_26px_-4px_rgba(63,169,245,1)]"
        onclick={open}>
        + New Project
      </button>
    </div>
  </nav>

  <div class="relative z-10 mx-auto max-w-6xl px-6 py-10">
    <p class="font-mono text-[11px] uppercase tracking-[0.25em] text-[#5db9f7]">
      Workspace Overview
    </p>
    <h1 class="mt-2 text-3xl font-semibold tracking-tight sm:text-4xl">Dashboard</h1>

    <!-- 1. DASHBOARD METRICS AGGREGATION CARDS -->
    {#if dashboardStats}
      <div class="mt-8 grid grid-cols-2 gap-4 md:grid-cols-4">
        <div class="rounded-2xl border border-white/[0.09] bg-white/[0.02] p-5">
          <p class="font-mono text-xs uppercase text-white/50">Total Clients</p>
          <p class="mt-2 text-2xl font-bold text-white">{dashboardStats.total_clients}</p>
        </div>
        <div class="rounded-2xl border border-white/[0.09] bg-white/[0.02] p-5">
          <p class="font-mono text-xs uppercase text-white/50">Total Projects</p>
          <p class="mt-2 text-2xl font-bold text-white">{dashboardStats.total_projects}</p>
        </div>
        <div class="rounded-2xl border border-white/[0.09] bg-white/[0.02] p-5">
          <p class="font-mono text-xs uppercase text-white/50">Pending Tasks</p>
          <p class="mt-2 text-2xl font-bold text-[#f7ad3d]">{dashboardStats.pending_tasks}</p>
        </div>
        <div class="rounded-2xl border border-white/[0.09] bg-white/[0.02] p-5">
          <p class="font-mono text-xs uppercase text-white/50">Completed Tasks</p>
          <p class="mt-2 text-2xl font-bold text-[#42d6a4]">{dashboardStats.completed_tasks}</p>
        </div>
      </div>
    {/if}

    <!-- 2. CLIENTS & SUMMARY BREAKDOWN TABLE -->
    {#if dashboardStats && dashboardStats.clients.length > 0}
      <div class="mt-10">
        <h2 class="text-xl font-semibold mb-4 text-white">Client Summaries</h2>
        <div class="overflow-x-auto rounded-2xl border border-white/[0.09] bg-white/[0.02]">
          <table class="w-full text-left text-sm text-white/80">
            <thead class="border-b border-white/[0.09] font-mono text-xs uppercase text-white/40">
              <tr>
                <th class="px-6 py-4">Client Name</th>
                <th class="px-6 py-4">Company</th>
                <th class="px-6 py-4">Projects</th>
                <th class="px-6 py-4">Tasks</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-white/[0.05]">
              {#each dashboardStats.clients as client}
                <tr class="hover:bg-white/[0.02]">
                  <td class="px-6 py-4 font-medium text-white">{client.name}</td>
                  <td class="px-6 py-4 text-white/50">{client.company ?? "—"}</td>
                  <td class="px-6 py-4">{client.total_projects}</td>
                  <td class="px-6 py-4">{client.total_tasks}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    {/if}

    <!-- 3. ACTIVE PROJECTS GRID -->
    <div class="mt-12">
      <h2 class="text-xl font-semibold text-white mb-6">Active Projects</h2>
      {#if projects.length === 0}
        <div class="flex flex-col items-center justify-center rounded-2xl border border-white/[0.09] bg-white/[0.02] py-16 text-center">
          <p class="text-sm text-white/40">No projects found. Create one to get started!</p>
        </div>
      {:else}
        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3">
          {#each projects as p}
            <div class="group relative overflow-hidden rounded-2xl border border-white/[0.09] bg-white/[0.02] p-6 transition hover:border-[#3fa9f5]/40 flex flex-col justify-between">
              <div class="font-medium text-white truncate mb-6">{p.name}</div>
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

<!-- DELETE MODAL -->
{#if showDeleteModal}
  <Pop bind:open={showDeleteModal}>
    <div class="font-display rounded-2xl border border-white/[0.10] bg-[#0e1116] p-6">
      <h2 class="text-xl font-semibold text-white mb-2">Delete project?</h2>
      <p class="text-sm text-white/50 mb-6">This action cannot be undone.</p>
      <div class="flex justify-end gap-2">
        <button
          class="rounded-full border border-white/10 px-4 py-2 text-sm font-medium text-white/70"
          onclick={() => showDeleteModal = false}>
          Cancel
        </button>
        <button
          class="rounded-full bg-gradient-to-b from-[#ff5c7c] to-[#c21f45] px-4 py-2 text-sm font-medium text-white"
          onclick={deleteProject}>
          Confirm delete
        </button>
      </div>
    </div>
  </Pop>
{/if}

<!-- UPDATE MODAL -->
{#if showUpdateModal}
  <Pop bind:open={showUpdateModal}>
    <div class="font-display rounded-2xl border border-white/[0.10] bg-[#0e1116] p-6">
      <h2 class="text-xl font-semibold text-white mb-5">Update project</h2>
      <div class="mb-6">
        <label class="block text-[11px] font-mono uppercase tracking-[0.1em] text-white/50 mb-2" for="update-title">
          Project Name
        </label>
        <input
          id="update-title"
          class="w-full px-4 py-3 rounded-xl border border-white/10 bg-white/[0.03] text-white"
          bind:value={updateTitle}
          type="text"
          placeholder="Enter new project name"
        />
      </div>
      <div class="flex justify-end gap-2">
        <button
          class="rounded-full border border-white/10 px-4 py-2 text-sm font-medium text-white/70"
          onclick={() => showUpdateModal = false}>
          Cancel
        </button>
        <button
          class="rounded-full bg-gradient-to-b from-[#4fb3f7] to-[#1c6ba3] px-4 py-2 text-sm font-medium text-white"
          onclick={updateProject}>
          Save changes
        </button>
      </div>
    </div>
  </Pop>
{/if}