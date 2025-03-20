<script>
  import { onMount, afterUpdate, createEventDispatcher } from 'svelte';
  import * as d3 from 'd3';
  import { Logger } from '../services/logger';
  
  export let commits = [];
  export let branches = [];
  
  const dispatch = createEventDispatcher();
  
  let svg;
  let container;
  let width = 0;
  let height = 0;
  let selectedCommit = null;
  
  $: if (commits.length > 0 && svg) {
    renderGraph();
  }
  
  onMount(() => {
    Logger.info('GitGraph', 'Component mounted', { commitCount: commits.length });
    
    const resizeObserver = new ResizeObserver(entries => {
      for (let entry of entries) {
        width = entry.contentRect.width;
        height = entry.contentRect.height;
        if (commits.length > 0) {
          renderGraph();
        }
      }
    });
    
    if (container) {
      resizeObserver.observe(container);
    }
    
    return () => {
      resizeObserver.disconnect();
    };
  });
  
  function renderGraph() {
    if (!svg || !width || !height) return;
    
    Logger.debug('GitGraph', 'Rendering graph', { width, height, commits: commits.length });
    
    // Clear previous graph
    d3.select(svg).selectAll('*').remove();
    
    const svgEl = d3.select(svg);
    
    // Set up basic dimensions
    const margin = { top: 20, right: 20, bottom: 20, left: 20 };
    const graphWidth = width - margin.left - margin.right;
    const graphHeight = height - margin.top - margin.bottom;
    
    // Create a group element for the graph
    const g = svgEl.append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`);
    
    // Process commits to create a graph structure
    const commitMap = new Map();
    const lanes = [];
    
    commits.forEach((commit, index) => {
      commitMap.set(commit.id, { ...commit, index });
    });
    
    // Assign lanes to each commit
    commits.forEach(commit => {
      // Find an available lane or create a new one
      let laneIndex = -1;
      
      if (commit.parent_ids.length > 0) {
        // Try to find a lane that has the parent
        for (let i = 0; i < lanes.length; i++) {
          const lastCommitId = lanes[i];
          if (commit.parent_ids.includes(lastCommitId)) {
            laneIndex = i;
            break;
          }
        }
      }
      
      if (laneIndex === -1) {
        // Create a new lane
        laneIndex = lanes.length;
        lanes.push(null);
      }
      
      // Update the commit with lane information
      commitMap.get(commit.id).lane = laneIndex;
      
      // Update lane with this commit
      lanes[laneIndex] = commit.id;
    });
    
    // Create scales
    const xScale = d3.scaleLinear()
      .domain([0, Math.max(lanes.length - 1, 0)])
      .range([50, graphWidth - 50]);
    
    const yScale = d3.scaleLinear()
      .domain([0, commits.length - 1])
      .range([50, graphHeight - 50]);
    
    // Draw lanes
    for (let i = 0; i < lanes.length; i++) {
      g.append('line')
        .attr('x1', xScale(i))
        .attr('y1', yScale(0))
        .attr('x2', xScale(i))
        .attr('y2', yScale(commits.length - 1))
        .attr('stroke', '#ddd')
        .attr('stroke-width', 2)
        .attr('stroke-dasharray', '4');
    }
    
    // Draw connections between commits
    commits.forEach(commit => {
      const commitData = commitMap.get(commit.id);
      const x1 = xScale(commitData.lane);
      const y1 = yScale(commitData.index);
      
      commit.parent_ids.forEach(parentId => {
        const parent = commitMap.get(parentId);
        if (parent) {
          const x2 = xScale(parent.lane);
          const y2 = yScale(parent.index);
          
          g.append('path')
            .attr('d', `M ${x1} ${y1} C ${x1} ${(y1 + y2) / 2}, ${x2} ${(y1 + y2) / 2}, ${x2} ${y2}`)
            .attr('stroke', '#999')
            .attr('stroke-width', 2)
            .attr('fill', 'none');
        }
      });
    });
    
    // Determine branch colors
    const branchColors = d3.scaleOrdinal(d3.schemeCategory10);
    const commitToBranch = new Map();
    
    branches.forEach(branch => {
      commitToBranch.set(branch.commit_id, branch.name);
    });
    
    // Draw commit dots and make them interactive
    const commitGroups = g.selectAll('.commit')
      .data(commits)
      .enter()
      .append('g')
      .attr('class', 'commit')
      .attr('transform', d => {
        const commitData = commitMap.get(d.id);
        return `translate(${xScale(commitData.lane)}, ${yScale(commitData.index)})`;
      })
      .style('cursor', 'pointer')
      .on('click', (event, d) => {
        event.stopPropagation();
        selectedCommit = d;
        Logger.info('GitGraph', 'Commit selected', { id: d.id, summary: d.summary });
        dispatch('commitSelected', d);
        renderGraph(); // Re-render to highlight selected commit
      });
    
    // Draw commit circles
    commitGroups.append('circle')
      .attr('r', d => selectedCommit && d.id === selectedCommit.id ? 8 : 6)
      .attr('fill', d => {
        const branchName = commitToBranch.get(d.id);
        return branchName ? branchColors(branchName) : '#999';
      })
      .attr('stroke', d => selectedCommit && d.id === selectedCommit.id ? '#ff9900' : '#fff')
      .attr('stroke-width', d => selectedCommit && d.id === selectedCommit.id ? 3 : 2);
    
    // Add commit info
    commitGroups.append('text')
      .attr('x', 15)
      .attr('y', 5)
      .text(d => `${d.short_id} - ${d.summary.substring(0, 40)}${d.summary.length > 40 ? '...' : ''}`)
      .attr('fill', d => selectedCommit && d.id === selectedCommit.id ? '#000' : '#333')
      .attr('font-weight', d => selectedCommit && d.id === selectedCommit.id ? 'bold' : 'normal')
      .attr('font-size', '12px');
    
    // Add author info
    commitGroups.append('text')
      .attr('x', 15)
      .attr('y', 20)
      .text(d => `${d.author} - ${new Date(d.time * 1000).toLocaleDateString()}`)
      .attr('fill', '#666')
      .attr('font-size', '10px');
    
    // Add branch labels with interactivity
    branches.forEach(branch => {
      const commit = commitMap.get(branch.commit_id);
      if (commit) {
        // Branch label group
        const branchGroup = g.append('g')
          .attr('class', 'branch-label')
          .style('cursor', 'pointer')
          .on('click', (event) => {
            event.stopPropagation();
            Logger.info('GitGraph', 'Branch selected', { name: branch.name, commitId: branch.commit_id });
            dispatch('branchSelected', branch);
          });
          
        // Branch background
        branchGroup.append('rect')
          .attr('x', xScale(commit.lane) + 15)
          .attr('y', yScale(commit.index) - 25)
          .attr('width', branch.name.length * 8 + 10)
          .attr('height', 20)
          .attr('rx', 10)
          .attr('ry', 10)
          .attr('fill', branchColors(branch.name))
          .attr('opacity', branch.is_head ? 1 : 0.7);
        
        // Branch text
        branchGroup.append('text')
          .attr('x', xScale(commit.lane) + 20)
          .attr('y', yScale(commit.index) - 12)
          .text(branch.name)
          .attr('fill', 'white')
          .attr('font-size', '10px')
          .attr('font-weight', branch.is_head ? 'bold' : 'normal');
          
        // Add HEAD indicator if this is the current branch
        if (branch.is_head) {
          branchGroup.append('text')
            .attr('x', xScale(commit.lane) + 15 + branch.name.length * 8 + 15)
            .attr('y', yScale(commit.index) - 12)
            .text('HEAD')
            .attr('fill', '#333')
            .attr('font-size', '10px')
            .attr('font-weight', 'bold');
        }
      }
    });
    
    // Add clear selection handler to the svg
    svgEl.on('click', () => {
      if (selectedCommit) {
        selectedCommit = null;
        Logger.debug('GitGraph', 'Selection cleared');
        renderGraph();
      }
    });
  }
  
  // Function to handle commit details request
  function showCommitDetails(commit) {
    Logger.info('GitGraph', 'Showing commit details', { id: commit.id });
    // In a real implementation, this would show a detailed view
    dispatch('showCommitDetails', commit);
  }
</script>

<div class="graph-container" bind:this={container}>
  <div class="toolbar">
    <h2>Git Graph</h2>
    <div class="spacer"></div>
    <button class="refresh-button" on:click={() => renderGraph()}>
      Refresh
    </button>
  </div>

  <svg bind:this={svg} width="100%" height="100%"></svg>
  
  {#if selectedCommit}
    <div class="commit-details">
      <h3>Commit Details</h3>
      <div class="detail-row">
        <span class="label">ID:</span>
        <span class="value">{selectedCommit.short_id}</span>
      </div>
      <div class="detail-row">
        <span class="label">Author:</span>
        <span class="value">{selectedCommit.author} &lt;{selectedCommit.author_email}&gt;</span>
      </div>
      <div class="detail-row">
        <span class="label">Date:</span>
        <span class="value">{new Date(selectedCommit.time * 1000).toLocaleString()}</span>
      </div>
      <div class="detail-row">
        <span class="label">Message:</span>
        <span class="value message">{selectedCommit.message}</span>
      </div>
      <div class="detail-actions">
        <button on:click={() => showCommitDetails(selectedCommit)}>View Changes</button>
      </div>
    </div>
  {/if}
</div>

<style>
  .graph-container {
    width: 100%;
    height: 100%;
    overflow: auto;
    position: relative;
    display: flex;
    flex-direction: column;
  }
  
  .toolbar {
    display: flex;
    align-items: center;
    padding: 0 15px 10px 15px;
    border-bottom: 1px solid var(--border-color, #ddd);
    margin-bottom: 10px;
  }
  
  .toolbar h2 {
    margin: 0;
    font-size: 18px;
  }
  
  .spacer {
    flex-grow: 1;
  }
  
  .refresh-button {
    background-color: var(--primary-color, #0066cc);
    color: white;
    border: none;
    border-radius: 4px;
    padding: 5px 10px;
    font-size: 14px;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  .refresh-button:hover {
    background-color: var(--primary-dark, #004c99);
  }
  
  svg {
    flex-grow: 1;
    min-height: 400px;
  }
  
  .commit-details {
    position: absolute;
    bottom: 20px;
    right: 20px;
    width: 300px;
    background-color: white;
    border: 1px solid var(--border-color, #ccc);
    border-radius: 5px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    padding: 15px;
  }
  
  .commit-details h3 {
    margin-top: 0;
    margin-bottom: 10px;
    font-size: 16px;
    color: var(--heading-color, #444);
  }
  
  .detail-row {
    margin-bottom: 5px;
    display: flex;
  }
  
  .detail-row .label {
    font-weight: bold;
    width: 70px;
    flex-shrink: 0;
  }
  
  .detail-row .value {
    flex-grow: 1;
  }
  
  .detail-row .message {
    white-space: pre-wrap;
  }
  
  .detail-actions {
    margin-top: 10px;
    display: flex;
    justify-content: flex-end;
  }
  
  .detail-actions button {
    background-color: var(--primary-color, #0066cc);
    color: white;
    border: none;
    border-radius: 3px;
    padding: 5px 10px;
    font-size: 12px;
    cursor: pointer;
  }
  
  .detail-actions button:hover {
    background-color: var(--primary-dark, #004c99);
  }
</style> 