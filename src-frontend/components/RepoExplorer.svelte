<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  
  export let repoPath = '';
  
  let files = [];
  let currentPath = '';
  let isLoading = false;
  let selectedFile = null;
  
  $: if (repoPath && repoPath !== currentPath) {
    currentPath = repoPath;
    loadFiles(repoPath);
  }
  
  async function loadFiles(path) {
    if (!path) return;
    
    try {
      isLoading = true;
      
      // This would be a real function in a full implementation
      // For now we'll mock some data
      // files = await invoke('list_files', { repoPath: path });
      
      // Mocked data
      files = [
        { name: 'src', type: 'directory', path: `${path}/src` },
        { name: 'package.json', type: 'file', path: `${path}/package.json` },
        { name: 'README.md', type: 'file', path: `${path}/README.md` },
        { name: '.gitignore', type: 'file', path: `${path}/.gitignore` },
      ];
    } catch (error) {
      console.error('Failed to load files:', error);
    } finally {
      isLoading = false;
    }
  }
  
  async function navigateToDirectory(path) {
    loadFiles(path);
  }
  
  function selectFile(file) {
    selectedFile = file;
    // In a real implementation, we would dispatch an event here
    // to notify the parent component
  }
</script>

<div class="repo-explorer">
  <h3>Files</h3>
  
  {#if isLoading}
    <div class="loading">Loading files...</div>
  {:else}
    <div class="file-list">
      {#if currentPath !== repoPath}
        <div 
          class="file-item directory"
          on:click={() => {
            const parentPath = currentPath.split('/').slice(0, -1).join('/');
            navigateToDirectory(parentPath || repoPath);
          }}
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
              navigateToDirectory(file.path);
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
  }
  
  h3 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
  }
  
  .loading {
    padding: 1rem;
    color: #666;
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
</style> 