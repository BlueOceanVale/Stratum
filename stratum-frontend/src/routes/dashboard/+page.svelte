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
  let selectedProject: Project | null = null;
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

  async function deleteProject() {
    if (!selectedProject) return;
    const token = localStorage.getItem("token");
    if (!token) return;

    const res = await fetch(
        `${API}/projects/${selectedProject.id}`,
        {
            method: "DELETE",
            headers: {
                Authorization: `Bearer ${token}`
            }
        }
    );

    if (!res.ok) {
        console.error("Delete failed");
        return;
    }

    const selectedProjectId = selectedProject.id;
    projects = projects.filter(
        p => p.id !== selectedProjectId
    );

    showDeleteModal = false;
    selectedProject = null;
  }

  async function updateProject() {
    if (!selectedProject) return;
    const token = localStorage.getItem("token");
    if (!token) return;

    const projectId = selectedProject.id;
    const response = await fetch(
      `${API}/projects/${projectId}`,
      {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`
        },
        body: JSON.stringify({
          title: updateTitle
        })
      }
    );

    if (!response.ok) {
      console.error("Failed to update project");
      return;
    }

    projects = projects.map(p =>
      p.id === projectId ? { ...p, title: updateTitle } : p
    );

    showUpdateModal = false;
    selectedProject = null;
  }

  async function getProjects() {
    const token = localStorage.getItem("token");
    if (!token) return;

    const response = await fetch(`${API}/projects`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        "Authorization": `Bearer ${token}`
      }
    });

    if (!response.ok) {
      console.error("Failed to fetch projects");
      return;
    }

    projects = await response.json();
  }

  onMount(() => {
    getProjects();
  });

  const open = () => (show = true);
  const close = () => (show = false);

  async function create(event: CustomEvent<{ title: string }>) {
  const token = localStorage.getItem("token");
  if (!token) return;

  const response = await fetch(`${API}/project`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "Authorization": `Bearer ${token}`
    },
    body: JSON.stringify(event.detail)
  });

  if (!response.ok) {
    console.error("Failed to create project");
    return;
  }

  const p = await response.json();

  projects = [p, ...projects];
  show = false;
}
</script>

<div class="flex items-center flex-row p-4 mb-2">
  <h1 class="text-3xl font-bold">Stratum</h1>

  <button class="ml-auto px-4 py-2 border border-gray-400 rounded hover:bg-gray-100" onclick={open}>
    New Project
  </button>
</div>

<hr class="border-gray-400">

<div class="border-1 border-gray-400 rounded m-5 h-120 p-4 mt-7">
  <h1 class="text-xl font-medium pb-3">Projects</h1>
  <hr class="border-gray-400">
  <div class="grid grid-cols-3 gap-4">
    {#each projects as p}
      <div class="border-gray-400 p-4 m-1">
        <div class="font-semibold">{p.title}</div>
        <div class="mt-3 flex gap-2">
          <button class="rounded border px-3 py-1 text-sm hover:bg-gray-100" onclick={() => openUpdateModal(p)}>
            Update
          </button>
          <button class="rounded border px-3 py-1 text-sm hover:bg-gray-100" onclick={() => openDeleteModal(p)}>
            Delete
          </button>
        </div>
      </div>
    {/each}
  </div>
  {#if show}
    <Modal on:create={create} on:close={close}/>
  {/if}
</div>

{#if showDeleteModal}
  <Pop bind:open={showDeleteModal}>
    <h2 class="text-xl font-semibold mb-4">Delete Project?</h2>
    <p class="mb-4">This action cannot be undone.</p>
    <div class="flex justify-end gap-2">
      <button class="rounded border px-4 py-2 text-sm" onclick={() => showDeleteModal = false}>
        Cancel
      </button>
      <button class="rounded bg-red-600 px-4 py-2 text-sm text-white hover:bg-red-700" onclick={deleteProject}>
        Confirm Delete
      </button>
    </div>
  </Pop>
{/if}

{#if showUpdateModal}
  <Pop bind:open={showUpdateModal}>
    <h2 class="text-xl font-semibold mb-4">Update Project</h2>
    <label class="mb-2 block text-sm font-medium text-slate-700" for="update-title">
      Project title
    </label>
    <input
      id="update-title"
      class="w-full rounded border border-slate-300 px-3 py-2 text-sm focus:border-sky-500 focus:outline-none mb-4"
      bind:value={updateTitle}
      type="text"
      placeholder="Enter new title"
    />
    <div class="flex justify-end gap-2">
      <button class="rounded border px-4 py-2 text-sm" onclick={() => showUpdateModal = false}>
        Cancel
      </button>
      <button class="rounded bg-sky-600 px-4 py-2 text-sm text-white hover:bg-sky-700" onclick={updateProject}>
        Save
      </button>
    </div>
  </Pop>
{/if}


