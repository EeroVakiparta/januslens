use crate::git;
use crate::test_utils;
use std::fs;

#[test]
fn test_commit_history() {
    // Create a test repository
    let test_dir = test_utils::create_test_repository("commit_history");
    let repo_path = test_dir.to_string_lossy().to_string();
    
    // Get initial commits (should be just the Initial commit)
    let initial_commits = git::get_commits(repo_path.clone(), None, None)
        .expect("Failed to get commits");
    assert_eq!(initial_commits.len(), 1, "Should have one commit initially");
    assert_eq!(initial_commits[0].summary, "Initial commit", "First commit should be the initial commit");
    
    // Create a series of commits
    for i in 1..=5 {
        // Create a file for this commit
        let file_name = format!("file_{}.txt", i);
        fs::write(test_dir.join(&file_name), format!("Content for commit {}", i))
            .expect("Failed to write file");
        
        // Stage and commit
        git::stage_file(repo_path.clone(), file_name.clone()).expect("Failed to stage file");
        let commit_msg = format!("Commit {}", i);
        let commit = git::create_commit(repo_path.clone(), commit_msg.clone())
            .expect("Failed to create commit");
        
        // Verify commit details
        assert_eq!(commit.summary, commit_msg, "Commit message should match");
        assert_eq!(commit.author, "Test User", "Author should be Test User");
        assert!(!commit.id.is_empty(), "Commit ID should not be empty");
    }
    
    // Get all commits
    let all_commits = git::get_commits(repo_path.clone(), None, None)
        .expect("Failed to get commits");
    assert_eq!(all_commits.len(), 6, "Should have 6 commits total (initial + 5 new)");
    
    // Commits should be in reverse chronological order (newest first)
    assert_eq!(all_commits[0].summary, "Commit 5", "Latest commit should be first");
    assert_eq!(all_commits[5].summary, "Initial commit", "Initial commit should be last");
    
    // Test limit parameter
    let limited_commits = git::get_commits(repo_path.clone(), None, Some(3))
        .expect("Failed to get limited commits");
    assert_eq!(limited_commits.len(), 3, "Should have 3 commits with limit");
    assert_eq!(limited_commits[0].summary, "Commit 5", "First limited commit should be latest");
    assert_eq!(limited_commits[2].summary, "Commit 3", "Last limited commit should be Commit 3");
    
    // Create a branch at an earlier commit
    let branch_point_id = all_commits[3].id.clone(); // Commit 2
    let branch_name = "history-test-branch";
    
    // Checkout the specific commit
    test_utils::run_git_command(&["checkout", &branch_point_id], &test_dir);
    
    // Create the branch at this point
    git::create_branch(repo_path.clone(), branch_name.to_string())
        .expect("Failed to create branch");
    
    // Checkout the branch
    git::checkout_branch(repo_path.clone(), branch_name.to_string())
        .expect("Failed to checkout branch");
    
    // Make a new commit on this branch
    let branch_file = "branch_specific.txt";
    fs::write(test_dir.join(branch_file), "Branch specific content")
        .expect("Failed to write file");
    git::stage_file(repo_path.clone(), branch_file.to_string()).expect("Failed to stage file");
    git::create_commit(repo_path.clone(), "Branch specific commit".to_string())
        .expect("Failed to create commit");
    
    // Get commits for this specific branch
    let branch_commits = git::get_commits(repo_path.clone(), Some(branch_name.to_string()), None)
        .expect("Failed to get branch commits");
    
    // Should have 4 commits (Initial + Commit 1 + Commit 2 + Branch specific)
    assert_eq!(branch_commits.len(), 4, "Branch should have 4 commits");
    assert_eq!(branch_commits[0].summary, "Branch specific commit", "Latest branch commit should be first");
    
    // Main branch should still have all 6 commits
    let main_commits = git::get_commits(repo_path.clone(), Some("main".to_string()), None)
        .expect("Failed to get main branch commits");
    assert_eq!(main_commits.len(), 6, "Main should still have 6 commits");
    
    // Clean up
    test_utils::cleanup_test_repository(&test_dir);
} 