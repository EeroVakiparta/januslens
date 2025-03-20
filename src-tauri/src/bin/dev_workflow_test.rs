use januslens::git;
use januslens::error::JanusError;
use std::path::PathBuf;
use std::fs;
use std::process::Command;
use std::env;
use std::thread::sleep;
use std::time::Duration;

// Colors for prettier console output
const GREEN: &str = "\x1B[32m";
const BLUE: &str = "\x1B[34m";
const CYAN: &str = "\x1B[36m";
const RESET: &str = "\x1B[0m";
const BOLD: &str = "\x1B[1m";

fn main() -> Result<(), JanusError> {
    println!("{}{}=== JanusLens Developer Workflow Simulation ==={}", BOLD, BLUE, RESET);
    println!("{}This test simulates a day in the life of a developer using JanusLens{}", CYAN, RESET);
    println!("{}It demonstrates a typical Git workflow with multiple branches, commits, and merges{}", CYAN, RESET);
    
    // Setup: Create a test repository
    println!("\n{}{}Step 1: Setting up test project{}", BOLD, GREEN, RESET);
    let test_dir = create_test_repo()?;
    let repo_path = test_dir.to_string_lossy().to_string();
    println!("Test repository created at: {}", repo_path);
    
    // 1. Initial repository exploration
    println!("\n{}{}Step 2: Exploring the repository{}", BOLD, GREEN, RESET);
    
    // Check if it's a valid repository
    let is_repo = git::is_git_repository(repo_path.clone())?;
    println!("Is valid Git repository: {}", is_repo);
    
    // Open the repository
    let repo_info = git::open_repository(repo_path.clone())?;
    println!("Opened repository: {} ({})", repo_info.name, repo_info.path);
    
    // List initial branches
    let branches = git::get_branches(repo_path.clone())?;
    println!("Initial branches:");
    for branch in &branches {
        println!("  {} {}", branch.name, if branch.is_head { "(HEAD)" } else { "" });
    }
    
    // 2. Feature development
    println!("\n{}{}Step 3: Starting feature development{}", BOLD, GREEN, RESET);
    
    // Create a feature branch
    let feature_branch = "feature-user-profile";
    let branch_info = git::create_branch(repo_path.clone(), feature_branch.to_string())?;
    println!("Created branch: {}", branch_info.name);
    
    // Switch to the feature branch
    git::checkout_branch(repo_path.clone(), feature_branch.to_string())?;
    println!("Checked out branch: {}", feature_branch);
    
    // Add some files for the feature
    add_feature_files(&test_dir);
    println!("Added feature files");
    
    // Check status
    let status = git::get_status(repo_path.clone())?;
    println!("Unstaged files: {}", status.unstaged.len());
    for file in &status.unstaged {
        println!("  {} ({})", file.path, file.status);
    }
    
    // Stage files
    for file in &status.unstaged {
        git::stage_file(repo_path.clone(), file.path.clone())?;
        println!("Staged: {}", file.path);
    }
    
    // Create a commit
    let commit = git::create_commit(repo_path.clone(), "Implement user profile feature".to_string())?;
    println!("Created commit: {} - {}", commit.short_id, commit.summary);
    
    // 3. Bugfix on main branch
    println!("\n{}{}Step 4: Fixing a bug on main branch{}", BOLD, GREEN, RESET);
    
    // Switch back to main
    git::checkout_branch(repo_path.clone(), "main".to_string())?;
    println!("Checked out branch: main");
    
    // Create a bugfix
    fs::write(test_dir.join("bug_fix.txt"), "Fix for critical bug")?;
    println!("Created bugfix file");
    
    // Stage and commit
    git::stage_file(repo_path.clone(), "bug_fix.txt".to_string())?;
    println!("Staged bugfix file");
    
    let bugfix_commit = git::create_commit(repo_path.clone(), "Fix critical bug".to_string())?;
    println!("Created bugfix commit: {} - {}", bugfix_commit.short_id, bugfix_commit.summary);
    
    // 4. Continue feature development
    println!("\n{}{}Step 5: Continuing feature development{}", BOLD, GREEN, RESET);
    
    // Switch back to feature branch
    git::checkout_branch(repo_path.clone(), feature_branch.to_string())?;
    println!("Checked out branch: {}", feature_branch);
    
    // Update feature
    fs::write(test_dir.join("user_profile.txt"), "Updated profile functionality\nAdded new settings")?;
    println!("Updated feature file");
    
    // Check diff
    let diff = git::get_diff(repo_path.clone(), "user_profile.txt".to_string(), false)?;
    println!("Diff for user_profile.txt:");
    println!("{}", diff);
    
    // Stage and commit
    git::stage_file(repo_path.clone(), "user_profile.txt".to_string())?;
    let update_commit = git::create_commit(repo_path.clone(), "Update user profile settings".to_string())?;
    println!("Created update commit: {} - {}", update_commit.short_id, update_commit.summary);
    
    // 5. Merge bugfix into feature
    println!("\n{}{}Step 6: Merging bugfix into feature branch{}", BOLD, GREEN, RESET);
    
    // Merge main into feature branch
    let merge_result = git::merge_branch(repo_path.clone(), "main".to_string())?;
    if merge_result.success {
        println!("Successfully merged main into {}: {}", feature_branch, merge_result.message);
    } else if merge_result.has_conflicts {
        println!("Merge has conflicts: {}", merge_result.message);
        println!("Conflicted files:");
        for file in merge_result.conflicted_files {
            println!("  {}", file);
        }
    }
    
    // 6. Complete and merge feature
    println!("\n{}{}Step 7: Completing feature and merging to main{}", BOLD, GREEN, RESET);
    
    // Add final feature file
    fs::write(test_dir.join("feature_complete.txt"), "Feature is now complete")?;
    git::stage_file(repo_path.clone(), "feature_complete.txt".to_string())?;
    let final_commit = git::create_commit(repo_path.clone(), "Complete user profile feature".to_string())?;
    println!("Created final feature commit: {} - {}", final_commit.short_id, final_commit.summary);
    
    // Switch to main
    git::checkout_branch(repo_path.clone(), "main".to_string())?;
    println!("Checked out branch: main");
    
    // Merge feature into main
    let feature_merge = git::merge_branch(repo_path.clone(), feature_branch.to_string())?;
    if feature_merge.success {
        println!("Successfully merged {} into main: {}", feature_branch, feature_merge.message);
    } else if feature_merge.has_conflicts {
        println!("Merge has conflicts: {}", feature_merge.message);
        println!("Conflicted files:");
        for file in feature_merge.conflicted_files {
            println!("  {}", file);
        }
    }
    
    // 7. View repository history
    println!("\n{}{}Step 8: Viewing repository history{}", BOLD, GREEN, RESET);
    
    // Get all commits
    let commits = git::get_commits(repo_path.clone(), None, None)?;
    println!("Repository history (latest {} commits):", commits.len());
    for (i, commit) in commits.iter().enumerate() {
        println!("  {}. {} - {} (by {})", 
                i+1, commit.short_id, commit.summary, commit.author);
    }
    
    // 8. List all branches
    println!("\n{}{}Step 9: Viewing all branches{}", BOLD, GREEN, RESET);
    let final_branches = git::get_branches(repo_path.clone())?;
    println!("All branches:");
    for branch in &final_branches {
        println!("  {} {}", branch.name, if branch.is_head { "(HEAD)" } else { "" });
    }
    
    // 9. Delete feature branch
    println!("\n{}{}Step 10: Cleaning up feature branch{}", BOLD, GREEN, RESET);
    git::delete_branch(repo_path.clone(), feature_branch.to_string())?;
    println!("Deleted branch: {}", feature_branch);
    
    // Final repository state
    let final_branch_list = git::get_branches(repo_path.clone())?;
    println!("Final branches:");
    for branch in &final_branch_list {
        println!("  {} {}", branch.name, if branch.is_head { "(HEAD)" } else { "" });
    }
    
    // Clean up
    println!("\n{}{}Test Completed Successfully!{}", BOLD, GREEN, RESET);
    println!("Cleaning up test repository...");
    sleep(Duration::from_secs(1)); // Short pause for readability
    fs::remove_dir_all(&test_dir)?;
    println!("Test repository removed");
    
    Ok(())
}

