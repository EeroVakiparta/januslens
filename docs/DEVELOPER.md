# JanusLens Developer Guide

Welcome to the JanusLens developer guide! This document provides detailed information for developers who want to contribute to or understand the project.

## Table of Contents
- [Development Environment Setup](#development-environment-setup)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Common Issues & Troubleshooting](#common-issues--troubleshooting)
- [Architecture Overview](#architecture-overview)
- [Testing Guidelines](#testing-guidelines)
- [Performance Considerations](#performance-considerations)

## Development Environment Setup

### Prerequisites

1. **Rust & Cargo**
   - Install Rust and Cargo using [rustup](https://rustup.rs/)
   - Minimum Rust version: 1.70+

2. **Node.js & npm**
   - Install [Node.js](https://nodejs.org/) (16+)
   - npm 8+ is recommended

3. **Tauri CLI**
   - Install the Tauri CLI: `cargo install tauri-cli`

4. **Development Tools**
   - A code editor (VS Code recommended)
   - Git

### Setting Up The Project

1. **Clone the repository**
   ```bash
   git clone https://github.com/YOUR-USERNAME/januslens.git
   cd januslens
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Run in development mode**
   ```bash
   npm run tauri dev
   ```
   This will start the development server for the frontend and launch the Tauri application.

## Project Structure

```
januslens/
├── src-frontend/         # Svelte frontend code
│   ├── components/       # Reusable UI components
│   ├── App.svelte        # Main app component
│   ├── main.ts           # App entry point
│   └── styles.css        # Global styles
├── src-tauri/            # Rust backend code
│   ├── src/              # Rust source files
│   │   ├── main.rs       # Application entry point
│   │   ├── git.rs        # Git operations
│   │   └── error.rs      # Error handling
│   ├── Cargo.toml        # Rust dependencies
│   ├── tauri.conf.json   # Tauri configuration
│   └── icons/            # Application icons
├── docs/                 # Documentation files
├── index.html            # Web entry point
├── package.json          # Node dependencies
├── tsconfig.json         # TypeScript configuration
└── vite.config.ts        # Vite configuration
```

## Development Workflow

### Frontend Development (Svelte)

1. **Component Structure**
   - All UI components should be in `src-frontend/components/`
   - Use TypeScript for type safety
   - Follow the Svelte component lifecycle patterns

2. **State Management**
   - Use Svelte stores for application state
   - Keep state logic separate from UI components when possible

3. **Communication with Rust Backend**
   - Use Tauri's `invoke` function to call Rust functions:
     ```typescript
     import { invoke } from '@tauri-apps/api/tauri';
     
     // Call a Rust function
     const result = await invoke('function_name', { param1: value1 });
     ```

### Backend Development (Rust)

1. **Tauri Commands**
   - Expose functionality to the frontend with the `#[tauri::command]` attribute
   - Example:
     ```rust
     #[tauri::command]
     fn get_repositories() -> Result<Vec<RepoInfo>, JanusError> {
         // Implementation
     }
     ```

2. **Git Operations**
   - Use the `git2` crate for Git operations
   - All Git-related code should be in `src-tauri/src/git.rs`
   - Handle errors properly using `Result` types

3. **Error Handling**
   - Use the custom `JanusError` type in `error.rs`
   - Provide meaningful error messages that can be displayed to users

### Building & Testing

1. **Development Build**
   ```bash
   npm run tauri dev
   ```

2. **Production Build**
   ```bash
   npm run tauri build
   ```

3. **Running Tests**
   ```bash
   # Frontend tests
   npm test
   
   # Backend tests
   cd src-tauri && cargo test
   ```

## Common Issues & Troubleshooting

### Tauri Build Failures

#### Icon Issues
If you encounter problems with icons during the build process:
- Ensure all icon files in `src-tauri/icons/` are valid image files in the correct format
- Check that all required icon files are present as specified in `tauri.conf.json`

#### Rust Errors
- Run `cargo check` in the `src-tauri` directory to identify Rust compilation issues
- Ensure Rust dependencies are up to date with `cargo update`

### Frontend Issues

#### TypeScript Errors
- Verify your TypeScript configuration in `tsconfig.json`
- Run `npx tsc --noEmit` to check for type errors

#### Component Issues
- Check browser console for errors
- Ensure all imports are correct
- Verify that Svelte component syntax is valid

## Architecture Overview

JanusLens follows a hybrid architecture with a Rust backend and Svelte frontend:

1. **Frontend (Svelte)**
   - Responsible for UI rendering and user interactions
   - Communicates with the Rust backend via Tauri's message passing

2. **Backend (Rust)**
   - Handles all Git operations and file system interactions
   - Exposes a set of commands that can be invoked from the frontend
   - Manages performance-critical operations

3. **Communication Flow**
   - Frontend invokes backend commands using Tauri's `invoke`
   - Backend processes the command and returns results
   - Frontend updates its state and UI based on the results

## Testing Guidelines

### Frontend Testing

- Use Jest for unit testing components
- Test UI interactions and state management
- Mock Tauri's `invoke` calls when testing components that interact with the backend

### Backend Testing

- Write unit tests for all Rust functions
- Use integration tests for Git operations
- Mock file system operations when appropriate

## Performance Considerations

JanusLens is designed for performance, especially with large Git repositories:

1. **Lazy Loading**
   - Load repository data incrementally
   - Implement pagination for commit history

2. **Background Processing**
   - Run heavy Git operations in separate threads
   - Use Rust's async/await for non-blocking operations

3. **Memory Management**
   - Minimize cloning of large data structures
   - Release resources when they're no longer needed

4. **UI Performance**
   - Use virtualized lists for displaying large datasets
   - Debounce frequent events like scrolling and input changes
   - Optimize D3.js visualizations for large graphs

---

If you have any questions or need clarification on any part of the development process, please open an issue in the GitHub repository. 