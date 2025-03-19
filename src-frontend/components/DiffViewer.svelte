<script lang="ts">
  export let diffContent = '';
  export let fileName = '';
  
  let diffLines = [];
  
  $: {
    if (diffContent) {
      parseDiff(diffContent);
    }
  }
  
  function parseDiff(content) {
    // Split by lines
    const lines = content.split('\n');
    
    // Process each line
    diffLines = lines.map(line => {
      let type = 'normal';
      
      if (line.startsWith('+')) {
        type = 'addition';
      } else if (line.startsWith('-')) {
        type = 'deletion';
      } else if (line.startsWith('@@')) {
        type = 'info';
      }
      
      return {
        content: line,
        type
      };
    });
  }
</script>

<div class="diff-viewer">
  {#if fileName}
    <div class="file-header">
      <h3>{fileName}</h3>
    </div>
  {/if}
  
  <div class="diff-content">
    {#each diffLines as line}
      <div class="diff-line {line.type}">
        <pre>{line.content}</pre>
      </div>
    {/each}
  </div>
</div>

<style>
  .diff-viewer {
    font-family: monospace;
    border: 1px solid #ddd;
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 1rem;
  }
  
  .file-header {
    background-color: #f0f0f0;
    padding: 0.5rem 1rem;
    border-bottom: 1px solid #ddd;
  }
  
  .file-header h3 {
    margin: 0;
    font-size: 1rem;
  }
  
  .diff-content {
    max-height: 400px;
    overflow: auto;
  }
  
  .diff-line {
    padding: 0 0.5rem;
    white-space: pre-wrap;
  }
  
  .diff-line pre {
    margin: 0;
    padding: 0.25rem 0;
  }
  
  .addition {
    background-color: #e6ffed;
  }
  
  .deletion {
    background-color: #ffeef0;
  }
  
  .info {
    background-color: #f1f8ff;
    color: #0366d6;
  }
</style> 