use januslens::git;
use januslens::error::JanusError;
use std::io::{self, Write};

fn main() -> Result<(), JanusError> {
    println!("JanusLens Path Resolution Test");
    println!("==============================");
    
    // Test path handling for a repository
    print!("Enter a repository path to test (absolute or relative): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let path = input.trim().to_string();
    
    // First check if it's a valid git repository
    println!("\nChecking if path is a git repository...");
    
    match git::is_git_repository(path.clone()) {
        Ok(true) => {
            println!("✅ Path is a valid git repository!");
            
            // Try to open it
            println!("\nOpening repository...");
            match git::open_repository(path) {
                Ok(repo_info) => {
                    println!("Repository opened successfully!");
                    println!("Repository details:");
                    println!("  Path: {}", repo_info.path);
                    println!("  Name: {}", repo_info.name);
                    println!("  Last accessed: {}", repo_info.last_accessed);
                    
                    // List repositories
                    println!("\nListing all recent repositories:");
                    match git::list_repositories() {
                        Ok(repos) => {
                            if repos.is_empty() {
                                println!("No recent repositories.");
                            } else {
                                for (i, repo) in repos.iter().enumerate() {
                                    println!("{}. {} ({})", i+1, repo.name, repo.path);
                                }
                            }
                        },
                        Err(e) => println!("Error listing repositories: {}", e),
                    }
                },
                Err(e) => println!("❌ Error opening repository: {}", e),
            }
        },
        Ok(false) => {
            println!("❌ Path is not a valid git repository.");
        },
        Err(e) => {
            println!("❌ Error checking repository: {}", e);
        }
    }
    
    println!("\nTest completed.");
    Ok(())
} 