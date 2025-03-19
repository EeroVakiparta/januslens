<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import RepoExplorer from './RepoExplorer.svelte';
  import BranchManager from './BranchManager.svelte';
  import CommitPanel from './CommitPanel.svelte';
  import GitGraph from './GitGraph.svelte';
  
  export let repoPath = '';
  
  let branches = [];
  let commits = [];
  let currentBranch = null;
  let isLoading = false;
  let errorMessage = '';
  
  $: if (repoPath) {
    loadRepository();
  }
  
  async function loadRepository() {
    try {
      isLoading = true;
      errorMessage = '';
      
      // Load branches
      branches = await invoke('get_branches', { repoPath });
      
      // Find current branch (HEAD)
      currentBranch = branches.find(b => b.is_head) || null;
      
      // Load commits for current branch
      if (currentBranch) {
        commits = await invoke('get_commits', { 
          repoPath, 
          branchName: currentBranch.name, 
          limit: 100 
        });
      } else {
        commits = [];
      }
    } catch (error) {
      console.error('Failed to load repository data:', error);
      errorMessage = `Failed to load repository: ${error.message || error}`;
    } finally {
      isLoading = false;
    }
  }
  
  async function handleBranchChange(branch) {
    if (!branch || branch.is_head) return;
    
    try {
      isLoading = true;
      errorMessage = '';
      
      // In a real implementation, we would use git2 to checkout branch
      // For now, let's just update our UI state as if the checkout succeeded
      
      // Simulate branch checkout
      await invoke('checkout_branch', { repoPath, branchName: branch.name });
      
      // Reload branches to get updated HEAD
      await loadRepository();
    } catch (error) {
      console.error('Failed to switch branch:', error);
      errorMessage = `Failed to switch branch: ${error.message || error}`;
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="main-layout">
  {#if isLoading}
    <div class="loading-overlay">
      <div class="spinner"></div>
      <div class="loading-text">Loading repository...</div>
    </div>
  {/if}
  
  {#if errorMessage}
    <div class="error-banner">
      <span>{errorMessage}</span>
      <button class="close-btn" on:click={() => errorMessage = ''}>×</button>
    </div>
  {/if}
  
  <div class="left-panel">
    <div class="branch-section">
      <BranchManager 
        {branches} 
        {repoPath} 
        {currentBranch}
        on:branchSelected={e => handleBranchChange(e.detail)}
      />
    </div>
    <div class="files-section">
      <RepoExplorer {repoPath} />
    </div>
  </div>
  
  <div class="center-panel">
    <div class="graph-section">
      <GitGraph {commits} {branches} />
    </div>
  </div>
  
  <div class="right-panel">
    <CommitPanel {repoPath} />
  </div>
</div>

<style>
  .main-layout {
    display: grid;
    grid-template-columns: 250px 1fr 300px;
    height: 100%;
    width: 100%;
    position: relative;
  }
  
  .left-panel {
    border-right: 1px solid #ddd;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  
  .branch-section {
    flex: 0 0 auto;
    max-height: 40%;
    overflow-y: auto;
    border-bottom: 1px solid #ddd;
  }
  
  .files-section {
    flex: 1;
    overflow-y: auto;
  }
  
  .center-panel {
    overflow: hidden;
  }
  
  .graph-section {
    height: 100%;
    width: 100%;
    overflow: auto;
  }
  
  .right-panel {
    border-left: 1px solid #ddd;
    overflow-y: auto;
  }
  
  .loading-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(255, 255, 255, 0.8);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }
  
  .spinner {
    width: 40px;
    height: 40px;
    border: 4px solid rgba(0, 0, 0, 0.1);
    border-radius: 50%;
    border-top-color: #0066cc;
    animation: spin 1s ease-in-out infinite;
    margin-bottom: 0.5rem;
  }
  
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  
  .loading-text {
    color: #333;
    font-size: 1rem;
  }
  
  .error-banner {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    background-color: #f8d7da;
    color: #721c24;
    padding: 0.5rem 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    z-index: 101;
  }
  
  .close-btn {
    background: none;
    border: none;
    font-size: 1.2rem;
    cursor: pointer;
    color: #721c24;
  }
</style> 