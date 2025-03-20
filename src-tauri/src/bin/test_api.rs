use std::env;
use std::process;
use januslens::git;
use januslens::error::JanusError;

fn main() {
    env_logger::init();
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "list-repos" => handle_command(list_repos),
        "open-repo" => {
            if args.len() < 3 {
                println!("Error: Missing repository path argument");
                print_usage();
                process::exit(1);
            }
            let path = &args[2];
            handle_command(|| open_repo(path))
        },
        "is-git-repo" => {
            if args.len() < 3 {
                println!("Error: Missing repository path argument");
                print_usage();
                process::exit(1);
            }
            let path = &args[2];
            handle_command(|| is_git_repo(path))
        },
        "get-common-locations" => handle_command(get_common_locations),
        _ => {
            println!("Unknown command: {}", command);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("JanusLens API Test Utility");
    println!("Usage:");
    println!("  test_api list-repos");
    println!("  test_api open-repo <path>");
    println!("  test_api is-git-repo <path>");
    println!("  test_api get-common-locations");
}

fn handle_command<F, T>(command: F) 
where 
    F: FnOnce() -> Result<T, JanusError>,
    T: std::fmt::Debug,
{
    match command() {
        Ok(result) => {
            println!("Success: {:#?}", result);
        },
        Err(err) => {
            println!("Error: {}", err);
            process::exit(1);
        }
    }
}

fn list_repos() -> Result<Vec<git::RepoInfo>, JanusError> {
    git::list_repositories()
}

fn open_repo(path: &str) -> Result<git::RepoInfo, JanusError> {
    git::open_repository(path.to_string())
}

fn is_git_repo(path: &str) -> Result<bool, JanusError> {
    git::is_git_repository(path.to_string())
}

fn get_common_locations() -> Result<Vec<String>, JanusError> {
    git::get_common_repo_locations()
} 