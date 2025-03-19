<script lang="ts">
  import { onMount, afterUpdate } from 'svelte';
  import * as d3 from 'd3';
  
  export let commits = [];
  export let branches = [];
  
  let svg;
  let container;
  let width = 0;
  let height = 0;
  
  $: if (commits.length > 0 && svg) {
    renderGraph();
  }
  
  onMount(() => {
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
    
    // Draw commit dots
    const commitGroups = g.selectAll('.commit')
      .data(commits)
      .enter()
      .append('g')
      .attr('class', 'commit')
      .attr('transform', d => {
        const commitData = commitMap.get(d.id);
        return `translate(${xScale(commitData.lane)}, ${yScale(commitData.index)})`;
      });
    
    // Determine branch colors
    const branchColors = d3.scaleOrdinal(d3.schemeCategory10);
    const commitToBranch = new Map();
    
    branches.forEach(branch => {
      commitToBranch.set(branch.commit_id, branch.name);
    });
    
    // Draw commit circles
    commitGroups.append('circle')
      .attr('r', 6)
      .attr('fill', d => {
        const branchName = commitToBranch.get(d.id);
        return branchName ? branchColors(branchName) : '#999';
      })
      .attr('stroke', '#fff')
      .attr('stroke-width', 2);
    
    // Add commit info
    commitGroups.append('text')
      .attr('x', 15)
      .attr('y', 5)
      .text(d => `${d.short_id} - ${d.summary.substring(0, 40)}${d.summary.length > 40 ? '...' : ''}`)
      .attr('fill', '#333')
      .attr('font-size', '12px');
    
    // Add author info
    commitGroups.append('text')
      .attr('x', 15)
      .attr('y', 20)
      .text(d => `${d.author} - ${new Date(d.time * 1000).toLocaleDateString()}`)
      .attr('fill', '#666')
      .attr('font-size', '10px');
    
    // Add branch labels
    branches.forEach(branch => {
      const commit = commitMap.get(branch.commit_id);
      if (commit) {
        g.append('rect')
          .attr('x', xScale(commit.lane) + 15)
          .attr('y', yScale(commit.index) - 25)
          .attr('width', branch.name.length * 8 + 10)
          .attr('height', 20)
          .attr('rx', 10)
          .attr('ry', 10)
          .attr('fill', branchColors(branch.name));
        
        g.append('text')
          .attr('x', xScale(commit.lane) + 20)
          .attr('y', yScale(commit.index) - 12)
          .text(branch.name)
          .attr('fill', 'white')
          .attr('font-size', '10px')
          .attr('font-weight', 'bold');
      }
    });
  }
</script>

<div class="graph-container" bind:this={container}>
  <svg bind:this={svg} width="100%" height="100%"></svg>
</div>

<style>
  .graph-container {
    width: 100%;
    height: 100%;
    overflow: auto;
    position: relative;
  }
  
  svg {
    min-height: 500px;
  }
</style> 