<script lang="ts">
  let { oncreate, onclose } = $props<{
    oncreate: (event: CustomEvent<{ title: string }>) => void;
    onclose: () => void;
  }>();

  let title = $state("");

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (!title.trim()) return;
    
    oncreate(new CustomEvent('create', { detail: { title } }));
  }
</script>

<div class="fixed inset-0 bg-slate-900/40 backdrop-blur-sm flex items-center justify-center p-4 z-50">
  <div class="bg-white rounded-2xl p-6 max-w-md w-full shadow-xl border border-slate-100">
    <h2 class="text-xl font-bold text-slate-900 mb-4">Create New Project</h2>
    
    <form onsubmit={handleSubmit}>
      <div class="mb-4">
        <label for="new-project-title" class="block text-xs font-semibold text-slate-700 mb-1.5">Project Title</label>
        <input 
          id="new-project-title"
          type="text" 
          placeholder="My awesome project..." 
          bind:value={title}
          required
          class="w-full px-3 py-2.5 border border-slate-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-slate-900 focus:border-transparent transition-all text-sm"
        />
      </div>

      <div class="flex justify-end gap-2">
        <button type="button" onclick={onclose} class="rounded-xl border border-slate-200 px-4 py-2 text-sm font-medium hover:bg-slate-50 transition-colors">
          Cancel
        </button>
        <button type="submit" class="rounded-xl bg-slate-900 px-4 py-2 text-sm font-medium text-white hover:bg-slate-800 transition-colors shadow-sm">
          Create Project
        </button>
      </div>
    </form>
  </div>
</div>