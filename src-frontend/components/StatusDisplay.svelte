<script>
  import { onMount, createEventDispatcher } from 'svelte';
  import { Logger } from '../services/logger';
  import { invoke } from '@tauri-apps/api/core';
  
  export let repoPath = '';
  
  // File status data
  let stagedFiles = [];
  let unstagedFiles = [];
  let isLoading = false;
  let error = null;
  
  const dispatch = createEventDispatcher();
  
  // Update status when repo path changes
  $: if (repoPath) {
    fetchStatus();
  }
  
  // Detect if we're in development mode without Tauri
  const isDevWithoutTauri = () => {
    return typeof window !== 'undefined' && 
           (window.__TAURI__ === undefined || 
            process.env.NODE_ENV === 'development');
  };
  
  async function fetchStatus() {
    if (!repoPath) return;
    
    Logger.info('StatusDisplay', 'Fetching status', { repoPath });
    isLoading = true;
    error = null;
    
    try {
      if (isDevWithoutTauri()) {
        // Mock data for development
        await new Promise(resolve => setTimeout(resolve, 300));
        
        stagedFiles = [
          { path: 'src/main.rs', status: 'modified' }
        ];
        
        unstagedFiles = [
          { path: 'Cargo.toml', status: 'modified' },
          { path: 'README.md', status: 'modified' },
          { path: 'src/lib.rs', status: 'new' }
        ];
        
        Logger.info('StatusDisplay', 'Loaded mock status', { 
          staged: stagedFiles.length, 
          unstaged: unstagedFiles.length 
        });
        return;
      }
      
      // Actual backend call
      const status = await invoke('get_status', { repoPath });
      stagedFiles = status.staged;
      unstagedFiles = status.unstaged;
      
      Logger.info('StatusDisplay', 'Status loaded successfully', {
        staged: stagedFiles.length,
        unstaged: unstagedFiles.length
      });
    } catch (err) {
      const errorMessage = err.message || String(err);
      error = `Failed to load status: ${errorMessage}`;
      stagedFiles = [];
      unstagedFiles = [];
      Logger.error('StatusDisplay', 'Failed to load status', { error: errorMessage });
    } finally {
      isLoading = false;
    }
  }
  
  // View diff for a file
  function viewDiff(path, staged) {
    Logger.info('StatusDisplay', 'View diff requested', { path, staged });
    dispatch('viewDiff', { path, staged });
  }
  
  // Stage a file
  async function stageFile(path) {
    if (!repoPath || !path) return;
    
    Logger.info('StatusDisplay', 'Staging file', { repoPath, path });
    
    try {
      if (isDevWithoutTauri()) {
        // Mock implementation for development
        await new Promise(resolve => setTimeout(resolve, 200));
        
        // Move file from unstaged to staged in the UI
        const fileToStage = unstagedFiles.find(f => f.path === path);
        if (fileToStage) {
          unstagedFiles = unstagedFiles.filter(f => f.path !== path);
          stagedFiles = [...stagedFiles, fileToStage];
        }
        
        Logger.info('StatusDisplay', 'Mock file staged successfully', { path });
        return;
      }
      
      // Actual backend call
      await invoke('stage_file', { repoPath, filePath: path });
      
      // Refresh status
      fetchStatus();
      
      Logger.info('StatusDisplay', 'File staged successfully', { path });
    } catch (err) {
      const errorMessage = err.message || String(err);
      error = `Failed to stage file: ${errorMessage}`;
      Logger.error('StatusDisplay', 'Failed to stage file', { path, error: errorMessage });
    }
  }
  
  // Unstage a file
  async function unstageFile(path) {
    if (!repoPath || !path) return;
    
    Logger.info('StatusDisplay', 'Unstaging file', { repoPath, path });
    
    try {
      if (isDevWithoutTauri()) {
        // Mock implementation for development
        await new Promise(resolve => setTimeout(resolve, 200));
        
        // Move file from staged to unstaged in the UI
        const fileToUnstage = stagedFiles.find(f => f.path === path);
        if (fileToUnstage) {
          stagedFiles = stagedFiles.filter(f => f.path !== path);
          unstagedFiles = [...unstagedFiles, fileToUnstage];
        }
        
        Logger.info('StatusDisplay', 'Mock file unstaged successfully', { path });
        return;
      }
      
      // Actual backend call
      await invoke('unstage_file', { repoPath, filePath: path });
      
      // Refresh status
      fetchStatus();
      
      Logger.info('StatusDisplay', 'File unstaged successfully', { path });
    } catch (err) {
      const errorMessage = err.message || String(err);
      error = `Failed to unstage file: ${errorMessage}`;
      Logger.error('StatusDisplay', 'Failed to unstage file', { path, error: errorMessage });
    }
  }
  
  // Get a status icon based on the file status
  function getStatusIcon(status) {
    switch (status) {
      case 'new': return '➕';
      case 'modified': return '✏️';
      case 'deleted': return '🗑️';
      case 'renamed': return '📝';
      default: return '❓';
    }
  }
  
  onMount(() => {
    Logger.info('StatusDisplay', 'Component mounted');
  });
</script>

