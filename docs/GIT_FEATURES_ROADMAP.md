# Git Features Implementation Roadmap

This document outlines the plan for implementing Git features in the JanusLens backend, tracking progress, and prioritizing future work.

## Repository Management

- [x] **Repository discovery and listing**
  - [x] List recent repositories
  - [x] Open repository and add to recent list
  - [x] Handle relative paths correctly
  - [x] Extract repository name properly

- [x] **Repository validation**
  - [x] Check if path is a valid git repository
  - [x] Verify repository accessibility
  - [x] Get common repository locations

## Branch Management

- [x] **Branch information**
  - [x] List branches in repository
  - [x] Identify current branch (HEAD)
  - [x] Get upstream branch information

- [x] **Branch operations**
  - [x] Create new branch
  - [x] Delete branch
  - [x] Checkout branch

## Commit Management

- [x] **Commit history**
  - [x] Get commits for branch
  - [x] Limit number of commits returned
  - [x] Extract commit details (author, message, time)

- [x] **Commit creation**
  - [x] Create new commit with message
  - [x] Return new commit information

## File Management

- [x] **File listing**
  - [x] List files in repository
  - [x] Group by directories
  - [x] Sort files and directories

- [x] **File status**
  - [x] Get file status (staged, unstaged)
  - [x] Identify new, modified, and deleted files

## Diff and Change Management

- [x] **Diff operations**
  - [x] Get diff for file changes
  - [x] Support both staged and unstaged diffs

- [x] **Staging operations**
  - [x] Stage file
  - [x] Unstage file

## Advanced Features (Upcoming)

- [x] **Merge operations**
  - [x] Merge branches
  - [x] Handle merge conflicts
  - [x] Get merge status

- [ ] **Stash operations**
  - [ ] Create stash
  - [ ] List stashes
  - [ ] Apply stash
  - [ ] Drop stash

- [ ] **Remote operations**
  - [ ] List remotes
  - [ ] Add remote
  - [ ] Push to remote
  - [ ] Pull from remote
  - [ ] Fetch from remote

- [ ] **Tag management**
  - [ ] List tags
  - [ ] Create tag
  - [ ] Delete tag

- [ ] **Submodule handling**
  - [ ] List submodules
  - [ ] Update submodules
  - [ ] Add submodule

## Performance Optimizations

- [ ] **Large repository support**
  - [ ] Implement pagination for commit history
  - [ ] Optimize file listing for large repositories
  - [ ] Cache repository metadata

- [ ] **Background operations**
  - [ ] Run long operations in background threads
  - [ ] Provide progress updates for long-running operations
  - [ ] Allow cancellation of operations

## Testing and Reliability

- [x] **Basic testing**
  - [x] Test repository path handling
  - [x] Test repository operations
  - [x] Test merge operations

- [ ] **Advanced testing**
  - [x] Setup testing utilities for standardized test repositories
  - [x] Create testing plan document
  - [ ] Integration tests for Git operations
  - [ ] Test edge cases (large files, special characters)
  - [ ] Test error recovery
  - [ ] Achieve target code coverage (80%+)

## Notes on implementation approach

Most Git operations are currently implemented using direct git2-rs library calls, with some operations using git CLI commands for simplicity. As the project matures, we will migrate all operations to use the git2-rs library directly for better performance and control.

## Next feature to implement

The next feature to implement is **Stash operations**, starting with creating and listing stashes. 

Additionally, we will continue improving test coverage according to the testing plan. 