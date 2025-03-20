use januslens::git;
use januslens::error::JanusError;
use std::process::Command;
use std::path::PathBuf;
use std::env;
use std::fs;

fn main() -> Result<(), JanusError> {
    println!("JanusLens Automated Path Resolution Test");
    println!("=======================================");
    
    // Create a temporary test directory in the current directory
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let test_repo_dir = current_dir.join("test_repo");
    
    // Clean up any previous test repo
    if test_repo_dir.exists() {
        println!("Cleaning up previous test repository...");
        fs::remove_dir_all(&test_repo_dir).expect("Failed to remove previous test directory");
    }
    
    // Create the test directory
    println!("Creating test repository directory...");
    fs::create_dir(&test_repo_dir).expect("Failed to create test directory");
    
    // Initialize a git repository
    println!("Initializing git repository...");
    let init_output = Command::new("git")
        .args(["init", "."])
        .current_dir(&test_repo_dir)
        .output()
        .expect("Failed to initialize git repository");
    
    if !init_output.status.success() {
        let error = String::from_utf8_lossy(&init_output.stderr);
        panic!("Failed to initialize git repository: {}", error);
    }
    
    // Create a test file and commit it
    println!("Creating test file...");
    let test_file_path = test_repo_dir.join("test.txt");
    fs::write(&test_file_path, "Hello JanusLens!").expect("Failed to write test file");
    
    // Add the file to git
    println!("Adding file to git...");
    let add_output = Command::new("git")
        .args(["add", "test.txt"])
        .current_dir(&test_repo_dir)
        .output()
        .expect("Failed to add file to git");
    
    if !add_output.status.success() {
        let error = String::from_utf8_lossy(&add_output.stderr);
        panic!("Failed to add file to git: {}", error);
    }
    
    // Configure git user for the test repository
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&test_repo_dir)
        .output()
        .expect("Failed to configure git user name");
    
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&test_repo_dir)
        .output()
        .expect("Failed to configure git user email");
    
    // Commit the file
    println!("Committing file...");
    let commit_output = Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(&test_repo_dir)
        .output()
        .expect("Failed to commit file");
    
    if !commit_output.status.success() {
        let error = String::from_utf8_lossy(&commit_output.stderr);
        panic!("Failed to commit file: {}", error);
    }
    
    // Now that we have a test repository, let's test our functions
    run_tests(&test_repo_dir)?;
    
    // Clean up
    println!("\nCleaning up test repository...");
    fs::remove_dir_all(&test_repo_dir).expect("Failed to remove test directory");
    
    println!("\nAll tests completed successfully!");
    Ok(())
}

fn run_tests(repo_path: &PathBuf) -> Result<(), JanusError> {
    println!("\n--- Testing Absolute Path ---");
    let absolute_path = repo_path.to_string_lossy().to_string();
    
    // Test is_git_repository with absolute path
    println!("Testing is_git_repository with absolute path...");
    match git::is_git_repository(absolute_path.clone()) {
        Ok(true) => println!("✅ Correctly identified as a git repository"),
        Ok(false) => panic!("❌ Failed to identify as a git repository"),
        Err(e) => panic!("❌ Error checking repository: {}", e),
    }
    
    // Test open_repository with absolute path
    println!("Testing open_repository with absolute path...");
    match git::open_repository(absolute_path) {
        Ok(repo_info) => {
            println!("✅ Repository opened successfully");
            println!("  Path: {}", repo_info.path);
            println!("  Name: {}", repo_info.name);
            assert_eq!(repo_info.name, "test_repo", "Repository name should be 'test_repo'");
        },
        Err(e) => panic!("❌ Error opening repository: {}", e),
    }
    
    println!("\n--- Testing Relative Path ---");
    // Create a subdirectory to test relative paths
    let subdir_path = repo_path.join("subdir");
    fs::create_dir(&subdir_path).expect("Failed to create subdirectory");
    
    // Change to the subdirectory
    env::set_current_dir(&subdir_path).expect("Failed to change directory");
    
    // Test is_git_repository with relative path
    println!("Testing is_git_repository with relative path '..'...");
    match git::is_git_repository("..".to_string()) {
        Ok(true) => println!("✅ Correctly identified parent directory as a git repository"),
        Ok(false) => panic!("❌ Failed to identify parent directory as a git repository"),
        Err(e) => panic!("❌ Error checking repository: {}", e),
    }
    
    // Test open_repository with relative path
    println!("Testing open_repository with relative path '..'...");
    match git::open_repository("..".to_string()) {
        Ok(repo_info) => {
            println!("✅ Repository opened successfully");
            println!("  Path: {}", repo_info.path);
            println!("  Name: {}", repo_info.name);
            assert_eq!(repo_info.name, "test_repo", "Repository name should be 'test_repo'");
        },
        Err(e) => panic!("❌ Error opening repository: {}", e),
    }
    
    // Test non-existent path
    println!("\n--- Testing Non-existent Path ---");
    println!("Testing is_git_repository with non-existent path...");
    match git::is_git_repository("/path/that/does/not/exist".to_string()) {
        Ok(false) => println!("✅ Correctly identified as not a git repository"),
        Ok(true) => panic!("❌ Incorrectly identified as a git repository"),
        Err(e) => panic!("❌ Error checking repository: {}", e),
    }
    
    // List repositories and verify our test repo is there
    println!("\n--- Verifying Repository List ---");
    match git::list_repositories() {
        Ok(repos) => {
            println!("Found {} repositories:", repos.len());
            for (i, repo) in repos.iter().enumerate() {
                println!("{}. {} ({})", i+1, repo.name, repo.path);
            }
            
            let found = repos.iter().any(|r| r.name == "test_repo");
            if found {
                println!("✅ Test repository found in the list");
            } else {
                println!("❌ Test repository not found in the list");
            }
        },
        Err(e) => println!("❌ Error listing repositories: {}", e),
    }
    
    // Return to the original directory
    env::set_current_dir(repo_path).expect("Failed to change back to original directory");
    
    Ok(())
} 