// Test UI entry point for development without the Tauri backend
// This allows developers to work on UI components with mock data

// Mock the Tauri API
window.__TAURI__ = true;
window.__TAURI_INVOKE_HANDLER__ = async (cmd, args) => {
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
    case 'list_branches': {
      return mockListBranches(args);
    }
    case 'checkout_branch': {
      console.log(`[MOCK TAURI] Checked out branch: ${args.branchName}`);
      return null;
    }
    case 'create_branch': {
      console.log(`[MOCK TAURI] Created branch: ${args.branchName}`);
      return { name: args.branchName, isHead: true };
    }
    case 'log_event_from_frontend': {
      console.log(`[MOCK LOGGER] ${args.level}: [${args.module}] ${args.message}`, args.data || {});
      return null;
    }
    default: {
      console.warn(`[MOCK TAURI] Unhandled command: ${cmd}`);
      return null;
    }
  }
};

// Mock implementation for get_status
function mockGetStatus(args) {
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
function mockGetDiff(args) {
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

// Mock implementation for list_branches
function mockListBranches(args) {
  return [
    { name: 'main', isHead: true },
    { name: 'develop', isHead: false },
    { name: 'feature/user-auth', isHead: false },
    { name: 'feature/file-explorer', isHead: false },
    { name: 'bugfix/issue-123', isHead: false }
  ];
}

// Initialize test UI
document.addEventListener('DOMContentLoaded', () => {
  console.log('[TEST UI] Initialized with mock Tauri API');
}); 