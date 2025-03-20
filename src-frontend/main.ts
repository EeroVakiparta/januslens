import App from './App.svelte';
import './styles.css';

console.log("JanusLens: Initializing application...");

const appElement = document.getElementById('app');
console.log("JanusLens: App element found:", appElement);

console.log("JanusLens: Creating Svelte app instance...");
const app = new App({
  target: document.getElementById('app'),
});

console.log("JanusLens: App instance created successfully");

export default app; 