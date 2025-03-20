// Test UI entry point for development without the Tauri backend
// This allows developers to work on UI components with mock data

// Type declarations for mocking Tauri - using interface augmentation instead of redeclaring
declare global {
  interface Window {
    // No need to redeclare __TAURI__ as it's already defined in svelte-env.d.ts
  }
}

// Mock the Tauri API for v2.0.0
window.__TAURI__ = {
  core: {
    invoke: async (cmd: string, args: any) => {
      console.log(`[MOCK TAURI] Command: ${cmd}`, args);
      
      // Add a slight delay to simulate network latency
      await new Promise(resolve => setTimeout(resolve, 300));
      
      // Handle different commands
      switch (cmd) {
        case 'get_status': {
          return mockGetStatus(args);
        }
        case 'get_diff': {
          return mockGetDiff(args);
        }
        case 'stage_file': {
          console.log(`[MOCK TAURI] Staged file: ${args.filePath}`);
          return null;
        }
        case 'unstage_file': {
          console.log(`[MOCK TAURI] Unstaged file: ${args.filePath}`);
          return null;
        }
        case 'get_branches': {
          return mockListBranches(args);
        }
        case 'checkout_branch': {
          console.log(`[MOCK TAURI] Checked out branch: ${args.branchName}`);
          return null;
        }
        case 'create_branch': {
          console.log(`[MOCK TAURI] Created branch: ${args.branchName}`);
          return { name: args.branchName, is_head: true };
        }
        case 'create_commit': {
          console.log(`[MOCK TAURI] Created commit with message: ${args.message}`);
          return { 
            id: "mock-commit-id-" + Date.now(), 
            short_id: "mock123", 
            summary: args.message,
            message: args.message,
            author: "Test User",
            author_email: "test@example.com",
            time: Date.now() / 1000,
            parent_ids: ["parent-id-123"]
          };
        }
        case 'log_event_from_frontend': {
          const { entry } = args;
          // Format the log colorfully based on the level
          const style = {
            'ERROR': 'color: red; font-weight: bold',
            'WARN': 'color: orange; font-weight: bold',
            'INFO': 'color: blue',
            'DEBUG': 'color: gray',
            'TRACE': 'color: lightgray',
            'VERIFY': 'color: green'
          }[entry.level] || 'color: black';
          
          console.log(
            `%c[${entry.level}] [${entry.component}] ${entry.message}`, 
            style, 
            entry.details || {}
          );
          return null;
        }
        default: {
          console.warn(`[MOCK TAURI] Unhandled command: ${cmd}`);
          return null;
        }
      }
    }
  }
};

// Alias the old invoke handler for compatibility
window.__TAURI_INVOKE_HANDLER__ = (cmd: string, args: any) => {
  console.warn('[MOCK TAURI] Using deprecated __TAURI_INVOKE_HANDLER__, please update to use window.__TAURI__.core.invoke');
  return window.__TAURI__.core.invoke(cmd, args);
};

// Types for repository status
interface FileStatus {
  path: string;
  status: string;
}

interface RepoStatus {
  staged: FileStatus[];
  unstaged: FileStatus[];
}

// Mock implementation for get_status
function mockGetStatus(args: { repoPath: string }): RepoStatus {
  const { repoPath } = args;
  
  return {
    staged: [
      { path: 'src/main.rs', status: 'modified' },
      { path: 'README.md', status: 'modified' },
      { path: 'src/components/new_component.rs', status: 'new' }
    ],
    unstaged: [
      { path: 'src/lib.rs', status: 'modified' },
      { path: 'Cargo.toml', status: 'modified' },
      { path: 'src/models/user.rs', status: 'new' },
      { path: 'src/utils/deprecated.rs', status: 'deleted' },
      { path: '.gitignore', status: 'modified' }
    ]
  };
}

// Mock implementation for get_diff
function mockGetDiff(args: { filePath: string, staged: boolean }): string {
  const { filePath, staged } = args;
  
  if (filePath.endsWith('.rs')) {
    return `diff --git a/${filePath} b/${filePath}
index abcdef1..1234567 100644
--- a/${filePath}
+++ b/${filePath}
@@ -10,7 +10,7 @@ fn main() {
    // Some code here
-    let x = 5;
+    let x = 10;
    
    println!("The value is: {}", x);
}
@@ -25,6 +25,9 @@ fn some_function() {
    let result = 42;
+   // This is a new comment
+   // That spans multiple lines
+   let new_variable = "Hello, world!";
    
    return result;
}`;
  } else if (filePath === 'Cargo.toml') {
    return `diff --git a/Cargo.toml b/Cargo.toml
index 9876543..abcdef1 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -5,6 +5,9 @@ version = "0.1.0"
edition = "2021"

[dependencies]
+serde = { version = "1.0", features = ["derive"] }
+serde_json = "1.0"
+log = "0.4"

[dev-dependencies]
tempfile = "3.2"`;
  } else if (filePath.endsWith('.md')) {
    return `diff --git a/${filePath} b/${filePath}
index 9876543..abcdef1 100644
--- a/${filePath}
+++ b/${filePath}
@@ -1,5 +1,7 @@
# JanusLens

+A modern Git UI built with Rust and Tauri.
+
## Features

* Fast and responsive UI
@@ -10,3 +12,6 @@
* Branch management
* Commit history
* Diff viewer
+* Pull requests
+* Code reviews
+* And more!`;
  } else {
    return `diff --git a/${filePath} b/${filePath}
index 9876543..abcdef1 100644
--- a/${filePath}
+++ b/${filePath}
@@ -1,3 +1,4 @@
+# Added this line
Some generic content
More generic content
Even more generic content`;
  }
}

interface BranchInfo {
  name: string;
  is_head: boolean;
  upstream?: string | null;
  commit_id?: string;
}

// Mock implementation for list_branches
function mockListBranches(args: { repoPath: string }): BranchInfo[] {
  return [
    { name: 'main', is_head: true, upstream: 'origin/main', commit_id: 'abc123' },
    { name: 'develop', is_head: false, upstream: 'origin/develop', commit_id: 'def456' },
    { name: 'feature/user-auth', is_head: false, upstream: null, commit_id: 'ghi789' },
    { name: 'feature/file-explorer', is_head: false, upstream: null, commit_id: 'jkl012' },
    { name: 'bugfix/issue-123', is_head: false, upstream: null, commit_id: 'mno345' }
  ];
}

// Initialize test UI
document.addEventListener('DOMContentLoaded', () => {
  console.log('[TEST UI] Initialized with mock Tauri API');
});

export {}; 