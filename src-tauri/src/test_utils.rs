use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use std::env;

/// Creates a standard test repository with common Git structures
pub fn create_test_repository(name: &str) -> PathBuf {
    // Use a temporary directory that's guaranteed to exist
    let temp_dir = std::env::temp_dir();
    let test_dir = temp_dir.join(format!("januslens_test_{}", name));
    
    // Clean up any previous test repo
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir).expect("Failed to remove previous test directory");
    }
    
    fs::create_dir(&test_dir).expect("Failed to create test directory");
    
    // Initialize git repository
    run_git_command(&["init", "."], &test_dir);
    run_git_command(&["config", "user.name", "Test User"], &test_dir);
    run_git_command(&["config", "user.email", "test@example.com"], &test_dir);
    
    // Create initial commit
    fs::write(test_dir.join("README.md"), "# Test Repository").expect("Failed to write README");
    run_git_command(&["add", "README.md"], &test_dir);
    run_git_command(&["commit", "-m", "Initial commit"], &test_dir);
    
    test_dir
}

/// Creates a repository with multiple branches for testing
pub fn create_multi_branch_repository(name: &str) -> PathBuf {
    let test_dir = create_test_repository(name);
    
    // Create a feature branch
    run_git_command(&["checkout", "-b", "feature"], &test_dir);
    fs::write(test_dir.join("feature.txt"), "Feature content").expect("Failed to write feature file");
    run_git_command(&["add", "feature.txt"], &test_dir);
    run_git_command(&["commit", "-m", "Add feature"], &test_dir);
    
    // Create another branch with conflicts
    run_git_command(&["checkout", "main"], &test_dir);
    run_git_command(&["checkout", "-b", "conflict-branch"], &test_dir);
    fs::write(test_dir.join("README.md"), "# Modified in conflict branch").expect("Failed to modify README");
    run_git_command(&["add", "README.md"], &test_dir);
    run_git_command(&["commit", "-m", "Modify README in conflict branch"], &test_dir);
    
    // Go back to main and make conflicting changes
    run_git_command(&["checkout", "main"], &test_dir);
    fs::write(test_dir.join("README.md"), "# Modified in main branch").expect("Failed to modify README in main");
    run_git_command(&["add", "README.md"], &test_dir);
    run_git_command(&["commit", "-m", "Modify README in main branch"], &test_dir);
    
    test_dir
}

/// Run a git command in the specified directory
pub fn run_git_command(args: &[&str], dir: &PathBuf) {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect(&format!("Failed to execute git command: {:?}", args));
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        panic!("Git command failed: {}\nCommand: {:?}", error, args);
    }
}

/// Clean up a test repository
pub fn cleanup_test_repository(path: &Path) {
    if path.exists() {
        fs::remove_dir_all(path).expect("Failed to remove test directory");
    }
} 