fn create_test_repo() -> Result<PathBuf, JanusError> {
    // Use a temporary directory
    let temp_dir = env::temp_dir();
    let test_dir = temp_dir.join("januslens_dev_workflow_test");
    
    // Clean up any previous test repo
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir)?;
    }
    
    fs::create_dir(&test_dir)?;
    
    // Initialize git repository
    run_git_command(&["init", "."], &test_dir);
    run_git_command(&["config", "user.name", "JanusLens Tester"], &test_dir);
    run_git_command(&["config", "user.email", "test@januslens.com"], &test_dir);
    
    // Create initial files
    fs::write(test_dir.join("README.md"), "# Test Project\n\nThis is a test project for JanusLens.")?;
    fs::write(test_dir.join("main.js"), "console.log('Hello, JanusLens!');")?;
    
    // Initial commit
    run_git_command(&["add", "."], &test_dir);
    run_git_command(&["commit", "-m", "Initial commit"], &test_dir);
    
    Ok(test_dir)
}

fn add_feature_files(test_dir: &PathBuf) {
    fs::write(test_dir.join("user_profile.txt"), "User profile functionality").expect("Failed to write feature file");
    fs::write(test_dir.join("profile_settings.js"), "function saveUserProfile() {\n  // Save profile code\n}").expect("Failed to write JS file");
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