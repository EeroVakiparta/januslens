use crate::git;
use crate::test_utils;
use std::fs;

#[test]
fn test_branch_operations() {
    // Create a test repository
    let test_dir = test_utils::create_test_repository("branch_operations");
    let repo_path = test_dir.to_string_lossy().to_string();
    
    // Get initial branches (should just be main)
    let initial_branches = git::get_branches(repo_path.clone()).expect("Failed to get branches");
    assert_eq!(initial_branches.len(), 1, "Should have one branch initially");
    assert_eq!(initial_branches[0].name, "main", "Initial branch should be main");
    assert!(initial_branches[0].is_head, "Main branch should be HEAD");
    
    // Create a new branch
    let new_branch_name = "test-branch";
    let branch_info = git::create_branch(repo_path.clone(), new_branch_name.to_string())
        .expect("Failed to create branch");
    
    // Verify branch creation
    assert_eq!(branch_info.name, new_branch_name, "Branch name should match");
    assert!(!branch_info.is_head, "New branch should not be HEAD yet");
    
    // List branches again to verify the new branch appears
    let branches_after_create = git::get_branches(repo_path.clone()).expect("Failed to get branches");
    assert_eq!(branches_after_create.len(), 2, "Should have two branches now");
    assert!(branches_after_create.iter().any(|b| b.name == new_branch_name), 
            "New branch should be in the list");
    
    // Checkout the new branch
    git::checkout_branch(repo_path.clone(), new_branch_name.to_string())
        .expect("Failed to checkout branch");
    
    // Verify the new branch is now HEAD
    let branches_after_checkout = git::get_branches(repo_path.clone()).expect("Failed to get branches");
    let new_branch = branches_after_checkout.iter()
        .find(|b| b.name == new_branch_name)
        .expect("New branch should exist");
    assert!(new_branch.is_head, "New branch should now be HEAD");
    
    // Create a file on this branch
    let test_file = "branch_test_file.txt";
    fs::write(test_dir.join(test_file), "Branch test content").expect("Failed to write file");
    git::stage_file(repo_path.clone(), test_file.to_string()).expect("Failed to stage file");
    git::create_commit(repo_path.clone(), "Commit on test branch".to_string())
        .expect("Failed to create commit");
    
    // Checkout back to main
    git::checkout_branch(repo_path.clone(), "main".to_string())
        .expect("Failed to checkout main");
    
    // Verify main is now HEAD
    let branches_final = git::get_branches(repo_path.clone()).expect("Failed to get branches");
    let main_branch = branches_final.iter()
        .find(|b| b.name == "main")
        .expect("Main branch should exist");
    assert!(main_branch.is_head, "Main branch should now be HEAD");
    
    // The file created on test-branch should not exist on main
    assert!(!test_dir.join(test_file).exists(), 
            "File from test branch should not exist on main");
    
    // Delete the test branch
    git::delete_branch(repo_path.clone(), new_branch_name.to_string())
        .expect("Failed to delete branch");
    
    // Verify branch was deleted
    let branches_after_delete = git::get_branches(repo_path.clone()).expect("Failed to get branches");
    assert_eq!(branches_after_delete.len(), 1, "Should have one branch after deletion");
    assert_eq!(branches_after_delete[0].name, "main", "Only branch should be main");
    
    // Clean up
    test_utils::cleanup_test_repository(&test_dir);
} 