<div class="status-display" data-test-id="jl-status-display">
  <div class="status-header">
    <h3>Changes</h3>
    <button 
      class="refresh-button" 
      on:click={fetchStatus} 
      disabled={isLoading}
      data-test-id="jl-refresh-status-btn"
    >
      ↻
    </button>
  </div>
  
  {#if isLoading}
    <div class="loading" data-test-id="jl-status-loading">Loading repository status...</div>
  {:else if error}
    <div class="error" data-test-id="jl-status-error">{error}</div>
  {:else if stagedFiles.length === 0 && unstagedFiles.length === 0}
    <div class="empty-state" data-test-id="jl-status-empty">
      No changes detected.
      {#if !repoPath}
        <div class="hint">Select a repository first.</div>
      {/if}
    </div>
  {:else}
    <!-- Staged Changes -->
    <div class="files-section" data-test-id="jl-staged-files-section">
      <div class="section-header">
        <span>Staged Changes ({stagedFiles.length})</span>
        {#if stagedFiles.length > 0}
          <button 
            class="section-action" 
            on:click={() => { stagedFiles = []; unstagedFiles = [...stagedFiles, ...unstagedFiles]; }}
            data-test-id="jl-unstage-all-btn"
          >
            Unstage All
          </button>
        {/if}
      </div>
      
      {#if stagedFiles.length === 0}
        <div class="empty-section">No staged changes</div>
      {:else}
        <ul class="file-list">
          {#each stagedFiles as file}
            <li 
              class="file-item" 
              data-test-id="jl-staged-file"
              data-file-path={file.path}
              data-file-status={file.status}
            >
              <div class="file-checkbox">
                <input 
                  type="checkbox" 
                  checked={true} 
                  on:change={() => unstageFile(file.path)}
                  data-test-id="jl-file-checkbox"
                />
              </div>
              <div class="file-icon">{getStatusIcon(file.status)}</div>
              <div class="file-path" on:click={() => viewDiff(file.path, true)}>{file.path}</div>
              <button 
                class="file-action" 
                on:click={() => unstageFile(file.path)}
                data-test-id="jl-unstage-file-btn"
              >
                ↓
              </button>
            </li>
          {/each}
        </ul>
      {/if}
    </div>
    
    <!-- Unstaged Changes -->
    <div class="files-section" data-test-id="jl-unstaged-files-section">
      <div class="section-header">
        <span>Unstaged Changes ({unstagedFiles.length})</span>
        {#if unstagedFiles.length > 0}
          <button 
            class="section-action" 
            on:click={() => { stagedFiles = [...stagedFiles, ...unstagedFiles]; unstagedFiles = []; }}
            data-test-id="jl-stage-all-btn"
          >
            Stage All
          </button>
        {/if}
      </div>
      
      {#if unstagedFiles.length === 0}
        <div class="empty-section">No unstaged changes</div>
      {:else}
        <ul class="file-list">
          {#each unstagedFiles as file}
            <li 
              class="file-item" 
              data-test-id="jl-unstaged-file"
              data-file-path={file.path}
              data-file-status={file.status}
            >
              <div class="file-checkbox">
                <input 
                  type="checkbox" 
                  checked={false} 
                  on:change={() => stageFile(file.path)}
                  data-test-id="jl-file-checkbox"
                />
              </div>
              <div class="file-icon">{getStatusIcon(file.status)}</div>
              <div class="file-path" on:click={() => viewDiff(file.path, false)}>{file.path}</div>
              <button 
                class="file-action" 
                on:click={() => stageFile(file.path)}
                data-test-id="jl-stage-file-btn"
              >
                ↑
              </button>
            </li>
          {/each}
        </ul>
      {/if}
    </div>
  {/if}
</div>

<style>
  .status-display {
    height: 100%;
    overflow-y: auto;
    font-size: 14px;
  }
  
  .status-header {
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
  
  .files-section {
    margin-top: 16px;
  }
  
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 4px 8px;
    font-size: 12px;
    font-weight: bold;
    color: var(--text-muted, #666);
  }
  
  .section-action {
    background: none;
    border: none;
    padding: 2px 4px;
    font-size: 11px;
    color: var(--primary-color, #0066cc);
    cursor: pointer;
    transition: color 0.2s;
  }
  
  .section-action:hover {
    text-decoration: underline;
  }
  
  .file-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  
  .file-item {
    display: flex;
    align-items: center;
    padding: 4px 8px;
    border-radius: 4px;
    margin: 2px 0;
    transition: background-color 0.2s;
  }
  
  .file-item:hover {
    background-color: var(--item-hover-bg, #f0f0f0);
  }
  
  .file-item.selected {
    background-color: var(--item-selected-bg, #e6f7ff);
  }
  
  .file-checkbox {
    margin-right: 4px;
  }
  
  .file-icon {
    margin-right: 8px;
    width: 16px;
    text-align: center;
  }
  
  .file-path {
    flex-grow: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    cursor: pointer;
  }
  
  .file-path:hover {
    text-decoration: underline;
    color: var(--primary-color, #0066cc);
  }
  
  .file-action {
    background: none;
    border: none;
    cursor: pointer;
    padding: 0 4px;
    color: var(--text-muted, #666);
    transition: color 0.2s;
  }
  
  .file-action:hover {
    color: var(--primary-color, #0066cc);
  }
  
  .loading, .error, .empty-state, .empty-section {
    padding: 16px 8px;
    text-align: center;
    color: var(--text-muted, #666);
  }
  
  .error {
    color: var(--error-color, #d32f2f);
  }
  
  .empty-section {
    padding: 8px;
    font-size: 12px;
    font-style: italic;
  }
  
  .hint {
    font-size: 12px;
    margin-top: 8px;
    color: var(--text-light, #999);
  }
</style> 