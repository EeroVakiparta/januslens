/// <reference types="svelte" />

// This file is required for TypeScript to understand Svelte components

declare namespace svelteHTML {
  interface HTMLAttributes<T> {
    'on:click'?: (event: CustomEvent<T>) => void;
    'on:change'?: (event: CustomEvent<T>) => void;
    'on:viewDiff'?: (event: CustomEvent<any>) => void;
    'on:branchSelected'?: (event: CustomEvent<any>) => void;
    'on:commitCreated'?: (event: CustomEvent<any>) => void;
  }
}

// Global type definitions for the window object
interface Window {
  __TAURI__?: {
    core?: {
      invoke: (cmd: string, args: any) => Promise<any>;
    };
  };
  __TAURI_INVOKE_HANDLER__?: (cmd: string, args: any) => Promise<any>;
} 