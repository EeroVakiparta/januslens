<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { createEventDispatcher } from 'svelte';
  
  export let branches = [];
  export let repoPath = '';
  export let currentBranch = null;
  
  let isCreatingBranch = false;
  let newBranchName = '';
  let errorMessage = '';
  let isLoading = false;
  
  const dispatch = createEventDispatcher();
  
  function selectBranch(branch) {
    if (branch) {
      dispatch('branchSelected', branch);
    }
  }
  
  async function refreshBranches() {
    if (!repoPath) return;
    
    try {
      isLoading = true;
      branches = await invoke('get_branches', { repoPath });
      
      // Find current branch (HEAD)
      currentBranch = branches.find(b => b.is_head) || null;
    } catch (error) {
      console.error('Failed to refresh branches:', error);
      errorMessage = `Failed to refresh branches: ${error.message || error}`;
    } finally {
      isLoading = false;
    }
  }
  
  async function createBranch() {
    if (!newBranchName.trim()) {
      errorMessage = 'Branch name cannot be empty';
      return;
    }
    
    // Validate branch name (basic validation)
    if (!/^[a-zA-Z0-9_\-.]+$/.test(newBranchName)) {
      errorMessage = 'Invalid branch name. Use letters, numbers, dashes, underscores.';
      return;
    }
    
    // Check for duplicate
    if (branches.some(b => b.name === newBranchName)) {
      errorMessage = `Branch '${newBranchName}' already exists`;
      return;
    }
    
    try {
      isLoading = true;
      
      const newBranch = await invoke('create_branch', { 
        repoPath, 
        branchName: newBranchName 
      });
      
      // Refresh branches
      await refreshBranches();
      
      // Reset form
      newBranchName = '';
      isCreatingBranch = false;
      errorMessage = '';
    } catch (error) {
      console.error('Failed to create branch:', error);
      errorMessage = `Failed to create branch: ${error.message || error}`;
    } finally {
      isLoading = false;
    }
  }
  
  async function deleteBranch(branch) {
    if (branch.is_head) {
      errorMessage = 'Cannot delete the current branch';
      return;
    }
    
    if (confirm(`Are you sure you want to delete the branch '${branch.name}'?`)) {
      try {
        isLoading = true;
        
        await invoke('delete_branch', { 
          repoPath, 
          branchName: branch.name 
        });
        
        // Refresh branches
        await refreshBranches();
        
        errorMessage = '';
      } catch (error) {
        console.error('Failed to delete branch:', error);
        errorMessage = `Failed to delete branch: ${error.message || error}`;
      } finally {
        isLoading = false;
      }
    }
  }
  
  function cancelCreateBranch() {
    isCreatingBranch = false;
    newBranchName = '';
    errorMessage = '';
  }
</script>

<div class="branch-manager">
  <div class="header">
    <h3>Branches</h3>
    <div class="header-actions">
      <button class="refresh-btn" on:click={refreshBranches} disabled={isLoading}>↻</button>
      <button class="create-btn" on:click={() => isCreatingBranch = true} disabled={isLoading}>New</button>
    </div>
  </div>
  
  {#if errorMessage}
    <div class="error-message">{errorMessage}</div>
  {/if}
  
  {#if isCreatingBranch}
    <div class="new-branch-form">
      <input 
        type="text" 
        bind:value={newBranchName} 
        placeholder="New branch name"
        autofocus
      />
      <div class="form-actions">
        <button class="cancel-btn" on:click={cancelCreateBranch} disabled={isLoading}>Cancel</button>
        <button class="create-btn" on:click={createBranch} disabled={isLoading}>Create</button>
      </div>
    </div>
  {/if}
  
  {#if isLoading}
    <div class="loading">
      <div class="mini-spinner"></div>
      <span>Loading branches...</span>
    </div>
  {:else if branches.length === 0}
    <div class="empty-state">No branches found</div>
  {:else}
    <ul class="branch-list">
      {#each branches as branch}
        <li class={branch.is_head ? 'current' : ''}>
          <div class="branch-info" on:click={() => selectBranch(branch)}>
            <span class="branch-name">{branch.name}</span>
            {#if branch.is_head}
              <span class="badge">HEAD</span>
            {/if}
          </div>
          <div class="branch-actions">
            {#if !branch.is_head}
              <button 
                class="delete-btn" 
                on:click={(e) => {
                  e.stopPropagation();
                  deleteBranch(branch);
                }}
              >
                &times;
              </button>
            {/if}
          </div>
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .branch-manager {
    padding: 0.5rem;
    height: 100%;
    display: flex;
    flex-direction: column;
  }
  
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }
  
  .header h3 {
    margin: 0;
    font-size: 1rem;
  }
  
  .header-actions {
    display: flex;
    gap: 0.5rem;
  }
  
  .branch-list {
    list-style: none;
    padding: 0;
    margin: 0;
    overflow-y: auto;
    flex: 1;
  }
  
  .branch-list li {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem;
    border-radius: 4px;
    cursor: pointer;
    margin-bottom: 0.25rem;
  }
  
  .branch-list li:hover {
    background-color: #f0f0f0;
  }
  
  .branch-list li.current {
    background-color: #e0f0ff;
    font-weight: bold;
  }
  
  .badge {
    font-size: 0.7rem;
    background-color: #0066cc;
    color: white;
    padding: 0.1rem 0.3rem;
    border-radius: 2px;
    margin-left: 0.5rem;
  }
  
  .branch-actions {
    opacity: 0;
    transition: opacity 0.2s;
  }
  
  .branch-list li:hover .branch-actions {
    opacity: 1;
  }
  
  .delete-btn {
    background: none;
    border: none;
    color: #f05050;
    font-size: 1.2rem;
    cursor: pointer;
    padding: 0;
  }
  
  .new-branch-form {
    margin-bottom: 1rem;
    padding: 0.5rem;
    background-color: #f9f9f9;
    border-radius: 4px;
  }
  
  .new-branch-form input {
    width: 100%;
    margin-bottom: 0.5rem;
  }
  
  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
  }
  
  .create-btn, .refresh-btn {
    background-color: #0066cc;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 0.25rem 0.5rem;
    cursor: pointer;
  }
  
  .create-btn:disabled, .refresh-btn:disabled {
    background-color: #99ccff;
    cursor: not-allowed;
  }
  
  .refresh-btn {
    font-size: 1rem;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    padding: 0;
  }
  
  .cancel-btn {
    background-color: #f0f0f0;
    border: 1px solid #ddd;
    border-radius: 4px;
    padding: 0.25rem 0.5rem;
    cursor: pointer;
  }
  
  .cancel-btn:disabled {
    background-color: #f8f8f8;
    color: #aaa;
    cursor: not-allowed;
  }
  
  .error-message {
    color: #f05050;
    font-size: 0.85rem;
    margin-bottom: 0.5rem;
    padding: 0.5rem;
    background-color: #fff0f0;
    border-radius: 4px;
  }
  
  .loading {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1rem;
    color: #666;
  }
  
  .mini-spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(0, 0, 0, 0.1);
    border-radius: 50%;
    border-top-color: #0066cc;
    animation: spin 1s ease-in-out infinite;
    margin-right: 0.5rem;
  }
  
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  
  .empty-state {
    padding: 1rem;
    text-align: center;
    color: #666;
    font-style: italic;
  }
</style> 