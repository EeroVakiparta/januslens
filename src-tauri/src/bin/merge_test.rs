use januslens::git;
use januslens::error::JanusError;
use std::process::Command;
use std::path::PathBuf;
use std::env;
use std::fs;

fn main() -> Result<(), JanusError> {
    println!("JanusLens Merge Functionality Test");
    println!("=================================");
    
    // Create a test repository
    let test_dir = setup_test_repository()?;
    
    // Run tests
    run_merge_tests(&test_dir)?;
    
    // Clean up
    println!("\nCleaning up test repository...");
    fs::remove_dir_all(&test_dir).expect("Failed to remove test directory");
    
    println!("\nAll tests completed!");
    Ok(())
}

fn setup_test_repository() -> Result<PathBuf, JanusError> {
    // Create a temporary test directory
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let test_dir = current_dir.join("merge_test_repo");
    
    // Clean up any previous test repo
    if test_dir.exists() {
        println!("Cleaning up previous test repository...");
        fs::remove_dir_all(&test_dir).expect("Failed to remove previous test directory");
    }
    
    println!("Creating test repository...");
    fs::create_dir(&test_dir).expect("Failed to create test directory");
    
    // Initialize a git repository
    println!("Initializing git repository...");
    run_git_command(&["init", "."], &test_dir);
    
    // Configure git user for the test repository
    run_git_command(&["config", "user.name", "Test User"], &test_dir);
    run_git_command(&["config", "user.email", "test@example.com"], &test_dir);
    
    // Create initial file and commit
    println!("Creating initial file and commit...");
    fs::write(test_dir.join("file1.txt"), "Initial content").expect("Failed to write file");
    run_git_command(&["add", "file1.txt"], &test_dir);
    run_git_command(&["commit", "-m", "Initial commit"], &test_dir);
    
    // Create two branches for testing merges
    println!("Creating branches for merge testing...");
    
    // Create feature branch
    run_git_command(&["checkout", "-b", "feature-branch"], &test_dir);
    fs::write(test_dir.join("feature-file.txt"), "Feature content").expect("Failed to write feature file");
    run_git_command(&["add", "feature-file.txt"], &test_dir);
    run_git_command(&["commit", "-m", "Add feature file"], &test_dir);
    
    // Create conflict branch
    run_git_command(&["checkout", "main"], &test_dir);
    run_git_command(&["checkout", "-b", "conflict-branch"], &test_dir);
    fs::write(test_dir.join("file1.txt"), "Modified in conflict branch").expect("Failed to modify file");
    run_git_command(&["add", "file1.txt"], &test_dir);
    run_git_command(&["commit", "-m", "Modify file1 in conflict branch"], &test_dir);
    
    // Go back to main branch and modify the same file to create a conflict
    run_git_command(&["checkout", "main"], &test_dir);
    fs::write(test_dir.join("file1.txt"), "Modified in main branch").expect("Failed to modify file in main");
    run_git_command(&["add", "file1.txt"], &test_dir);
    run_git_command(&["commit", "-m", "Modify file1 in main branch"], &test_dir);
    
    println!("Test repository setup complete!");
    Ok(test_dir)
}

fn run_git_command(args: &[&str], dir: &PathBuf) {
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

fn run_merge_tests(repo_path: &PathBuf) -> Result<(), JanusError> {
    let repo_path_str = repo_path.to_string_lossy().to_string();
    
    // Test 1: Successful merge (no conflicts)
    println!("\n\n--- Test 1: Successful Merge ---");
    
    // Make sure we're on main branch
    println!("Checking out main branch...");
    git::checkout_branch(repo_path_str.clone(), "main".to_string())?;
    
    // Merge the feature branch (should not have conflicts)
    println!("Merging feature-branch into main...");
    match git::merge_branch(repo_path_str.clone(), "feature-branch".to_string()) {
        Ok(result) => {
            println!("Merge result: {}", result.message);
            println!("Success: {}", result.success);
            println!("Has conflicts: {}", result.has_conflicts);
            
            if result.success && !result.has_conflicts {
                println!("✅ Test 1 passed: Successfully merged without conflicts");
            } else {
                println!("❌ Test 1 failed: Expected successful merge without conflicts");
            }
            
            // Verify that feature-file.txt exists in main branch
            if repo_path.join("feature-file.txt").exists() {
                println!("✅ Verification passed: Feature file exists in main branch after merge");
            } else {
                println!("❌ Verification failed: Feature file does not exist in main branch after merge");
            }
        },
        Err(e) => {
            println!("❌ Test 1 failed with error: {}", e);
        }
    }
    
    // Test 2: Merge with conflicts
    println!("\n\n--- Test 2: Merge with Conflicts ---");
    
    // Make sure we're on main branch
    println!("Checking out main branch again...");
    git::checkout_branch(repo_path_str.clone(), "main".to_string())?;
    
    // Merge the conflict branch (should have conflicts)
    println!("Merging conflict-branch into main...");
    match git::merge_branch(repo_path_str.clone(), "conflict-branch".to_string()) {
        Ok(result) => {
            println!("Merge result: {}", result.message);
            println!("Success: {}", result.success);
            println!("Has conflicts: {}", result.has_conflicts);
            
            if !result.success && result.has_conflicts {
                println!("✅ Test 2 passed: Correctly detected conflicts");
                println!("Conflicted files:");
                for file in &result.conflicted_files {
                    println!("  - {}", file);
                }
                
                if result.conflicted_files.contains(&"file1.txt".to_string()) {
                    println!("✅ Verification passed: file1.txt correctly identified as conflicted");
                } else {
                    println!("❌ Verification failed: file1.txt not identified as conflicted");
                }
            } else {
                println!("❌ Test 2 failed: Expected merge with conflicts");
            }
        },
        Err(e) => {
            println!("❌ Test 2 failed with error: {}", e);
        }
    }
    
    // Cleanup: Abort any in-progress merge
    run_git_command(&["merge", "--abort"], repo_path);
    
    Ok(())
} 