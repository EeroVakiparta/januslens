# Changes in 0.2.0 (2025-03-21)

Initial release

# Changelog

All notable changes to JanusLens will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project setup with Tauri and Svelte
- Basic UI layout and components (Repository Explorer, Branch Manager, Git Graph, Diff Viewer, Commit Panel)
- Git repository operations through git2-rs
- Documentation framework (README, Developer Guide, Architecture Overview, Setup Guide)
- Automated test script for repository path handling
- Branch merging functionality with conflict detection
- Git Features Roadmap to track implementation progress

### Fixed
- Icon configuration issues in Tauri setup
- Repository path handling to properly support relative paths
- Repository name extraction from paths
- Storage of absolute paths in recent repositories list

### Improved
- Code organization with helper method for repository info creation
- Test coverage for git repository path handling
- Test coverage for git merge operations

## [0.1.0] - TBD

Initial release of JanusLens with core functionality:

### Added
- Repository visualization with D3.js
- Git operations (commit, branch, status)
- Diff viewer with syntax highlighting
- Branch management interface 