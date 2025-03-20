<script>
  import { Logger } from './services/logger';
  
  // Simple component with no TypeScript to avoid errors
  let title = "JanusLens";
  
  function handleClick() {
    console.log("SimpleApp: Button clicked!");
    Logger.info('SimpleApp', 'Button clicked by user');
    alert('The app is running correctly in development mode!');
  }
</script>

<div class="container">
  <h1>{title}</h1>
  <p>This is a simplified version of the app to check if it can run correctly.</p>
  <p>The TypeScript errors in the main components need to be fixed for the full app to work.</p>
  <button on:click={handleClick}>Click me to verify the app is working</button>
</div>

<style>
  .container {
    max-width: 800px;
    margin: 50px auto;
    padding: 20px;
    background-color: #0066cc;
    color: white;
    border-radius: 8px;
    text-align: center;
  }
  
  h1 {
    font-size: 32px;
    margin-bottom: 20px;
  }
  
  button {
    background-color: white;
    color: #0066cc;
    border: none;
    padding: 10px 20px;
    border-radius: 4px;
    font-size: 16px;
    cursor: pointer;
    margin-top: 20px;
  }
</style> 