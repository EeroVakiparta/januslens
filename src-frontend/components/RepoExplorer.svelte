<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { createEventDispatcher } from 'svelte';
  
  export let repoPath = '';
  
  const dispatch = createEventDispatcher();
  
  function refresh() {
    console.log("Refresh files in:", repoPath);
  }
</script>

<div class="repo-explorer">
  <h3>Files</h3>
  <button on:click={refresh}>Refresh</button>
  <div>Path: {repoPath || 'No repository selected'}</div>
</div>

<style>
  .repo-explorer {
    padding: 1rem;
  }
</style>
