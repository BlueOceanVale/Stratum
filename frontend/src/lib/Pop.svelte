<script lang="ts">
  let { open = $bindable(), children } = $props();
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4">
    <div class="w-full max-w-md rounded bg-white p-6 shadow-lg">
      {@render children()}
    </div>
  </div>
{/if}