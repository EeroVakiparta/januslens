<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { open } from '@tauri-apps/api/dialog';
  import MainLayout from './components/MainLayout.svelte';
  
  let appName = "JanusLens";
  let isLoading = false;
  let errorMessage = '';
  let repoPath = '';
  let recentRepositories = [];
  
  onMount(async () => {
    // Load recent repositories on startup
    try {
      recentRepositories = await invoke('list_repositories');
    } catch (error) {
      console.error('Failed to load recent repositories:', error);
      // This is non-critical, so we won't show an error message
      recentRepositories = [];
    }
  });
  
  async function openRepository() {
    try {
      isLoading = true;
      errorMessage = '';
      
      // Open a directory selection dialog
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select Git Repository'
      });
      
      // If user cancelled, do nothing
      if (!selected) {
        isLoading = false;
        return;
      }
      
      // Try to open the selected repository
      const repoInfo = await invoke('open_repository', { path: selected });
      
      // If successful, update current repository
      repoPath = repoInfo.path;
      
      // Add to recent repositories (would usually be handled by backend)
      if (!recentRepositories.some(repo => repo.path === repoPath)) {
        recentRepositories = [repoInfo, ...recentRepositories.slice(0, 9)];
      }
    } catch (error) {
      console.error('Failed to open repository:', error);
      errorMessage = `Failed to open repository: ${error.message || error}`;
      repoPath = '';
    } finally {
      isLoading = false;
    }
  }
  
  function openRecentRepository(repo) {
    repoPath = repo.path;
  }
</script>

<main>
  <div class="app-container">
    <header class="app-header">
      <h1>{appName}</h1>
      <div class="header-actions">
        {#if repoPath}
          <div class="current-repo">
            <span>Current: {repoPath.split('/').pop()}</span>
            <button class="header-btn" on:click={openRepository}>Change</button>
          </div>
        {:else}
          <button class="header-btn primary" on:click={openRepository}>Open Repository</button>
        {/if}
        <div class="version-info">v0.1.0</div>
      </div>
    </header>
    
    {#if errorMessage}
      <div class="error-banner">
        <span>{errorMessage}</span>
        <button class="close-btn" on:click={() => errorMessage = ''}>×</button>
      </div>
    {/if}
    
    <div class="app-content">
      {#if isLoading}
        <div class="loading">
          <div class="spinner"></div>
          <div class="loading-text">Loading...</div>
        </div>
      {:else if repoPath}
        <MainLayout {repoPath} />
      {:else}
        <div class="empty-state">
          <h2>Welcome to JanusLens</h2>
          <p>Open a Git repository to get started</p>
          <button class="primary-btn" on:click={openRepository}>Open Repository</button>
          
          {#if recentRepositories.length > 0}
            <div class="recent-repos">
              <h3>Recent Repositories</h3>
              <ul>
                {#each recentRepositories as repo}
                  <li>
                    <button class="repo-item" on:click={() => openRecentRepository(repo)}>
                      <span class="repo-name">{repo.name}</span>
                      <span class="repo-path">{repo.path}</span>
                    </button>
                  </li>
                {/each}
              </ul>
            </div>
          {/if}
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
  
  .header-actions {
    display: flex;
    align-items: center;
    gap: 1rem;
  }
  
  .current-repo {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.9rem;
  }
  
  .header-btn {
    padding: 0.25rem 0.5rem;
    font-size: 0.85rem;
    border-radius: 4px;
    border: 1px solid #ccc;
    background-color: #f8f8f8;
    cursor: pointer;
  }
  
  .header-btn.primary {
    background-color: #0066cc;
    color: white;
    border: none;
  }
  
  .version-info {
    font-size: 0.8rem;
    color: #666;
  }
  
  .app-content {
    display: flex;
    flex: 1;
    overflow: hidden;
    position: relative;
  }
  
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    width: 100%;
    color: #666;
    text-align: center;
    padding: 2rem;
  }
  
  .empty-state h2 {
    margin-bottom: 0.5rem;
    font-size: 2rem;
  }
  
  .empty-state p {
    margin-bottom: 2rem;
    font-size: 1.2rem;
  }
  
  .primary-btn {
    padding: 0.75rem 1.5rem;
    background-color: #0066cc;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    cursor: pointer;
    transition: background-color 0.2s ease;
  }
  
  .primary-btn:hover {
    background-color: #0055aa;
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
    border: 4px solid rgba(0, 0, 0, 0.1);
    border-radius: 50%;
    border-top-color: #0066cc;
    animation: spin 1s ease-in-out infinite;
    margin-bottom: 1rem;
  }
  
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  
  .loading-text {
    color: #666;
    font-size: 1.2rem;
  }
  
  .error-banner {
    background-color: #f8d7da;
    color: #721c24;
    padding: 0.5rem 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .close-btn {
    background: none;
    border: none;
    font-size: 1.2rem;
    cursor: pointer;
    color: #721c24;
  }
  
  .recent-repos {
    margin-top: 2rem;
    width: 100%;
    max-width: 600px;
  }
  
  .recent-repos h3 {
    font-size: 1.2rem;
    margin-bottom: 1rem;
    color: #333;
  }
  
  .recent-repos ul {
    list-style: none;
    padding: 0;
    margin: 0;
    text-align: left;
  }
  
  .repo-item {
    width: 100%;
    padding: 0.75rem 1rem;
    background: none;
    border: none;
    border-bottom: 1px solid #eee;
    cursor: pointer;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    text-align: left;
  }
  
  .repo-item:hover {
    background-color: #f5f5f5;
  }
  
  .repo-name {
    font-weight: 600;
    font-size: 1rem;
    color: #333;
  }
  
  .repo-path {
    font-size: 0.8rem;
    color: #666;
    margin-top: 0.25rem;
  }
</style> 