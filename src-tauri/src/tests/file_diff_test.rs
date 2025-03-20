use crate::git;
use crate::test_utils;
use std::fs;

#[test]
fn test_file_listing_and_diff() {
    // Create a test repository
    let test_dir = test_utils::create_test_repository("file_diff");
    let repo_path = test_dir.to_string_lossy().to_string();
    
    // Get initial file listing (should just be README.md)
    let initial_files = git::list_files(repo_path.clone(), None)
        .expect("Failed to list files");
    assert!(initial_files.len() >= 1, "Should have at least one file initially");
    
    // There should be a README.md file in the initial files
    let readme = initial_files.iter()
        .find(|f| f.name == "README.md")
        .expect("README.md should exist");
    assert_eq!(readme.type_, "file", "README.md should be of type 'file'");
    
    // Create a directory with some files
    let dir_name = "test_dir";
    let dir_path = test_dir.join(dir_name);
    fs::create_dir(&dir_path).expect("Failed to create directory");
    
    // Create files in the directory
    fs::write(dir_path.join("file1.txt"), "File 1 content").expect("Failed to write file1");
    fs::write(dir_path.join("file2.txt"), "File 2 content").expect("Failed to write file2");
    
    // List files again to see the directory
    let files_with_dir = git::list_files(repo_path.clone(), None)
        .expect("Failed to list files");
    let initial_count = initial_files.len();
    assert_eq!(files_with_dir.len(), initial_count + 1, 
        "Should have one more entry (the directory) than before");
    
    // One of the entries should be a directory
    let dir_entry = files_with_dir.iter()
        .find(|e| e.name == dir_name)
        .expect("Directory should be in the listing");
    assert_eq!(dir_entry.type_, "directory", "Entry should be a directory");
    
    // List files in the directory
    let dir_files = git::list_files(repo_path.clone(), Some(dir_name.to_string()))
        .expect("Failed to list files in directory");
    assert_eq!(dir_files.len(), 2, "Directory should contain 2 files");
    assert!(dir_files.iter().any(|f| f.name == "file1.txt"), "file1.txt should be in directory");
    assert!(dir_files.iter().any(|f| f.name == "file2.txt"), "file2.txt should be in directory");
    
    // Test file diff functionality by modifying a file
    let test_file = "diff_test.txt";
    let initial_content = "Initial content for diff test";
    let modified_content = "Modified content for diff test";
    
    // Create and commit the file
    fs::write(test_dir.join(test_file), initial_content).expect("Failed to write test file");
    git::stage_file(repo_path.clone(), test_file.to_string()).expect("Failed to stage file");
    git::create_commit(repo_path.clone(), "Add file for diff testing".to_string())
        .expect("Failed to create commit");
    
    // Modify the file
    fs::write(test_dir.join(test_file), modified_content).expect("Failed to modify test file");
    
    // Get diff for unstaged changes
    let unstaged_diff = git::get_diff(repo_path.clone(), test_file.to_string(), false)
        .expect("Failed to get unstaged diff");
    
    // Diff should contain both the old and new content
    assert!(unstaged_diff.contains("-Initial content"), "Diff should show removed content");
    assert!(unstaged_diff.contains("+Modified content"), "Diff should show added content");
    
    // Stage the changes
    git::stage_file(repo_path.clone(), test_file.to_string()).expect("Failed to stage file");
    
    // Get diff for staged changes
    let staged_diff = git::get_diff(repo_path.clone(), test_file.to_string(), true)
        .expect("Failed to get staged diff");
    
    // Staged diff should contain the same information
    assert!(staged_diff.contains("-Initial content"), "Staged diff should show removed content");
    assert!(staged_diff.contains("+Modified content"), "Staged diff should show added content");
    
    // Clean up
    test_utils::cleanup_test_repository(&test_dir);
} 