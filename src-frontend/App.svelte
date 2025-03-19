<script lang="ts">
  import { onMount } from 'svelte';
  
  let appName = "JanusLens";
  let isLoading = false;
</script>

<main>
  <div class="app-container">
    <header class="app-header">
      <h1>{appName}</h1>
      <div class="version-info">v0.1.0</div>
    </header>
    
    <div class="app-content">
      {#if isLoading}
        <div class="loading">
          <div class="spinner"></div>
          <div class="loading-text">Loading...</div>
        </div>
      {:else}
        <div class="empty-state">
          <h2>Welcome to JanusLens</h2>
          <p>This is a simplified placeholder UI</p>
          <button class="primary-btn">Open Repository</button>
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
  
  .version-info {
    font-size: 0.8rem;
    color: #666;
  }
  
  .app-content {
    display: flex;
    flex: 1;
    overflow: hidden;
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
</style> 