<script>
  import { onMount, onDestroy } from 'svelte';
  import { Logger } from '../services/logger';
  import { invoke } from '@tauri-apps/api/core';
  import RepoExplorer from './RepoExplorer.svelte';
  import BranchManager from './BranchManager.svelte';
  import CommitPanel from './CommitPanel.svelte';
  import GitGraph from './GitGraph.svelte';
  import BranchList from './BranchList.svelte';
  import StatusDisplay from './StatusDisplay.svelte';
  import DiffViewer from './DiffViewer.svelte';
  
  // Panel sizes (percentage of available space)
  let leftPanelWidth = 20;
  let rightPanelWidth = 25;
  
  // Resizing state
  let isResizingLeft = false;
  let isResizingRight = false;
  let startX = 0;
  let startLeftWidth = 0;
  let startRightWidth = 0;
  
  // Active repository tab
  let activeTabIndex = 0;
  
  // Mock repository tabs for initial development
  let repoTabs = [
    { id: '1', name: 'JanusLens', path: '/path/to/januslens' },
    { id: '2', name: 'Project 2', path: '/path/to/project2' }
  ];
  
  // Center panel view state
  let centerPanelView = 'git-graph'; // Values: 'git-graph' or 'diff-viewer'
  let diffViewFile = '';
  let diffViewStaged = false;
  
  // Mock data for GitGraph component
  let mockCommits = [
    {
      id: "commit1",
      short_id: "abc123",
      summary: "Initial commit",
      message: "Initial commit",
      author: "John Doe",
      author_email: "john@example.com",
      time: Date.now() / 1000 - 3600 * 24 * 7,
      parent_ids: []
    },
    {
      id: "commit2",
      short_id: "def456",
      summary: "Add basic layout",
      message: "Add basic layout and component structure",
      author: "Jane Smith",
      author_email: "jane@example.com",
      time: Date.now() / 1000 - 3600 * 24 * 6,
      parent_ids: ["commit1"]
    },
    {
      id: "commit3",
      short_id: "ghi789",
      summary: "Implement Git operations",
      message: "Implement basic Git operations on backend",
      author: "John Doe",
      author_email: "john@example.com",
      time: Date.now() / 1000 - 3600 * 24 * 5,
      parent_ids: ["commit2"]
    },
    {
      id: "commit4",
      short_id: "jkl012",
      summary: "Add branch visualization",
      message: "Add visualization for branches in graph",
      author: "Jane Smith",
      author_email: "jane@example.com",
      time: Date.now() / 1000 - 3600 * 24 * 4,
      parent_ids: ["commit3"]
    },
    {
      id: "commit5",
      short_id: "mno345",
      summary: "Create feature branch",
      message: "Create feature branch for new UI",
      author: "John Doe",
      author_email: "john@example.com",
      time: Date.now() / 1000 - 3600 * 24 * 3,
      parent_ids: ["commit4"]
    },
    {
      id: "commit6",
      short_id: "pqr678",
      summary: "Implement new UI",
      message: "Implement new UI design",
      author: "Jane Smith",
      author_email: "jane@example.com",
      time: Date.now() / 1000 - 3600 * 24 * 2,
      parent_ids: ["commit5"]
    },
    {
      id: "commit7",
      short_id: "stu901",
      summary: "Fix bug in UI",
      message: "Fix critical bug in UI layout",
      author: "John Doe",
      author_email: "john@example.com",
      time: Date.now() / 1000 - 3600 * 24 * 1,
      parent_ids: ["commit6"]
    }
  ];
  
  let mockBranches = [
    {
      name: "main",
      is_head: true,
      upstream: "origin/main",
      commit_id: "commit7"
    },
    {
      name: "feature/new-ui",
      is_head: false,
      upstream: null,
      commit_id: "commit5"
    },
    {
      name: "develop",
      is_head: false,
      upstream: "origin/develop",
      commit_id: "commit4"
    }
  ];
  
  // Component references
  let statusDisplayComponent;
  
  // Handle mouse down on resize handle
  function startResizeLeft(e) {
    isResizingLeft = true;
    startX = e.clientX;
    startLeftWidth = leftPanelWidth;
    Logger.debug('MainLayout', 'Start resizing left panel', { startX, startLeftWidth });
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', stopResize);
    e.preventDefault();
  }
  
  function startResizeRight(e) {
    isResizingRight = true;
    startX = e.clientX;
    startRightWidth = rightPanelWidth;
    Logger.debug('MainLayout', 'Start resizing right panel', { startX, startRightWidth });
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', stopResize);
    e.preventDefault();
  }
  
  // Handle mouse movement
  function handleMouseMove(e) {
    if (isResizingLeft) {
      const containerWidth = document.querySelector('.main-layout')?.clientWidth || 1000;
      const deltaX = e.clientX - startX;
      const deltaPercent = (deltaX / containerWidth) * 100;
      leftPanelWidth = Math.max(10, Math.min(40, startLeftWidth + deltaPercent));
      Logger.trace('MainLayout', 'Resizing left panel', { deltaX, leftPanelWidth });
    } else if (isResizingRight) {
      const containerWidth = document.querySelector('.main-layout')?.clientWidth || 1000;
      const deltaX = startX - e.clientX;
      const deltaPercent = (deltaX / containerWidth) * 100;
      rightPanelWidth = Math.max(15, Math.min(50, startRightWidth + deltaPercent));
      Logger.trace('MainLayout', 'Resizing right panel', { deltaX, rightPanelWidth });
    }
  }
  
  // Handle mouse up
  function stopResize() {
    if (isResizingLeft || isResizingRight) {
      Logger.debug('MainLayout', 'Stop resizing panels', { leftPanelWidth, rightPanelWidth });
    }
    isResizingLeft = false;
    isResizingRight = false;
    window.removeEventListener('mousemove', handleMouseMove);
    window.removeEventListener('mouseup', stopResize);
  }
  
  // Switch to a different repository tab
  function switchTab(index) {
    Logger.info('MainLayout', 'Switching repository tab', { 
      from: activeTabIndex, 
      to: index, 
      repo: repoTabs[index]?.name 
    });
    activeTabIndex = index;
  }
  
  // Add a new repository tab (mock implementation)
  function addTab() {
    // In development mode, just add a mock repository
    if (typeof window === 'undefined' || 
        window.__TAURI__ === undefined || 
        process.env.NODE_ENV === 'development') {
      const newId = (repoTabs.length + 1).toString();
      repoTabs = [...repoTabs, { 
        id: newId, 
        name: `New Repo ${newId}`, 
        path: `/path/to/repo${newId}` 
      }];
      activeTabIndex = repoTabs.length - 1;
      Logger.info('MainLayout', 'Added mock repository tab', { 
        id: newId, 
        totalTabs: repoTabs.length 
      });
    } else {
      // In production, open a directory dialog
      openRepositoryDialog();
    }
  }
  
  // Open a repository dialog to select a Git repository
  async function openRepositoryDialog() {
    Logger.info('MainLayout', 'Opening repository dialog');
    
    try {
      // In development mode, simply add a mock repository
      // This avoids the import error with @tauri-apps/api/dialog
      const mockPath = `/path/to/mock-repo-${new Date().getTime()}`;
      const mockName = `Mock Repository ${repoTabs.length + 1}`;
      
      // Add a new tab for this mock repository
      repoTabs = [...repoTabs, { 
        id: mockPath, 
        name: mockName, 
        path: mockPath 
      }];
      activeTabIndex = repoTabs.length - 1;
      
      Logger.info('MainLayout', 'Mock repository added successfully', { 
        name: mockName, 
        path: mockPath 
      });
      
      // When running with actual Tauri in production, we'd use:
      // const { open } = await import('@tauri-apps/api/dialog');
      // const selected = await open({
      //   directory: true,
      //   multiple: false,
      //   title: 'Select Git Repository'
      // });
      // 
      // if (selected) {
      //   const repoPath = selected;
      //   const isGitRepo = await invoke('is_git_repository', { path: repoPath });
      //   ...
      // }
    } catch (err) {
      Logger.error('MainLayout', 'Failed to open repository', { error: err.message || err });
      // Show an error message to the user (in a real app)
    }
  }
  
  // Close a repository tab
  function closeTab(index, event) {
    event.stopPropagation(); // Prevent tab switching when clicking close button
    
    Logger.info('MainLayout', 'Closing repository tab', { 
      index, 
      repo: repoTabs[index]?.name 
    });
    
    repoTabs = repoTabs.filter((_, i) => i !== index);
    
    // Adjust active tab if necessary
    if (index === activeTabIndex) {
      activeTabIndex = Math.min(index, repoTabs.length - 1);
    } else if (index < activeTabIndex) {
      activeTabIndex--;
    }
  }
  
  // Handle branch selection
  function handleBranchSelected(event) {
    const branch = event.detail;
    Logger.info('MainLayout', 'Branch selected from BranchList', { branch: branch.name });
    // In a real implementation, we would handle the branch checkout here
  }
  
  // Handle file diff view request
  function handleViewDiff(event) {
    const { path, staged } = event.detail;
    Logger.info('MainLayout', 'View diff requested', { path, staged });
    
    // Update state to show diff in center panel
    diffViewFile = path;
    diffViewStaged = staged;
    centerPanelView = 'diff-viewer';
  }
  
  // Switch back to Git Graph view
  function showGitGraph() {
    Logger.info('MainLayout', 'Switching to Git Graph view');
    centerPanelView = 'git-graph';
    diffViewFile = '';
  }
  
  // Handle commit creation
  function handleCommitCreated(event) {
    const { message } = event.detail;
    Logger.info('MainLayout', 'Commit created', { message });
    
    // Refresh the status display to show updated state
    if (statusDisplayComponent) {
      statusDisplayComponent.fetchStatus();
    }
    
    // Switch back to git graph view to show the new commit
    showGitGraph();
  }
  
  // Handle commit selection in the GitGraph
  function handleCommitSelected(event) {
    const commit = event.detail;
    Logger.info('MainLayout', 'Commit selected in GitGraph', { id: commit.id, summary: commit.summary });
    // You could show more details or load specific information based on the commit
  }
  
  // Handle branch selection in the GitGraph
  function handleBranchSelectedInGraph(event) {
    const branch = event.detail;
    Logger.info('MainLayout', 'Branch selected in GitGraph', { name: branch.name });
    
    // In a real implementation, this would checkout the branch
    // For now, just log it
    Logger.info('MainLayout', 'Would checkout branch', { name: branch.name });
  }
  
  // Handle showing commit details/changes
  function handleShowCommitDetails(event) {
    const commit = event.detail;
    Logger.info('MainLayout', 'Showing commit details', { id: commit.id });
    
    // In a real implementation, this might load a diff view for the commit
    // For now, just log it
    Logger.info('MainLayout', 'Would show commit changes', { id: commit.id });
  }
  
  // Load commits for the current repository
  async function loadCommits() {
    if (!repoTabs[activeTabIndex]) return;
    
    const repoPath = repoTabs[activeTabIndex].path;
    Logger.info('MainLayout', 'Loading commits', { repoPath });
    
    try {
      // Check if we're in development mode without Tauri
      if (typeof window === 'undefined' || 
          window.__TAURI__ === undefined || 
          process.env.NODE_ENV === 'development') {
        Logger.debug('MainLayout', 'Using mock data for commits in development mode');
        // We're already using mock data
        return;
      }
      
      // In a real app, we would fetch actual commits from the backend
      const commits = await invoke('get_commits', { 
        repoPath,
        branchName: null, // All branches
        limit: 50 // Reasonable limit
      });
      
      const branches = await invoke('get_branches', { repoPath });
      
      // Update the mock data with real data
      mockCommits = commits;
      mockBranches = branches;
      
      Logger.info('MainLayout', 'Commits loaded successfully', { 
        count: commits.length,
        branches: branches.length 
      });
    } catch (err) {
      Logger.error('MainLayout', 'Failed to load commits', { error: err.message || err });
      // Keep using mock data if there's an error
    }
  }
  
  onMount(() => {
    Logger.info('MainLayout', 'Component mounted');
    
    // Load commits when the component mounts
    loadCommits();
  });
  
  onDestroy(() => {
    Logger.info('MainLayout', 'Component destroyed');
    window.removeEventListener('mousemove', handleMouseMove);
    window.removeEventListener('mouseup', stopResize);
  });
