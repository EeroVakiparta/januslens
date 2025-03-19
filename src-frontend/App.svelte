<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  
  let repositories = [];
  let currentRepo = null;
  let branches = [];
  let commits = [];
  let isLoading = false;
  
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
  
  async function openRepository(path) {
    try {
      isLoading = true;
      currentRepo = await invoke('open_repository', { path });
      
      // Once a repo is opened, load branches
      branches = await invoke('get_branches', { repo_path: path });
      
      // Load commits from HEAD
      commits = await invoke('get_commits', { 
        repo_path: path,
        branch_name: null,
        limit: 100
      });
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
</script>

<main>
  <div class="app-container">
    <header class="app-header">
      <h1>JanusLens</h1>
      <div class="repo-selector">
        {#if repositories.length > 0}
          <select on:change={(e) => openRepository(e.target.value)}>
            <option value="">Select Repository</option>
            {#each repositories as repo}
              <option value={repo.path}>{repo.name}</option>
            {/each}
          </select>
        {:else}
          <button class="open-repo-btn">Open Repository</button>
        {/if}
      </div>
    </header>
    
    <div class="app-content">
      {#if isLoading}
        <div class="loading">Loading...</div>
      {:else if currentRepo}
        <div class="sidebar">
          <h3>Branches</h3>
          <ul class="branch-list">
            {#each branches as branch}
              <li 
                class:current={branch.is_head}
                on:click={() => selectBranch(branch.name)}
              >
                {branch.name}
              </li>
            {/each}
          </ul>
        </div>
        
        <div class="main-content">
          <div class="commit-graph">
            <h3>Commit History</h3>
            <div class="commits-container">
              {#each commits as commit}
                <div class="commit-item">
                  <div class="commit-id">{commit.short_id}</div>
                  <div class="commit-details">
                    <div class="commit-summary">{commit.summary}</div>
                    <div class="commit-meta">
                      {commit.author} - {new Date(commit.time * 1000).toLocaleString()}
                    </div>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        </div>
      {:else}
        <div class="empty-state">
          <h2>No Repository Selected</h2>
          <p>Open a Git repository to get started</p>
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
    width: 220px;
    border-right: 1px solid #ddd;
    overflow-y: auto;
    padding: 1rem;
  }
  
  .main-content {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
  }
  
  .branch-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  
  .branch-list li {
    padding: 0.5rem;
    border-radius: 4px;
    cursor: pointer;
  }
  
  .branch-list li:hover {
    background-color: #f0f0f0;
  }
  
  .branch-list li.current {
    background-color: #e0f0ff;
    font-weight: bold;
  }
  
  .commit-item {
    display: flex;
    padding: 0.75rem;
    border-bottom: 1px solid #eee;
  }
  
  .commit-id {
    font-family: monospace;
    color: #777;
    margin-right: 1rem;
  }
  
  .commit-summary {
    font-weight: 500;
  }
  
  .commit-meta {
    font-size: 0.85rem;
    color: #666;
    margin-top: 0.25rem;
  }
  
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #666;
  }
  
  .loading {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    width: 100%;
    font-size: 1.2rem;
    color: #666;
  }
  
  .open-repo-btn {
    padding: 0.5rem 1rem;
    background-color: #0066cc;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  
  .open-repo-btn:hover {
    background-color: #0055aa;
  }
</style> 