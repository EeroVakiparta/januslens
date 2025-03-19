<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { dialog } from '@tauri-apps/api';
  
  import GitGraph from './components/GitGraph.svelte';
  import BranchManager from './components/BranchManager.svelte';
  import RepoExplorer from './components/RepoExplorer.svelte';
  import CommitPanel from './components/CommitPanel.svelte';
  import DiffViewer from './components/DiffViewer.svelte';
  
  // App state
  let repositories = [];
  let currentRepo = null;
  let branches = [];
  let commits = [];
  let isLoading = false;
  let activeTab = 'history'; // 'history' or 'commit'
  let currentBranch = null;
  
  onMount(async () => {
    try {
      isLoading = true;
      repositories = await invoke('list_repositories');
    } catch (error) {
      console.error('Failed to load repositories:', error);
    } finally {
      isLoading = false;
    }
  });
  
  async function openRepository() {
    try {
      // Open a file dialog to select a directory
      const selected = await dialog.open({
        directory: true,
        multiple: false,
        title: 'Select Git Repository'
      });
      
      if (selected) {
        isLoading = true;
        const path = Array.isArray(selected) ? selected[0] : selected;
        
        currentRepo = await invoke('open_repository', { path });
        
        // Once a repo is opened, load branches
        branches = await invoke('get_branches', { repo_path: path });
        
        // Find current branch (HEAD)
        currentBranch = branches.find(b => b.is_head) || null;
        
        // Load commits from HEAD
        commits = await invoke('get_commits', { 
          repo_path: path,
          branch_name: null,
          limit: 100
        });
      }
    } catch (error) {
      console.error('Failed to open repository:', error);
    } finally {
      isLoading = false;
    }
  }
  
  async function selectBranch(branchName) {
    if (!currentRepo) return;
    
    try {
      isLoading = true;
      
      // Find the branch object
      const branch = branches.find(b => b.name === branchName);
      if (branch) {
        currentBranch = branch;
      }
      
      // Load commits for selected branch
      commits = await invoke('get_commits', {
        repo_path: currentRepo.path,
        branch_name: branchName,
        limit: 100
      });
    } catch (error) {
      console.error('Failed to load commits:', error);
    } finally {
      isLoading = false;
    }
  }
  
  function handleBranchSelect(branch) {
    selectBranch(branch.name);
  }
</script>

<main>
  <div class="app-container">
    <header class="app-header">
      <h1>JanusLens</h1>
      <div class="repo-selector">
        {#if currentRepo}
          <span class="current-repo">{currentRepo.name}</span>
          <button class="repo-btn" on:click={() => openRepository()}>Change</button>
        {:else}
          <button class="open-repo-btn" on:click={() => openRepository()}>Open Repository</button>
        {/if}
      </div>
    </header>
    
    <div class="app-content">
      {#if isLoading}
        <div class="loading">
          <div class="spinner"></div>
          <div class="loading-text">Loading...</div>
        </div>
      {:else if currentRepo}
        <div class="sidebar">
          <BranchManager 
            branches={branches} 
            repoPath={currentRepo.path}
            currentBranch={currentBranch}
          />
          
          <div class="sidebar-section">
            <RepoExplorer repoPath={currentRepo.path} />
          </div>
        </div>
        
        <div class="main-content">
          <div class="tab-nav">
            <button 
              class="tab-btn" 
              class:active={activeTab === 'history'}
              on:click={() => activeTab = 'history'}
            >
              History
            </button>
            <button 
              class="tab-btn"
              class:active={activeTab === 'commit'}
              on:click={() => activeTab = 'commit'}
            >
              Changes
            </button>
          </div>
          
          <div class="tab-content">
            {#if activeTab === 'history'}
              <GitGraph commits={commits} branches={branches} />
            {:else}
              <CommitPanel repoPath={currentRepo.path} />
            {/if}
          </div>
        </div>
      {:else}
        <div class="empty-state">
          <h2>Welcome to JanusLens</h2>
          <p>Open a Git repository to get started</p>
          <button class="primary-btn" on:click={() => openRepository()}>Open Repository</button>
        </div>
      {/if}
    </div>
  </div>
</main>

<style>
  .app-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100%;
    color: #333;
  }
  
  .app-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 1rem;
    background-color: #f0f4f8;
    border-bottom: 1px solid #ddd;
  }
  
  .app-header h1 {
    font-size: 1.5rem;
    font-weight: 600;
    margin: 0;
  }
  
  .app-content {
    display: flex;
    flex: 1;
    overflow: hidden;
  }
  
  .sidebar {
    width: 240px;
    border-right: 1px solid #ddd;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }
  
  .sidebar-section {
    flex: 1;
    overflow-y: auto;
    border-top: 1px solid #ddd;
    padding: 1rem;
  }
  
  .main-content {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
  
  .tab-nav {
    display: flex;
    border-bottom: 1px solid #ddd;
  }
  
  .tab-btn {
    padding: 0.75rem 1.5rem;
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    font-weight: 500;
  }
  
  .tab-btn.active {
    border-bottom-color: #0066cc;
    color: #0066cc;
  }
  
  .tab-content {
    flex: 1;
    overflow: auto;
  }
  
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #666;
    text-align: center;
    padding: 2rem;
  }
  
  .open-repo-btn, .primary-btn {
    padding: 0.5rem 1.5rem;
    background-color: #0066cc;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
    margin-top: 1rem;
  }
  
  .open-repo-btn:hover, .primary-btn:hover {
    background-color: #0055aa;
  }
  
  .current-repo {
    font-weight: 500;
    margin-right: 0.5rem;
  }
  
  .repo-btn {
    padding: 0.25rem 0.5rem;
    background-color: #f8f9fa;
    border: 1px solid #ddd;
    border-radius: 4px;
    cursor: pointer;
  }
  
  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    width: 100%;
  }
  
  .spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #f3f3f3;
    border-top: 4px solid #0066cc;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 1rem;
  }
  
  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
</style> 