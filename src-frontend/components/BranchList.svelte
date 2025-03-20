<script>
  import { onMount, createEventDispatcher } from 'svelte';
  import { Logger } from '../services/logger';
  import { invoke } from '@tauri-apps/api/core';
  
  export let repoPath = '';
  
  // Branch data
  let branches = [];
  let isLoading = false;
  let error = null;
  
  const dispatch = createEventDispatcher();
  
  // Fetch branches when repo path changes
  $: if (repoPath) {
    fetchBranches();
  }
  
  // Detect if we're in development mode without Tauri
  const isDevWithoutTauri = () => {
    return typeof window !== 'undefined' && 
           (window.__TAURI__ === undefined || 
            process.env.NODE_ENV === 'development');
  };
  
  async function fetchBranches() {
    if (!repoPath) return;
    
    Logger.info('BranchList', 'Fetching branches', { repoPath });
    isLoading = true;
    error = null;
    
    try {
      if (isDevWithoutTauri()) {
        // Mock data for development
        await new Promise(resolve => setTimeout(resolve, 300));
        
        branches = [
          { 
            name: "main", 
            is_head: true, 
            upstream: "origin/main",
            commit_id: "abc123" 
          },
          { 
            name: "develop", 
            is_head: false, 
            upstream: "origin/develop",
            commit_id: "def456" 
          },
          { 
            name: "feature/new-ui", 
            is_head: false, 
            upstream: null,
            commit_id: "ghi789" 
          }
        ];
        
        Logger.info('BranchList', 'Loaded mock branches', { count: branches.length });
        return;
      }
      
      // Actual backend call
      branches = await invoke('get_branches', { repoPath });
      Logger.info('BranchList', 'Branches loaded successfully', { count: branches.length });
    } catch (err) {
      const errorMessage = err.message || String(err);
      error = `Failed to load branches: ${errorMessage}`;
      branches = [];
      Logger.error('BranchList', 'Failed to load branches', { error: errorMessage });
    } finally {
      isLoading = false;
    }
  }
  
  function selectBranch(branch) {
    Logger.info('BranchList', 'Branch selected', { name: branch.name });
    dispatch('branchSelected', branch);
  }
  
  onMount(() => {
    Logger.info('BranchList', 'Component mounted');
  });
</script>

<div class="branch-list" data-test-id="jl-branch-list">
  <div class="branch-list-header">
    <h3>Branches</h3>
    <button 
      class="refresh-button" 
      on:click={fetchBranches} 
      disabled={isLoading}
      data-test-id="jl-refresh-branches-btn"
    >
      ↻
    </button>
  </div>
  
  {#if isLoading}
    <div class="loading" data-test-id="jl-branch-list-loading">Loading branches...</div>
  {:else if error}
    <div class="error" data-test-id="jl-branch-list-error">{error}</div>
  {:else if branches.length === 0}
    <div class="empty-state" data-test-id="jl-branch-list-empty">
      No branches found. 
      {#if !repoPath}
        <div class="hint">Select a repository first.</div>
      {/if}
    </div>
  {:else}
    <!-- Local Branches -->
    <div class="branch-section" data-test-id="jl-local-branches-section">
      <div class="section-header">LOCAL</div>
      <ul class="branch-items">
        {#each branches as branch}
          <li 
            class="branch-item {branch.is_head ? 'active' : ''}" 
            on:click={() => selectBranch(branch)}
            data-test-id="jl-branch-item"
            data-branch-name={branch.name}
            data-is-head={branch.is_head}
          >
            <span class="branch-indicator">
              {#if branch.is_head}●{:else}○{/if}
            </span>
            <span class="branch-name">{branch.name}</span>
            {#if branch.upstream}
              <span class="branch-tracking">→ {branch.upstream}</span>
            {/if}
          </li>
        {/each}
      </ul>
    </div>
  {/if}
</div>

<style>
  .branch-list {
    height: 100%;
    overflow-y: auto;
    font-size: 14px;
  }
  
  .branch-list-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 8px 8px 8px;
    border-bottom: 1px solid var(--border-color, #ddd);
  }
  
  h3 {
    margin: 0;
    font-size: 16px;
    font-weight: 500;
  }
  
  .refresh-button {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 16px;
    color: var(--text-muted, #666);
    transition: color 0.2s;
  }
  
  .refresh-button:hover {
    color: var(--primary-color, #0066cc);
  }
  
  .refresh-button:disabled {
    color: var(--text-disabled, #ccc);
    cursor: not-allowed;
  }
  
  .branch-section {
    margin-top: 12px;
  }
  
  .section-header {
    padding: 4px 8px;
    font-size: 11px;
    font-weight: bold;
    color: var(--text-muted, #666);
  }
  
  .branch-items {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  
  .branch-item {
    display: flex;
    align-items: center;
    padding: 4px 8px;
    cursor: pointer;
    border-radius: 4px;
    margin: 2px 0;
    transition: background-color 0.2s;
  }
  
  .branch-item:hover {
    background-color: var(--item-hover-bg, #f0f0f0);
  }
  
  .branch-item.active {
    background-color: var(--item-active-bg, #e6f7ff);
  }
  
  .branch-indicator {
    margin-right: 8px;
    color: var(--primary-color, #0066cc);
    font-size: 12px;
    width: 12px;
    text-align: center;
  }
  
  .branch-indicator.remote {
    color: var(--text-muted, #666);
  }
  
  .branch-name {
    flex-grow: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  
  .branch-tracking {
    margin-left: 8px;
    font-size: 11px;
    color: var(--text-muted, #666);
  }
  
  .loading, .error, .empty-state {
    padding: 16px 8px;
    text-align: center;
    color: var(--text-muted, #666);
  }
  
  .error {
    color: var(--error-color, #d32f2f);
  }
  
  .hint {
    font-size: 12px;
    margin-top: 8px;
    color: var(--text-light, #999);
  }
</style> 