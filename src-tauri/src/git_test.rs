use crate::git::{RepoInfo, is_git_repository};
use std::path::PathBuf;
use tempfile::tempdir;
use std::process::Command;
use std::fs;

#[test]
fn test_repo_info_from_path() {
    // Create a temporary directory
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    let repo_name = temp_path.file_name().unwrap().to_string_lossy().to_string();

    // Test that RepoInfo::from_path works correctly
    let repo_info = RepoInfo::from_path(temp_path.to_path_buf()).unwrap();
    assert_eq!(repo_info.name, repo_name);
    assert_eq!(repo_info.path, temp_path.to_string_lossy().to_string());
}

#[test]
fn test_path_resolution() {
    // Create a temporary directory for the test
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    
    // Create a subdirectory to test relative paths
    let subdir_path = temp_path.join("subdir");
    fs::create_dir(&subdir_path).unwrap();
    
    // Initialize a git repo in the temp directory
    Command::new("git")
        .args(&["init", "."])
        .current_dir(temp_path)
        .output()
        .expect("Failed to initialize git repository");
    
    // Test is_git_repository with absolute path
    assert!(is_git_repository(temp_path.to_string_lossy().to_string()).unwrap());
    
    // Change directory to subdirectory and test with relative path
    std::env::set_current_dir(&subdir_path).unwrap();
    assert!(is_git_repository("..".to_string()).unwrap());
    
    // Test with non-existent path
    assert!(!is_git_repository("/path/that/does/not/exist".to_string()).unwrap());
} 