# JanusLens Project Plan

## Development Phases

### Phase 1: Project Setup and Core Infrastructure (2-4 weeks)
- Set up Tauri + Rust project structure
- Design basic UI layout and components
- Implement Git repository connection and basic metadata retrieval
- Create basic repository listing/management

### Phase 2: Git Graph Visualization (4-6 weeks)
- Implement core graph visualization with D3.js or Cytoscape.js
- Create branch representation
- Add commit node visualization
- Implement basic navigation and filtering
- Optimize performance for larger repositories

### Phase 3: Diff and Commit Features (4-5 weeks)
- Implement file diff viewer with syntax highlighting
- Create commit interface for staging/unstaging changes
- Add commit history viewing
- Implement file history visualization

### Phase 4: Branch Management (3-4 weeks)
- Add branch creation/deletion
- Implement merge visualization
- Create intuitive branch switching
- Add stash management

### Phase 5: Performance Optimization (Ongoing)
- Implement virtual scrolling for large repositories
- Add pagination for commit history
- Optimize memory usage for Git operations
- Improve startup and operation times

### Phase 6: UI Polish and UX Improvements (2-3 weeks)
- Refine visual aesthetics
- Add keyboard shortcuts
- Implement themes (light/dark)
- Create intuitive onboarding process

## Core Principles
1. **Performance First**: Every feature must be implemented with performance in mind
2. **Intuitive UX**: Focus on creating a clean, intuitive user experience
3. **Stability**: Ensure stable Git operations that won't corrupt repositories
4. **Minimalism**: Include only essential features that most users need

## Tech Stack Details
- **Backend**: Rust with git2-rs for Git operations
- **Framework**: Tauri
- **Frontend**: Either React, Svelte, or Vue (to be decided)
- **Styling**: TailwindCSS for efficient styling
- **Testing**: Rust tests for backend, Jest for frontend

## Success Metrics
- Cold start time under 2 seconds
- Memory usage under 100MB for medium repositories
- Smooth scrolling and navigation on repositories with 10k+ commits
- Positive user feedback on core workflows 