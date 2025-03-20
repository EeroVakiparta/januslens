<script>
  import { onMount, createEventDispatcher } from 'svelte';
  import { Logger } from '../services/logger';
  import { invoke } from '@tauri-apps/api/core';
  
  export let repoPath = '';
  
  let message = '';
  let isCommitting = false;
  let error = null;
  
  // Detect if we're in development mode without Tauri
  const isDevWithoutTauri = () => {
    return typeof window !== 'undefined' && 
           (window.__TAURI__ === undefined || 
            (typeof window.__TAURI__ === 'object' && !window.__TAURI__.core) ||
            process.env.NODE_ENV === 'development');
  };
  
  const dispatch = createEventDispatcher();
  
  // Create a commit with the current message
  async function createCommit() {
    if (!repoPath) {
      error = 'No repository selected';
      return;
    }
    
    if (!message.trim()) {
      error = 'Commit message cannot be empty';
      return;
    }
    
    Logger.info('CommitPanel', 'Creating commit', { repoPath, message });
    isCommitting = true;
    error = null;
    
    try {
      if (isDevWithoutTauri()) {
        // Mock implementation for development
        await new Promise(resolve => setTimeout(resolve, 500));
        
        // Emit event to refresh status
        dispatch('commitCreated', { message });
        
        // Clear message after successful commit
        message = '';
        Logger.info('CommitPanel', 'Mock commit created successfully');
        return;
      }
      
      // Actual backend call
      const result = await invoke('create_commit', { repoPath, message });
      
      // Emit event to refresh status
      dispatch('commitCreated', { message });
      
      // Clear message after successful commit
      message = '';
      Logger.info('CommitPanel', 'Commit created successfully', { result });
    } catch (err) {
      const errorMessage = err.message || String(err);
      error = `Failed to create commit: ${errorMessage}`;
      Logger.error('CommitPanel', 'Failed to create commit', { error: errorMessage });
    } finally {
      isCommitting = false;
    }
  }
  
  onMount(() => {
    Logger.info('CommitPanel', 'Component mounted');
  });
</script>

<div class="commit-panel" data-test-id="jl-commit-panel">
  <h3>Commit Changes</h3>
  
  {#if error}
    <div class="error" data-test-id="jl-commit-error">{error}</div>
  {/if}
  
  <div class="commit-form">
    <textarea
      placeholder="Commit message..."
      bind:value={message}
      disabled={isCommitting || !repoPath}
      data-test-id="jl-commit-message"
    ></textarea>
    
    <button
      class="commit-button"
      on:click={createCommit}
      disabled={isCommitting || !message.trim() || !repoPath}
      data-test-id="jl-commit-button"
    >
      {#if isCommitting}
        Committing...
      {:else}
        Commit
      {/if}
    </button>
  </div>
  
  {#if !repoPath}
    <div class="hint">Select a repository to commit changes.</div>
  {/if}
</div>

<style>
  .commit-panel {
    margin-top: 16px;
    padding-top: 16px;
    border-top: 1px solid var(--border-color, #ddd);
  }
  
  h3 {
    margin: 0 0 8px 0;
    font-size: 16px;
    font-weight: 500;
  }
  
  .error {
    margin-bottom: 8px;
    padding: 8px;
    color: var(--error-color, #d32f2f);
    background-color: rgba(211, 47, 47, 0.1);
    border-radius: 4px;
    font-size: 14px;
  }
  
  .commit-form {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  
  textarea {
    width: 100%;
    min-height: 80px;
    padding: 8px;
    border: 1px solid var(--border-color, #ddd);
    border-radius: 4px;
    font-family: inherit;
    font-size: 14px;
    resize: vertical;
  }
  
  textarea:focus {
    outline: none;
    border-color: var(--primary-color, #0066cc);
    box-shadow: 0 0 0 2px rgba(0, 102, 204, 0.2);
  }
  
  textarea:disabled {
    background-color: var(--disabled-bg, #f5f5f5);
    color: var(--text-disabled, #ccc);
    cursor: not-allowed;
  }
  
  .commit-button {
    padding: 8px 16px;
    background-color: var(--primary-color, #0066cc);
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  .commit-button:hover:not(:disabled) {
    background-color: var(--primary-dark, #004c99);
  }
  
  .commit-button:disabled {
    background-color: var(--disabled-bg, #cccccc);
    cursor: not-allowed;
  }
  
  .hint {
    margin-top: 8px;
    font-size: 12px;
    color: var(--text-light, #999);
    font-style: italic;
  }
</style> 