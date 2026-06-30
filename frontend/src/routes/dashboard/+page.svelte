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

<div class="max-w-6xl mx-auto p-6">
  <div class="flex items-center justify-between mb-6">
    <h1 class="text-3xl font-extrabold text-slate-950 tracking-tight">Stratum</h1>
    <button class="px-4 py-2 bg-slate-900 text-white font-medium text-sm rounded-xl hover:bg-slate-800 transition-colors shadow-sm" onclick={open}>
      New Project
    </button>
  </div>

  <hr class="border-slate-200 mb-8" />

  <div class="bg-white border border-slate-200 rounded-2xl p-6 min-h-[400px] shadow-sm">
    <h2 class="text-xl font-bold text-slate-800 mb-4">Projects</h2>
    
    {#if projects.length === 0}
      <div class="flex flex-col items-center justify-center py-12 text-slate-400">
        <p class="text-sm">No projects found. Create one to get started!</p>
      </div>
    {:else}
      <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
        {#each projects as p}
          <div class="border border-slate-200 rounded-xl p-4 hover:border-slate-300 hover:shadow-sm transition-all bg-slate-50/50 flex flex-col justify-between">
            <div class="font-semibold text-slate-800 truncate mb-4">{p.title}</div>
            <div class="flex gap-2 justify-end">
              <button class="rounded-lg border border-slate-200 bg-white px-3 py-1.5 text-xs font-medium text-slate-600 hover:bg-slate-50 transition-colors" onclick={() => openUpdateModal(p)}>
                Update
              </button>
              <button class="rounded-lg border border-red-200 bg-white px-3 py-1.5 text-xs font-medium text-red-600 hover:bg-red-50 transition-colors" onclick={() => openDeleteModal(p)}>
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

{#if showDeleteModal}
  <Pop bind:open={showDeleteModal}>
    <h2 class="text-xl font-bold text-slate-900 mb-2">Delete Project?</h2>
    <p class="text-sm text-slate-500 mb-5">This action cannot be undone. This will permanently remove the project parameters.</p>
    <div class="flex justify-end gap-2">
      <button class="rounded-xl border border-slate-200 px-4 py-2 text-sm font-medium hover:bg-slate-50 transition-colors" onclick={() => showDeleteModal = false}>
        Cancel
      </button>
      <button class="rounded-xl bg-red-600 px-4 py-2 text-sm font-medium text-white hover:bg-red-700 transition-colors shadow-sm" onclick={deleteProject}>
        Confirm Delete
      </button>
    </div>
  </Pop>
{/if}

{#if showUpdateModal}
  <Pop bind:open={showUpdateModal}>
    <h2 class="text-xl font-bold text-slate-900 mb-4">Update Project</h2>
    <div class="mb-4">
      <label class="mb-1.5 block text-xs font-semibold text-slate-700" for="update-title">
        Project Title
      </label>
      <input
        id="update-title"
        class="w-full rounded-xl border border-slate-200 px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-all"
        bind:value={updateTitle}
        type="text"
        placeholder="Enter new title"
      />
    </div>
    <div class="flex justify-end gap-2">
      <button class="rounded-xl border border-slate-200 px-4 py-2 text-sm font-medium hover:bg-slate-50 transition-colors" onclick={() => showUpdateModal = false}>
        Cancel
      </button>
      <button class="rounded-xl bg-sky-600 px-4 py-2 text-sm font-medium text-white hover:bg-sky-700 transition-colors shadow-sm" onclick={updateProject}>
        Save Changes
      </button>
    </div>
  </Pop>
{/if}