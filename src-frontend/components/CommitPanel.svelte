<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import DiffViewer from './DiffViewer.svelte';
  
  export let repoPath = '';
  
  let stagedFiles = [];
  let unstagedFiles = [];
  let commitMessage = '';
  let selectedFile = null;
  let diffContent = '';
  let isLoading = false;
  let errorMessage = '';
  
  $: if (repoPath) {
    refreshStatus();
  }
  
  async function refreshStatus() {
    if (!repoPath) return;
    
    try {
      isLoading = true;
      
      // In a real implementation, this would fetch from the backend
      // const status = await invoke('get_status', { repoPath });
      // stagedFiles = status.staged;
      // unstagedFiles = status.unstaged;
      
      // Mock data for now
      unstagedFiles = [
        { path: 'src/main.rs', status: 'modified' },
        { path: 'README.md', status: 'modified' },
        { path: 'src/new_file.rs', status: 'new' }
      ];
      
      stagedFiles = [
        { path: 'Cargo.toml', status: 'modified' }
      ];
      
    } catch (error) {
      console.error('Failed to get repository status:', error);
      errorMessage = `Failed to get repository status: ${error.message || error}`;
    } finally {
      isLoading = false;
    }
  }
  
  async function viewDiff(file) {
    selectedFile = file;
    
    try {
      // In a real implementation, this would fetch the diff from the backend
      // const diff = await invoke('get_diff', { 
      //   repoPath,
      //   filePath: file.path,
      //   staged: file.staged 
      // });
      // diffContent = diff;
      
      // Mock diff data
      if (file.status === 'modified') {
        diffContent = `diff --git a/${file.path} b/${file.path}
--- a/${file.path}
+++ b/${file.path}
@@ -1,5 +1,5 @@
 # Example File
-This is the old content
+This is the new modified content
 
 More text here
 End of file`;
      } else if (file.status === 'new') {
        diffContent = `diff --git a/${file.path} b/${file.path}
--- /dev/null
+++ b/${file.path}
@@ -0,0 +1,3 @@
+// New file
+
+// With some content`;
      }
      
    } catch (error) {
      console.error('Failed to get diff:', error);
      errorMessage = `Failed to get diff: ${error.message || error}`;
      diffContent = '';
    }
  }
  
  async function stageFile(file) {
    try {
      // In a real implementation, this would call the backend
      // await invoke('stage_file', { repoPath, filePath: file.path });
      
      // For mock, just update our lists
      unstagedFiles = unstagedFiles.filter(f => f.path !== file.path);
      stagedFiles = [...stagedFiles, { ...file, staged: true }];
      
      if (selectedFile && selectedFile.path === file.path) {
        selectedFile = { ...file, staged: true };
      }
      
    } catch (error) {
      console.error('Failed to stage file:', error);
      errorMessage = `Failed to stage file: ${error.message || error}`;
    }
  }
  
  async function unstageFile(file) {
    try {
      // In a real implementation, this would call the backend
      // await invoke('unstage_file', { repoPath, filePath: file.path });
      
      // For mock, just update our lists
      stagedFiles = stagedFiles.filter(f => f.path !== file.path);
      unstagedFiles = [...unstagedFiles, { ...file, staged: false }];
      
      if (selectedFile && selectedFile.path === file.path) {
        selectedFile = { ...file, staged: false };
      }
      
    } catch (error) {
      console.error('Failed to unstage file:', error);
      errorMessage = `Failed to unstage file: ${error.message || error}`;
    }
  }
  
  async function createCommit() {
    if (!commitMessage.trim()) {
      errorMessage = 'Commit message cannot be empty';
      return;
    }
    
    if (stagedFiles.length === 0) {
      errorMessage = 'No files staged for commit';
      return;
    }
    
    try {
      // In a real implementation, this would call the backend
      // await invoke('create_commit', { repoPath, message: commitMessage });
      
      // Mock success
      // Reset UI
      commitMessage = '';
      stagedFiles = [];
      selectedFile = null;
      diffContent = '';
      errorMessage = '';
      
      // Refresh to get new status
      refreshStatus();
      
    } catch (error) {
      console.error('Failed to create commit:', error);
      errorMessage = `Failed to create commit: ${error.message || error}`;
    }
  }
</script>