</script>

<div class="main-layout" data-test-id="jl-main-layout">
  <!-- Repository Tab Bar -->
  <div class="repo-tabs-bar" data-test-id="jl-repo-tabs-bar">
    {#each repoTabs as tab, index}
      <div 
        class="repo-tab {index === activeTabIndex ? 'active' : ''}" 
        on:click={() => switchTab(index)}
        data-test-id="jl-repo-tab"
        data-active={index === activeTabIndex}
      >
        <span class="repo-name">{tab.name}</span>
        <button 
          class="close-tab" 
          on:click={(e) => closeTab(index, e)}
          data-test-id="jl-close-tab-btn"
        >×</button>
      </div>
    {/each}
    <div class="add-tab" on:click={addTab} data-test-id="jl-add-tab-btn">
      <span>+</span>
    </div>
  </div>
  
  <!-- Main Content Area -->
  <div class="content-area">
    <!-- Left Panel (Branches, etc.) -->
    <div class="panel left-panel" style="width: {leftPanelWidth}%" data-test-id="jl-left-panel">
      <div class="panel-content">
        <BranchList 
          repoPath={repoTabs[activeTabIndex]?.path || ''} 
          on:branchSelected={handleBranchSelected}
        />
      </div>
      <div class="resize-handle left" on:mousedown={startResizeLeft} data-test-id="jl-left-resize-handle"></div>
    </div>
    
    <!-- Center Panel (Git Graph / Diff Viewer) -->
    <div 
      class="panel center-panel" 
      style="width: calc(100% - {leftPanelWidth}% - {rightPanelWidth}%)"
      data-test-id="jl-center-panel"
    >
      <div class="panel-content">
        {#if centerPanelView === 'git-graph'}
          <GitGraph 
            commits={mockCommits} 
            branches={mockBranches}
            on:commitSelected={handleCommitSelected}
            on:branchSelected={handleBranchSelectedInGraph}
            on:showCommitDetails={handleShowCommitDetails}
          />
        {:else if centerPanelView === 'diff-viewer'}
          <div class="center-panel-header">
            <button 
              class="back-button" 
              on:click={showGitGraph}
              data-test-id="jl-back-to-graph-btn"
            >
              ← Back to Graph
            </button>
          </div>
          <DiffViewer
            repoPath={repoTabs[activeTabIndex]?.path || ''}
            filePath={diffViewFile}
            staged={diffViewStaged}
          />
        {/if}
      </div>
    </div>
    
    <!-- Right Panel (Changes, Commit) -->
    <div class="panel right-panel" style="width: {rightPanelWidth}%" data-test-id="jl-right-panel">
      <div class="resize-handle right" on:mousedown={startResizeRight} data-test-id="jl-right-resize-handle"></div>
      <div class="panel-content">
        <StatusDisplay 
          repoPath={repoTabs[activeTabIndex]?.path || ''} 
          on:viewDiff={handleViewDiff}
          bind:this={statusDisplayComponent}
        />
        <CommitPanel 
          repoPath={repoTabs[activeTabIndex]?.path || ''} 
          on:commitCreated={handleCommitCreated}
        />
      </div>
    </div>
  </div>
  
  <!-- Status Bar -->
  <div class="status-bar" data-test-id="jl-status-bar">
    {#if repoTabs[activeTabIndex]}
      <div class="status-item">
        <span class="label">Repository:</span>
        <span class="value">{repoTabs[activeTabIndex].path}</span>
      </div>
      <div class="status-item">
        <span class="label">Branch:</span>
        <span class="value">main</span>
      </div>
    {:else}
      <div class="status-item">
        <span class="value">No repository open</span>
      </div>
    {/if}
  </div>
</div>

<style>
  .main-layout {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
    background-color: var(--bg-color, #f5f5f5);
    color: var(--text-color, #333);
  }
  
  /* Repository Tab Bar */
  .repo-tabs-bar {
    display: flex;
    height: 40px;
    background-color: var(--tab-bar-bg, #e0e0e0);
    border-bottom: 1px solid var(--border-color, #ccc);
    user-select: none;
  }
  
  .repo-tab {
    display: flex;
    align-items: center;
    padding: 0 16px;
    height: 100%;
    min-width: 120px;
    max-width: 200px;
    border-right: 1px solid var(--border-color, #ccc);
    cursor: pointer;
    background-color: var(--tab-bg, #dadada);
    transition: background-color 0.2s;
    position: relative;
  }
  
  .repo-tab.active {
    background-color: var(--tab-active-bg, #f5f5f5);
    border-bottom: 2px solid var(--primary-color, #0066cc);
  }
  
  .repo-tab:hover {
    background-color: var(--tab-hover-bg, #e5e5e5);
  }
  
  .repo-name {
    flex-grow: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  
  .close-tab {
    background: none;
    border: none;
    font-size: 16px;
    cursor: pointer;
    padding: 0 5px;
    color: var(--text-light, #666);
    transition: color 0.2s;
  }
  
  .close-tab:hover {
    color: var(--text-color, #333);
  }
  
  .add-tab {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    cursor: pointer;
    color: var(--primary-color, #0066cc);
    font-size: 20px;
    transition: background-color 0.2s;
  }
  
  .add-tab:hover {
    background-color: var(--tab-hover-bg, #e5e5e5);
  }
  
  /* Main Content Area */
  .content-area {
    display: flex;
    flex-grow: 1;
    position: relative;
    overflow: hidden;
  }
  
  .panel {
    height: 100%;
    position: relative;
    overflow: hidden;
  }
  
  .panel-content {
    padding: 16px;
    height: 100%;
    overflow: auto;
  }
  
  .left-panel {
    background-color: var(--panel-bg, #f0f0f0);
    border-right: 1px solid var(--border-color, #ccc);
  }
  
  .center-panel {
    background-color: var(--panel-bg, #ffffff);
  }
  
  .right-panel {
    background-color: var(--panel-bg, #f0f0f0);
    border-left: 1px solid var(--border-color, #ccc);
  }
  
  /* Resize handles */
  .resize-handle {
    position: absolute;
    width: 8px;
    height: 100%;
    top: 0;
    cursor: col-resize;
    background-color: transparent;
    transition: background-color 0.2s;
    z-index: 10;
  }
  
  .resize-handle:hover,
  .resize-handle:active {
    background-color: var(--primary-light, rgba(0, 102, 204, 0.1));
  }
  
  .resize-handle.left {
    right: 0;
  }
  
  .resize-handle.right {
    left: 0;
  }
  
  /* Status Bar */
  .status-bar {
    height: 24px;
    background-color: var(--status-bar-bg, #e0e0e0);
    border-top: 1px solid var(--border-color, #ccc);
    display: flex;
    align-items: center;
    padding: 0 16px;
    font-size: 12px;
    color: var(--text-light, #666);
  }
  
  .status-item {
    margin-right: 16px;
    display: flex;
    align-items: center;
  }
  
  .status-item .label {
    margin-right: 4px;
    font-weight: bold;
  }
  
  /* Panel headings */
  h2 {
    font-size: 16px;
    margin-top: 0;
    margin-bottom: 12px;
    color: var(--heading-color, #444);
  }
  
  .center-panel-header {
    margin-bottom: 12px;
  }
  
  .back-button {
    background: none;
    border: none;
    padding: 4px 8px;
    cursor: pointer;
    color: var(--primary-color, #0066cc);
    font-size: 14px;
    font-weight: 500;
    transition: color 0.2s;
    display: flex;
    align-items: center;
  }
  
  .back-button:hover {
    color: var(--primary-dark, #004c99);
    text-decoration: underline;
  }
</style> 