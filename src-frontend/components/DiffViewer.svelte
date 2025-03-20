<script>
  import { onMount, onDestroy } from 'svelte';
  import { Logger } from '../services/logger';
  import { invoke } from '@tauri-apps/api/core';
  
  export let repoPath = '';
  export let filePath = '';
  export let staged = false;
  
  let diffContent = '';
  let isLoading = false;
  let error = null;
  
  // Detect if we're in development mode without Tauri
  const isDevWithoutTauri = () => {
    return typeof window !== 'undefined' && 
           (window.__TAURI__ === undefined || 
            process.env.NODE_ENV === 'development');
  };
  
  // Load the diff content when props change
  $: if (repoPath && filePath) {
    loadDiff();
  }
  
  async function loadDiff() {
    if (!repoPath || !filePath) {
      return;
    }
    
    Logger.info('DiffViewer', 'Loading diff', { repoPath, filePath, staged });
    isLoading = true;
    error = null;
    
    try {
      if (isDevWithoutTauri()) {
        // Mock implementation for development
        await new Promise(resolve => setTimeout(resolve, 300));
        
        // Generate mock diff content
        diffContent = generateMockDiff(filePath, staged);
        Logger.info('DiffViewer', 'Mock diff loaded successfully');
        return;
      }
      
      // Actual backend call
      diffContent = await invoke('get_diff', { repoPath, filePath, staged });
      Logger.info('DiffViewer', 'Diff loaded successfully');
    } catch (err) {
      const errorMessage = err.message || String(err);
      error = `Failed to load diff: ${errorMessage}`;
      Logger.error('DiffViewer', 'Failed to load diff', { error: errorMessage });
    } finally {
      isLoading = false;
    }
  }
  
  function generateMockDiff(filePath, staged) {
    // Simple mock diff generation
    const fileExt = filePath.split('.').pop() || 'txt';
    const timestamp = new Date().toISOString();
    
    return `diff --git a/${filePath} b/${filePath}
index 1234567..7654321 100644
--- a/${filePath}
+++ b/${filePath}
@@ -1,5 +1,7 @@
-// Old content
-function oldFunction() {
-  return 'old';
+// New content
+function newFunction() {
+  console.log("Hello from new function");
+  return 'new';
 }
 
+// Added a comment at ${timestamp}
`;
  }
  
  // Format the diff for display (add syntax highlighting)
  function formatDiff(diff) {
    if (!diff) return [];
    
    return diff.split('\n').map(line => {
      // Determine line class based on first character
      let cssClass = '';
      if (line.startsWith('+')) {
        cssClass = 'addition';
      } else if (line.startsWith('-')) {
        cssClass = 'deletion';
      } else if (line.startsWith('@')) {
        cssClass = 'chunk-header';
      } else if (line.startsWith('diff') || line.startsWith('index') || 
                line.startsWith('---') || line.startsWith('+++')) {
        cssClass = 'meta';
      }
      
      return {
        text: line,
        cssClass
      };
    });
  }
  
  onMount(() => {
    Logger.info('DiffViewer', 'Component mounted');
  });
  
  onDestroy(() => {
    Logger.info('DiffViewer', 'Component destroyed');
  });
</script>

<div class="diff-viewer" data-test-id="jl-diff-viewer">
  <div class="diff-header">
    <h3>
      {#if filePath}
        Diff: {filePath} {staged ? '(Staged)' : '(Unstaged)'}
      {:else}
        Diff Viewer
      {/if}
    </h3>
  </div>
  
  {#if isLoading}
    <div class="loading" data-test-id="jl-diff-loading">Loading diff...</div>
  {:else if error}
    <div class="error" data-test-id="jl-diff-error">{error}</div>
  {:else if !filePath}
    <div class="empty-state" data-test-id="jl-diff-empty">
      No file selected. Click on a file in the changes list to view its diff.
    </div>
  {:else if !diffContent}
    <div class="empty-state" data-test-id="jl-diff-no-content">
      No changes to display for this file.
    </div>
  {:else}
    <div class="diff-content" data-test-id="jl-diff-content">
      <pre><code>
        {#each formatDiff(diffContent) as line}
          <div class="line {line.cssClass}">{line.text}</div>
        {/each}
      </code></pre>
    </div>
  {/if}
</div>

<style>
  .diff-viewer {
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    font-size: 14px;
  }
  
  .diff-header {
    padding: 0 8px 8px 8px;
    border-bottom: 1px solid var(--border-color, #ddd);
  }
  
  h3 {
    margin: 0;
    font-size: 16px;
    font-weight: 500;
  }
  
  .diff-content {
    flex-grow: 1;
    overflow: auto;
    background-color: var(--code-bg, #f5f5f5);
    font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
    font-size: 12px;
    line-height: 1.5;
    white-space: pre;
  }
  
  pre {
    margin: 0;
    padding: 0;
  }
  
  code {
    display: block;
    padding: 8px;
    margin: 0;
  }
  
  .line {
    white-space: pre;
  }
  
  .line.addition {
    background-color: rgba(0, 128, 0, 0.1);
    color: var(--addition-color, #22863a);
  }
  
  .line.deletion {
    background-color: rgba(255, 0, 0, 0.1);
    color: var(--deletion-color, #cb2431);
  }
  
  .line.chunk-header {
    background-color: rgba(0, 0, 255, 0.1);
    color: var(--chunk-header-color, #0366d6);
  }
  
  .line.meta {
    color: var(--meta-color, #6a737d);
  }
  
  .loading, .error, .empty-state {
    padding: 16px 8px;
    text-align: center;
    color: var(--text-muted, #666);
  }
  
  .error {
    color: var(--error-color, #d32f2f);
  }
</style> 