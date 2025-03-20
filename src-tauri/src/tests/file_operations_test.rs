use crate::git;
use crate::test_utils;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_stage_and_unstage_file() {
    // Create a test repository
    let test_dir = test_utils::create_test_repository("file_operations");
    let repo_path = test_dir.to_string_lossy().to_string();
    
    // Create a new file
    let test_file = "test_file.txt";
    fs::write(test_dir.join(test_file), "Test content").expect("Failed to write test file");
    
    // Initially the file should be unstaged
    let initial_status = git::get_status(repo_path.clone()).expect("Failed to get status");
    assert_eq!(initial_status.staged.len(), 0, "No files should be staged initially");
    assert_eq!(initial_status.unstaged.len(), 1, "One file should be unstaged");
    assert_eq!(initial_status.unstaged[0].path, test_file);
    
    // Stage the file
    git::stage_file(repo_path.clone(), test_file.to_string()).expect("Failed to stage file");
    
    // Verify the file is now staged
    let after_stage_status = git::get_status(repo_path.clone()).expect("Failed to get status");
    assert_eq!(after_stage_status.staged.len(), 1, "One file should be staged");
    assert_eq!(after_stage_status.staged[0].path, test_file);
    assert_eq!(after_stage_status.unstaged.len(), 0, "No files should be unstaged");
    
    // Unstage the file
    git::unstage_file(repo_path.clone(), test_file.to_string()).expect("Failed to unstage file");
    
    // Verify the file is unstaged again
    let after_unstage_status = git::get_status(repo_path.clone()).expect("Failed to get status");
    assert_eq!(after_unstage_status.staged.len(), 0, "No files should be staged");
    assert_eq!(after_unstage_status.unstaged.len(), 1, "One file should be unstaged");
    assert_eq!(after_unstage_status.unstaged[0].path, test_file);
    
    // Clean up
    test_utils::cleanup_test_repository(&test_dir);
}

#[test]
fn test_commit_creation() {
    // Create a test repository
    let test_dir = test_utils::create_test_repository("commit_operations");
    let repo_path = test_dir.to_string_lossy().to_string();
    
    // Create and stage a new file
    let test_file = "commit_test.txt";
    fs::write(test_dir.join(test_file), "Test content for commit").expect("Failed to write test file");
    git::stage_file(repo_path.clone(), test_file.to_string()).expect("Failed to stage file");
    
    // Create a commit
    let commit_message = "Test commit message";
    let commit = git::create_commit(repo_path.clone(), commit_message.to_string())
        .expect("Failed to create commit");
    
    // Verify commit details
    assert_eq!(commit.summary, commit_message);
    assert!(!commit.id.is_empty(), "Commit ID should not be empty");
    
    // Clean up
    test_utils::cleanup_test_repository(&test_dir);
} 