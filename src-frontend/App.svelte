<script lang="ts">
  import { onMount } from 'svelte';
  import MainLayout from './components/MainLayout.svelte';
  import { Logger, LogLevel } from './services/logger';
  
  // App initialization
  let isInitialized = false;
  let error = '';
  
  onMount(async () => {
    try {
      // Initialize logger
      Logger.initialize({
        enabled: true,
        logLevel: LogLevel.DEBUG // Set to DEBUG for development
      });
      
      Logger.info('App', 'JanusLens application initializing');
      
      // Perform any other initializations here
      
      // Mark as initialized
      isInitialized = true;
      Logger.info('App', 'JanusLens application initialized successfully');
    } catch (e) {
      error = `Failed to initialize application: ${e.message || e}`;
      Logger.error('App', 'Failed to initialize application', e);
    }
  });
</script>

<main>
  {#if error}
    <div class="error-screen">
      <h1>Error</h1>
      <p>{error}</p>
    </div>
  {:else if !isInitialized}
    <div class="loading-screen">
      <h1>JanusLens</h1>
      <div class="loading-spinner"></div>
      <p>Initializing...</p>
    </div>
  {:else}
    <MainLayout />
  {/if}
</main>

<style>
  main {
    width: 100%;
    height: 100%;
  }
  
  .loading-screen, .error-screen {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    text-align: center;
    background-color: var(--bg-color, #f5f5f5);
    color: var(--text-color, #333);
  }
  
  .error-screen {
    color: var(--error-color, #d32f2f);
  }
  
  .loading-spinner {
    width: 40px;
    height: 40px;
    border: 4px solid rgba(0, 102, 204, 0.1);
    border-radius: 50%;
    border-top-color: var(--primary-color, #0066cc);
    animation: spin 1s linear infinite;
    margin: 20px 0;
  }
  
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  
  h1 {
    font-size: 2rem;
    margin-bottom: 1rem;
  }
  
  p {
    font-size: 1rem;
    max-width: 600px;
    margin: 0 auto;
  }
</style> 