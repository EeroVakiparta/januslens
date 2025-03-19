<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { createEventDispatcher } from 'svelte';
  
  export let repoPath = '';
  
  let files = [];
  let currentPath = '';
  let isLoading = false;
  let selectedFile = null;
  let errorMessage = '';
  
  const dispatch = createEventDispatcher();
  
  $: if (repoPath && repoPath !== currentPath) {
    currentPath = repoPath;
    loadFiles(repoPath);
  }
  
  async function loadFiles(path, directory = '') {
    if (!path) return;
    
    try {
      isLoading = true;
      errorMessage = '';
      
      // Use the Tauri invoke to get the file list
      files = await invoke('list_files', { 
        repoPath: path,
        directory: directory || null
      });
      
      // If we're in a subdirectory, we need to set the currentPath
      if (directory) {
        currentPath = `${path}/${directory}`;
      } else {
        currentPath = path;
      }
    } catch (error) {
      console.error('Failed to load files:', error);
      errorMessage = `Failed to load files: ${error.message || error}`;
      files = [];
    } finally {
      isLoading = false;
    }
  }
  
  async function navigateToDirectory(directory) {
    const relPath = directory.replace(`${repoPath}/`, '');
    loadFiles(repoPath, relPath);
  }
  
  function navigateUp() {
    // Get the current relative path
    const relPath = currentPath.replace(`${repoPath}/`, '');
    
    if (relPath === '') {
      // Already at root
      return;
    }
    
    // Get parent directory
    const parts = relPath.split('/');
    parts.pop();
    const parentPath = parts.join('/');
    
    // Load the parent directory
    loadFiles(repoPath, parentPath);
  }
  
  function selectFile(file) {
    selectedFile = file;
    dispatch('fileSelected', file);
  }
  
  async function refreshFiles() {
    const relPath = currentPath.replace(`${repoPath}/`, '');
    loadFiles(repoPath, relPath === repoPath ? '' : relPath);
  }
</script>

<div class="repo-explorer">
  <div class="header">
    <h3>Files</h3>
    <button class="refresh-btn" on:click={refreshFiles} disabled={isLoading}>↻</button>
  </div>
  
  {#if errorMessage}
    <div class="error-message">{errorMessage}</div>
  {/if}
  
  <div class="path-breadcrumb">
    <button class="breadcrumb-item" on:click={() => loadFiles(repoPath)}>
      {repoPath.split('/').pop()}
    </button>
    
    {#if currentPath !== repoPath}
      {#each currentPath.replace(`${repoPath}/`, '').split('/') as part, i, array}
        <span class="breadcrumb-separator">/</span>
        <button 
          class="breadcrumb-item" 
          on:click={() => {
            if (i < array.length - 1) {
              const path = array.slice(0, i + 1).join('/');
              loadFiles(repoPath, path);
            }
          }}
        >
          {part}
        </button>
      {/each}
    {/if}
  </div>
  
  {#if isLoading}
    <div class="loading">
      <div class="mini-spinner"></div>
      <span>Loading files...</span>
    </div>
  {:else if files.length === 0}
    <div class="empty-state">No files found</div>
  {:else}
    <div class="file-list">
      {#if currentPath !== repoPath}
        <div 
          class="file-item directory"
          on:click={navigateUp}
        >
          <span class="icon">📁</span>
          <span class="name">..</span>
        </div>
      {/if}
      
      {#each files as file}
        <div 
          class="file-item {file.type} {selectedFile === file ? 'selected' : ''}"
          on:click={() => {
            if (file.type === 'directory') {
              navigateToDirectory(`${repoPath}/${file.path}`);
            } else {
              selectFile(file);
            }
          }}
        >
          <span class="icon">{file.type === 'directory' ? '📁' : '📄'}</span>
          <span class="name">{file.name}</span>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .repo-explorer {
    height: 100%;
    display: flex;
    flex-direction: column;
    padding: 0.5rem;
  }
  
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }
  
  h3 {
    margin: 0;
    font-size: 1rem;
  }
  
  .path-breadcrumb {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    padding: 0.25rem 0.5rem;
    background-color: #f0f0f0;
    border-radius: 4px;
    margin-bottom: 0.5rem;
    font-size: 0.9rem;
    max-width: 100%;
    overflow-x: auto;
  }
  
  .breadcrumb-item {
    background: none;
    border: none;
    padding: 2px 4px;
    cursor: pointer;
    color: #0066cc;
    white-space: nowrap;
  }
  
  .breadcrumb-item:hover {
    text-decoration: underline;
  }
  
  .breadcrumb-separator {
    margin: 0 2px;
    color: #666;
  }
  
  .loading {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1rem;
    color: #666;
    flex: 1;
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
    flex: 1;
  }
  
  .file-list {
    flex: 1;
    overflow-y: auto;
  }
  
  .file-item {
    padding: 0.5rem;
    display: flex;
    align-items: center;
    cursor: pointer;
    border-radius: 4px;
  }
  
  .file-item:hover {
    background-color: #f0f0f0;
  }
  
  .file-item.selected {
    background-color: #e0f0ff;
  }
  
  .icon {
    margin-right: 0.5rem;
  }
  
  .name {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  
  .directory {
    font-weight: 500;
  }
  
  .error-message {
    color: #f05050;
    font-size: 0.85rem;
    margin-bottom: 0.5rem;
    padding: 0.5rem;
    background-color: #fff0f0;
    border-radius: 4px;
  }
  
  .refresh-btn {
    background-color: #0066cc;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    padding: 0;
    cursor: pointer;
  }
  
  .refresh-btn:disabled {
    background-color: #99ccff;
    cursor: not-allowed;
  }
</style> 