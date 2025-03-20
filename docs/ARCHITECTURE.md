# JanusLens Architecture

This document provides an overview of the JanusLens architecture, explaining key components and how they interact.

## Architecture Overview

JanusLens follows a hybrid desktop application architecture, combining a web-based frontend with a native Rust backend, unified through the Tauri framework.

**Note:** The architecture diagram is currently a text-based placeholder in `../assets/architecture.txt`. This will be replaced with a proper image visualization in the future.

## Key Components

### 1. Frontend (Web-based UI)

- **Framework**: Svelte with TypeScript
- **Visualization**: D3.js for Git graph visualization
- **Styling**: CSS with responsive design principles

The frontend is responsible for:
- Rendering the user interface
- Handling user interactions
- Visualizing Git data as interactive graphs
- Managing application state

### 2. Backend (Rust)

- **Framework**: Tauri
- **Git Operations**: git2-rs library
- **Error Handling**: Custom error types with contextual information

The backend is responsible for:
- Interfacing with the Git repositories
- Performing file system operations
- Running computationally intensive tasks
- Managing application security

### 3. Communication Layer

Communication between the frontend and backend happens through:
- **Tauri Commands**: JavaScript-to-Rust function calls
- **Events**: Asynchronous messaging for background operations
- **File System Access**: Controlled native file system operations

## Data Flow

1. **User Interaction** → Frontend UI events are triggered
2. **Frontend Processing** → UI state is updated, Tauri commands are invoked
3. **Backend Processing** → Git operations are performed, results are returned
4. **UI Update** → Frontend displays the results to the user

## Key Design Decisions

### Performance Optimization

- **Lazy Loading**: Data is fetched incrementally to minimize memory usage
- **Background Processing**: Heavy Git operations run in separate threads
- **Efficient Data Structures**: Minimizing cloning of large data sets
- **Virtualized Rendering**: Only visible UI elements are rendered

### Security Considerations

- **Sandboxed Execution**: Frontend code runs in a restricted environment
- **Controlled File Access**: Backend mediates all file system operations
- **Permission Model**: Explicit user permission required for sensitive operations

### Error Handling

- **Graceful Degradation**: UI remains functional when non-critical operations fail
- **Contextual Error Messages**: User-friendly error messages with technical details where appropriate
- **Recovery Mechanisms**: Auto-save and state preservation to prevent data loss

## Module Breakdown

### Frontend Modules

1. **Repository View**: Displays repository structure and files
2. **Git Graph**: Visualizes commit history and branch relationships
3. **Diff Viewer**: Shows file changes with syntax highlighting
4. **Commit Interface**: Allows staging, unstaging, and committing changes
5. **Branch Management**: Tools for creating, switching, and merging branches

### Backend Modules

1. **Repository Management**: Opening, closing, and listing repositories
2. **Git Operations**: Commit, branch, merge, and other Git operations
3. **File System Interface**: Reading and writing files safely
4. **Configuration Management**: User preferences and application settings

## Extensibility

JanusLens is designed with extensibility in mind:

1. **Plugin Architecture**: The codebase is structured to allow future plugin support
2. **Component-Based UI**: Frontend components are modular and reusable
3. **Clear API Boundaries**: Backend functionality is exposed through well-defined interfaces

## Future Architectural Considerations

As JanusLens evolves, these architectural enhancements are planned:

1. **Improved Concurrency**: Moving more operations to background threads
2. **Enhanced Caching**: Smart caching of repository data for faster loading
3. **Remote Repository Support**: Extending the architecture to work with remote repositories
4. **Cross-Platform Optimizations**: Platform-specific enhancements while maintaining code unity

---

This architecture document is a living document and will be updated as the project evolves. 