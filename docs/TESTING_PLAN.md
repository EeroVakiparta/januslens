# JanusLens Testing Plan

This document outlines the testing strategy for JanusLens, focusing on achieving comprehensive test coverage for the Git features.

## Testing Goals

1. **Reliability**: Ensure all Git operations work correctly across different environments
2. **Maintainability**: Make it easy to add tests for new features
3. **Comprehensive Coverage**: Aim for at least 80% code coverage for core Git functionality
4. **Regression Prevention**: Prevent regressions when implementing new features

## Test Categories

### 1. Unit Tests

Unit tests focus on testing individual functions in isolation.

- **Location**: Within the same file as the code being tested, in a `tests` module
- **Coverage Target**: 90%+ for utility functions and data structures
- **Example**: Testing `RepoInfo::from_path`

### 2. Integration Tests

Integration tests verify interactions between components.

- **Location**: In dedicated test files in the `src/tests` directory
- **Coverage Target**: 80%+ for core Git operations
- **Example**: Testing repository operations that involve multiple functions

### 3. Mock Tests

Tests that use mock repositories created specifically for testing.

- **Location**: In `src/tests` directory
- **Utility**: Use `test_utils.rs` to create standardized test repositories
- **Coverage Target**: 70%+ for repository operations

### 4. E2E Tests

End-to-end tests that verify complete workflows.

- **Location**: In the `src/bin` directory as executable tests
- **Coverage Target**: Cover all main user scenarios

## Test Coverage Best Practices

### Repository for Testing

- Use temporary directories for test repositories
- Always clean up after tests
- Use a standard set of test repositories for consistency

### Error Cases

- Test both happy paths and error paths
- Ensure proper error messages are returned
- Test boundary conditions

### Test Isolation

- Tests should not depend on each other
- Each test should set up its own environment
- Tests should clean up after themselves

## Test Implementation Roadmap

### Phase 1: Core Repository Operations (Current)

- [x] Repository path handling
- [x] Repository info extraction
- [x] File staging/unstaging
- [x] Merge operations

### Phase 2: Branch Operations

- [ ] Branch creation/deletion tests
- [ ] Branch checkout tests
- [ ] Branch listing tests

### Phase 3: Commit Operations

- [ ] Commit creation tests
- [ ] Commit history tests
- [ ] Commit detail tests

### Phase 4: Advanced Operations

- [ ] Stash operations tests
- [ ] Remote operations tests
- [ ] Tag management tests

## Running Tests

### Regular Testing

```bash
cargo test
```

### Coverage Testing

```bash
cargo tarpaulin --skip-clean
```

### Test Reporting

- Generate coverage reports after significant changes
- Update this document with coverage metrics monthly
- Prioritize testing for low-coverage areas

## Current Coverage Status

As of the initial implementation:
- Overall coverage: 9.82%
- File operations coverage: Basic testing implemented
- Git merge operations: Basic testing implemented

## Next Steps

1. Implement more targeted unit tests for individual functions
2. Create integration tests for common workflows
3. Add tests for error conditions and edge cases
4. Improve documentation of testing utilities 