<div class="commit-panel">
  <div class="changes-section">
    <div class="section-header">
      <h3>Changes</h3>
      <button class="refresh-btn" on:click={refreshStatus}>↻</button>
    </div>
    
    {#if isLoading}
      <div class="loading">Loading changes...</div>
    {:else}
      {#if errorMessage}
        <div class="error-message">{errorMessage}</div>
      {/if}
      
      <div class="file-section">
        <h4>Staged Changes</h4>
        {#if stagedFiles.length === 0}
          <div class="empty-message">No staged changes</div>
        {:else}
          <ul class="file-list">
            {#each stagedFiles as file}
              <li class={selectedFile && selectedFile.path === file.path ? 'selected' : ''}>
                <div class="file-info" on:click={() => viewDiff(file)}>
                  <span class="file-status">{file.status}</span>
                  <span class="file-path">{file.path}</span>
                </div>
                <button 
                  class="unstage-btn"
                  on:click={(e) => {
                    e.stopPropagation();
                    unstageFile(file);
                  }}
                >
                  -
                </button>
              </li>
            {/each}
          </ul>
        {/if}
      </div>
      
      <div class="file-section">
        <h4>Unstaged Changes</h4>
        {#if unstagedFiles.length === 0}
          <div class="empty-message">No unstaged changes</div>
        {:else}
          <ul class="file-list">
            {#each unstagedFiles as file}
              <li class={selectedFile && selectedFile.path === file.path ? 'selected' : ''}>
                <div class="file-info" on:click={() => viewDiff(file)}>
                  <span class="file-status">{file.status}</span>
                  <span class="file-path">{file.path}</span>
                </div>
                <button 
                  class="stage-btn"
                  on:click={(e) => {
                    e.stopPropagation();
                    stageFile(file);
                  }}
                >
                  +
                </button>
              </li>
            {/each}
          </ul>
        {/if}
      </div>
    {/if}
  </div>
  
  <div class="diff-section">
    {#if selectedFile}
      <DiffViewer diffContent={diffContent} fileName={selectedFile.path} />
    {:else}
      <div class="empty-diff">
        <p>Select a file to view changes</p>
      </div>
    {/if}
  </div>
  
  <div class="commit-section">
    <textarea 
      bind:value={commitMessage} 
      placeholder="Commit message"
      rows="3"
    ></textarea>
    <button class="commit-btn" on:click={createCommit} disabled={stagedFiles.length === 0}>
      Commit Changes
    </button>
  </div>
</div>

<style>
  .commit-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .section-header h3 {
    margin: 0;
    font-size: 1rem;
  }
  
  .changes-section {
    flex: 0 0 auto;
    padding: 0.5rem;
    border-bottom: 1px solid #ddd;
  }
  
  .file-section {
    margin-bottom: 1rem;
  }
  
  .file-section h4 {
    margin: 0.5rem 0;
    font-size: 0.9rem;
    color: #555;
  }
  
  .file-list {
    list-style: none;
    padding: 0;
    margin: 0;
    max-height: 150px;
    overflow-y: auto;
  }
  
  .file-list li {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    margin-bottom: 0.1rem;
    cursor: pointer;
  }
  
  .file-list li:hover {
    background-color: #f0f0f0;
  }
  
  .file-list li.selected {
    background-color: #e0f0ff;
  }
  
  .file-info {
    display: flex;
    align-items: center;
    flex: 1;
  }
  
  .file-status {
    font-size: 0.75rem;
    padding: 0.1rem 0.3rem;
    border-radius: 2px;
    background-color: #eee;
    margin-right: 0.5rem;
  }
  
  .file-path {
    font-size: 0.85rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  
  .stage-btn, .unstage-btn {
    background: none;
    border: 1px solid #ddd;
    border-radius: 2px;
    width: 1.5rem;
    height: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    font-weight: bold;
    font-size: 1rem;
  }
  
  .stage-btn {
    color: #28a745;
  }
  
  .unstage-btn {
    color: #dc3545;
  }
  
  .diff-section {
    flex: 1;
    overflow-y: auto;
    padding: 0.5rem;
    border-bottom: 1px solid #ddd;
    min-height: 200px;
  }
  
  .empty-diff {
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #666;
    font-style: italic;
  }
  
  .commit-section {
    flex: 0 0 auto;
    padding: 0.5rem;
  }
  
  textarea {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    margin-bottom: 0.5rem;
    resize: vertical;
  }
  
  .commit-btn {
    background-color: #28a745;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 0.5rem 1rem;
    cursor: pointer;
    width: 100%;
  }
  
  .commit-btn:disabled {
    background-color: #6c757d;
    cursor: not-allowed;
  }
  
  .loading, .empty-message {
    padding: 0.5rem;
    color: #666;
    font-style: italic;
  }
  
  .error-message {
    color: #dc3545;
    font-size: 0.85rem;
    margin: 0.5rem 0;
    padding: 0.5rem;
    background-color: #f8d7da;
    border-radius: 4px;
  }
  
  .refresh-btn {
    background: none;
    border: 1px solid #ddd;
    border-radius: 2px;
    width: 1.5rem;
    height: 1.5rem;
    cursor: pointer;
    font-size: 1rem;
  }
</style> 