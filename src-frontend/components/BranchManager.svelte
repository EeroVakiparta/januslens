<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  
  export let branches = [];
  export let repoPath = '';
  export let currentBranch = null;
  
  let isCreatingBranch = false;
  let newBranchName = '';
  let errorMessage = '';
  
  function selectBranch(branch) {
    if (branch) {
      currentBranch = branch;
      // Dispatch event in a real implementation
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
      // In a real implementation, this would be a Tauri invoke call
      // const result = await invoke('create_branch', { 
      //   repoPath, 
      //   branchName: newBranchName 
      // });
      
      // Mock the result for now
      const mockBranch = {
        name: newBranchName,
        is_head: false,
        upstream: null,
        commit_id: currentBranch ? currentBranch.commit_id : ''
      };
      
      // Add to branches (would be updated via a refresh in real implementation)
      branches = [...branches, mockBranch];
      
      // Reset form
      newBranchName = '';
      isCreatingBranch = false;
      errorMessage = '';
    } catch (error) {
      errorMessage = `Failed to create branch: ${error.message || error}`;
    }
  }
  
  async function deleteBranch(branch) {
    if (branch.is_head) {
      errorMessage = 'Cannot delete the current branch';
      return;
    }
    
    if (confirm(`Are you sure you want to delete the branch '${branch.name}'?`)) {
      try {
        // In a real implementation, this would be a Tauri invoke call
        // await invoke('delete_branch', { 
        //   repoPath, 
        //   branchName: branch.name 
        // });
        
        // Update branches (would be updated via a refresh in real implementation)
        branches = branches.filter(b => b.name !== branch.name);
        errorMessage = '';
      } catch (error) {
        errorMessage = `Failed to delete branch: ${error.message || error}`;
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
    <button class="create-btn" on:click={() => isCreatingBranch = true}>New</button>
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
        <button class="cancel-btn" on:click={cancelCreateBranch}>Cancel</button>
        <button class="create-btn" on:click={createBranch}>Create</button>
      </div>
    </div>
  {/if}
  
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
</div>

<style>
  .branch-manager {
    padding: 0.5rem;
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
  
  .branch-list {
    list-style: none;
    padding: 0;
    margin: 0;
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
  
  .create-btn {
    background-color: #0066cc;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 0.25rem 0.5rem;
    cursor: pointer;
  }
  
  .cancel-btn {
    background-color: #f0f0f0;
    border: 1px solid #ddd;
    border-radius: 4px;
    padding: 0.25rem 0.5rem;
    cursor: pointer;
  }
  
  .error-message {
    color: #f05050;
    font-size: 0.85rem;
    margin-bottom: 0.5rem;
  }
</style> 