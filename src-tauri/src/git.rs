use crate::error::JanusError;
use git2::{BranchType, Commit, Repository, Oid, StatusOptions, StatusShow};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use log::{error, info};
use std::fs;

/// Represents a Git repository with metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoInfo {
    pub path: String,
    pub name: String,
    pub last_accessed: u64,
}

impl RepoInfo {
    /// Creates a new RepoInfo from a PathBuf
    pub fn from_path(path: PathBuf) -> Result<Self, JanusError> {
        if !path.exists() {
            return Err(JanusError::GitError(format!("Path does not exist: {:?}", path)));
        }

        // Extract repository name from the path
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Ok(Self {
            path: path.to_string_lossy().to_string(),
            name,
            last_accessed: now,
        })
    }
}

/// Storage for recent repositories
#[derive(Debug, Serialize, Deserialize)]
struct RecentRepos {
    repositories: Vec<RepoInfo>,
}

impl RecentRepos {
    fn new() -> Self {
        Self {
            repositories: Vec::new(),
        }
    }

    fn load() -> Result<Self, JanusError> {
        let config_dir = get_config_dir()?;
        let recent_repos_path = config_dir.join("recent_repos.json");

        if !recent_repos_path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(recent_repos_path)
            .map_err(|e| JanusError::IoError(e.to_string()))?;
        
        serde_json::from_str(&content)
            .map_err(|e| JanusError::ConfigError(format!("Failed to parse recent repositories: {}", e)))
    }

    fn save(&self) -> Result<(), JanusError> {
        let config_dir = get_config_dir()?;
        
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .map_err(|e| JanusError::IoError(format!("Failed to create config directory: {}", e)))?;
        }
        
        let recent_repos_path = config_dir.join("recent_repos.json");
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| JanusError::ConfigError(format!("Failed to serialize recent repositories: {}", e)))?;
        
        fs::write(recent_repos_path, content)
            .map_err(|e| JanusError::IoError(format!("Failed to write recent repositories: {}", e)))
    }

    fn add(&mut self, repo: RepoInfo) -> Result<(), JanusError> {
        // Remove if already exists
        self.repositories.retain(|r| r.path != repo.path);
        
        // Add to the front
        self.repositories.insert(0, repo);
        
        // Keep only the 20 most recent
        if self.repositories.len() > 20 {
            self.repositories.truncate(20);
        }
        
        self.save()
    }
}

/// Get the configuration directory for JanusLens
fn get_config_dir() -> Result<PathBuf, JanusError> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| JanusError::ConfigError("Could not find home directory".to_string()))?;
    
    #[cfg(target_os = "macos")]
    let config_dir = home_dir.join("Library/Application Support/com.januslens.dev");
    
    #[cfg(target_os = "windows")]
    let config_dir = home_dir.join("AppData/Roaming/JanusLens");
    
    #[cfg(target_os = "linux")]
    let config_dir = home_dir.join(".config/januslens");
    
    Ok(config_dir)
}

/// List recently opened repositories
#[tauri::command]
pub fn list_repositories() -> Result<Vec<RepoInfo>, JanusError> {
    let recent_repos = RecentRepos::load()?;
    Ok(recent_repos.repositories)
}

/// Open a repository and add it to recent repositories
#[tauri::command]
pub fn open_repository(path: String) -> Result<RepoInfo, JanusError> {
    // Validate that this is a git repository
    let repo_path = Path::new(&path);
    if !repo_path.exists() {
        return Err(JanusError::GitError(format!("Path does not exist: {}", path)));
    }
    
    // Try to open as git repository to validate
    Repository::open(repo_path)
        .map_err(|e| JanusError::GitError(format!("Not a valid git repository: {}", e)))?;
    
    // Convert to absolute path if it's relative
    let absolute_path = if repo_path.is_absolute() {
        repo_path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| JanusError::IoError(format!("Failed to get current directory: {}", e)))?
            .join(repo_path)
            .canonicalize()
            .map_err(|e| JanusError::IoError(format!("Failed to canonicalize path: {}", e)))?
    };
    
    // Create repo info using the helper method
    let repo_info = RepoInfo::from_path(absolute_path)?;
    
    // Add to recent repositories
    let mut recent_repos = RecentRepos::load()?;
    recent_repos.add(repo_info.clone())?;
    
    Ok(repo_info)
}

/// Check if a path is a valid git repository
#[tauri::command]
pub fn is_git_repository(path: String) -> Result<bool, JanusError> {
    let repo_path = Path::new(&path);
    if !repo_path.exists() {
        return Ok(false);
    }
    
    // Convert to absolute path if it's relative
    let absolute_path = if repo_path.is_absolute() {
        repo_path.to_path_buf()
    } else {
        match std::env::current_dir().map(|dir| dir.join(repo_path)) {
            Ok(path) => path,
            Err(_) => return Ok(false), // If we can't resolve the path, it's not a valid repo
        }
    };
    
    match Repository::open(absolute_path) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Get common locations where git repositories might be found
#[tauri::command]
pub fn get_common_repo_locations() -> Result<Vec<String>, JanusError> {
    let mut locations = Vec::new();
    
    if let Some(home_dir) = dirs::home_dir() {
        // Common project directories
        let common_dirs = vec![
            "projects", "repos", "git", "workspace", "code", "dev", "development",
            "Documents/projects", "Documents/repos", "Documents/git", "Documents/code",
        ];
        
        for dir in common_dirs {
            let path = home_dir.join(dir);
            if path.exists() && path.is_dir() {
                if let Some(path_str) = path.to_str() {
                    locations.push(path_str.to_string());
                }
            }
        }
    }
    
    Ok(locations)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchInfo {
    pub name: String,
    pub is_head: bool,
    pub upstream: Option<String>,
    pub commit_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitInfo {
    pub id: String,
    pub short_id: String,
    pub summary: String,
    pub message: String,
    pub author: String,
    pub author_email: String,
    pub time: i64,
    pub parent_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileStatus {
    pub path: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoStatus {
    pub staged: Vec<FileStatus>,
    pub unstaged: Vec<FileStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub type_: String,
}

/// Gets all branches in the repository
#[tauri::command]
pub fn get_branches(repo_path: String) -> Result<Vec<BranchInfo>, JanusError> {
    let repo = Repository::open(&repo_path).map_err(|e| {
        error!("Failed to open repository at {}: {}", repo_path, e);
        JanusError::GitError(format!("Failed to open repository: {}", e))
    })?;
    
    let mut branches = Vec::new();
    let head = repo.head().ok();
    let head_oid = head.as_ref().and_then(|h| h.target());
    
    // Get local branches
    for branch_result in repo.branches(Some(BranchType::Local)).map_err(|e| {
        error!("Failed to get branches: {}", e);
        JanusError::GitError(format!("Failed to get branches: {}", e))
    })? {
        let (branch, _) = branch_result.map_err(|e| {
            error!("Failed to get branch info: {}", e);
            JanusError::GitError(format!("Failed to get branch info: {}", e))
        })?;
        
        let name = branch.name().map_err(|e| {
            error!("Failed to get branch name: {}", e);
            JanusError::GitError(format!("Failed to get branch name: {}", e))
        })?;
        
        if let Some(name) = name {
            let commit_id = branch.get().target().map_or_else(|| "".to_string(), |id| id.to_string());
            let is_head = head_oid.map_or(false, |h| branch.get().target().map_or(false, |target| h == target));
            
            let upstream_name = match branch.upstream() {
                Ok(upstream_branch) => {
                    match upstream_branch.name() {
                        Ok(Some(name)) => Some(name.to_string()),
                        _ => None,
                    }
                },
                Err(_) => None,
            };
            
            branches.push(BranchInfo {
                name: name.to_string(),
                is_head,
                upstream: upstream_name,
                commit_id,
            });
        }
    }
    
    Ok(branches)
}

/// Gets commits for a specific branch
#[tauri::command]
pub fn get_commits(repo_path: String, branch_name: Option<String>, limit: Option<u32>) -> Result<Vec<CommitInfo>, JanusError> {
    let repo = Repository::open(&repo_path).map_err(|e| {
        error!("Failed to open repository at {}: {}", repo_path, e);
        JanusError::GitError(format!("Failed to open repository: {}", e))
    })?;
    
    let limit = limit.unwrap_or(100);
    let mut commits = Vec::new();
    
    // Determine which branch to use
    let oid = if let Some(branch) = branch_name {
        // Get the specific branch
        let branch_ref = repo.find_branch(&branch, BranchType::Local).map_err(|e| {
            error!("Failed to find branch {}: {}", branch, e);
            JanusError::GitError(format!("Failed to find branch {}: {}", branch, e))
        })?;
        
        branch_ref.get().target().ok_or_else(|| {
            error!("Branch {} does not point to a valid commit", branch);
            JanusError::GitError(format!("Branch {} does not point to a valid commit", branch))
        })?
    } else {
        // Use HEAD
        repo.head().map_err(|e| {
            error!("Failed to get HEAD reference: {}", e);
            JanusError::GitError(format!("Failed to get HEAD reference: {}", e))
        })?.target().ok_or_else(|| {
            error!("HEAD does not point to a valid commit");
            JanusError::GitError("HEAD does not point to a valid commit".to_string())
        })?
    };
    
    // Start the revision walker
    let mut revwalk = repo.revwalk().map_err(|e| {
        error!("Failed to create revision walker: {}", e);
        JanusError::GitError(format!("Failed to create revision walker: {}", e))
    })?;
    
    revwalk.push(oid).map_err(|e| {
        error!("Failed to push commit to revision walker: {}", e);
        JanusError::GitError(format!("Failed to push commit to revision walker: {}", e))
    })?;
    
    // Collect commits
    for (i, oid_result) in revwalk.enumerate() {
        if i >= limit as usize {
            break;
        }
        
        let oid = oid_result.map_err(|e| {
            error!("Failed to get commit ID: {}", e);
            JanusError::GitError(format!("Failed to get commit ID: {}", e))
        })?;
        
        let commit = repo.find_commit(oid).map_err(|e| {
            error!("Failed to find commit {}: {}", oid, e);
            JanusError::GitError(format!("Failed to find commit {}: {}", oid, e))
        })?;
        
        commits.push(commit_to_info(&commit)?);
    }
    
    Ok(commits)
}

// Helper function to convert a git2::Commit to our CommitInfo
fn commit_to_info(commit: &Commit) -> Result<CommitInfo, JanusError> {
    let id = commit.id().to_string();
    let short_id = id.chars().take(8).collect();
    
    let summary = commit.summary()
        .unwrap_or("No summary available")
        .to_string();
    
    let message = commit.message()
        .unwrap_or("No message available")
        .to_string();
    
    let author = commit.author();
    let author_name = author.name()
        .unwrap_or("Unknown")
        .to_string();
    
    let author_email = author.email()
        .unwrap_or("Unknown")
        .to_string();
    
    let time = commit.time().seconds();
    
    let parent_count = commit.parent_count();
    let mut parent_ids = Vec::with_capacity(parent_count);
    
    for i in 0..parent_count {
        if let Ok(parent) = commit.parent(i) {
            parent_ids.push(parent.id().to_string());
        }
    }
    
    Ok(CommitInfo {
        id,
        short_id,
        summary,
        message,
        author: author_name,
        author_email,
        time,
        parent_ids,
    })
}

/// Gets the status of files in the repository
#[tauri::command]
pub fn get_status(repo_path: String) -> Result<RepoStatus, JanusError> {
    let repo = Repository::open(&repo_path).map_err(|e| {
        error!("Failed to open repository at {}: {}", repo_path, e);
        JanusError::GitError(format!("Failed to open repository: {}", e))
    })?;
    
    let mut options = StatusOptions::new();
    options.include_untracked(true)
           .show(StatusShow::IndexAndWorkdir)
           .include_unmodified(false)
           .exclude_submodules(false);
    
    let statuses = repo.statuses(Some(&mut options)).map_err(|e| {
        error!("Failed to get repository status: {}", e);
        JanusError::GitError(format!("Failed to get repository status: {}", e))
    })?;
    
    let mut staged = Vec::new();
    let mut unstaged = Vec::new();
    
    for status_entry in statuses.iter() {
        let path = status_entry.path().unwrap_or("").to_string();
        let status = status_entry.status();
        
        // Check index (staged) changes
        if status.is_index_new() {
            staged.push(FileStatus {
                path: path.clone(),
                status: "new".to_string(),
            });
        } else if status.is_index_modified() {
            staged.push(FileStatus {
                path: path.clone(),
                status: "modified".to_string(),
            });
        } else if status.is_index_deleted() {
            staged.push(FileStatus {
                path: path.clone(),
                status: "deleted".to_string(),
            });
        } else if status.is_index_renamed() {
            staged.push(FileStatus {
                path: path.clone(),
                status: "renamed".to_string(),
            });
        } else if status.is_index_typechange() {
            staged.push(FileStatus {
                path: path.clone(),
                status: "typechange".to_string(),
            });
        }
        
        // Check worktree (unstaged) changes
        if status.is_wt_new() {
            unstaged.push(FileStatus {
                path: path.clone(),
                status: "new".to_string(),
            });
        } else if status.is_wt_modified() {
            unstaged.push(FileStatus {
                path: path.clone(),
                status: "modified".to_string(),
            });
        } else if status.is_wt_deleted() {
            unstaged.push(FileStatus {
                path: path.clone(),
                status: "deleted".to_string(),
            });
        } else if status.is_wt_renamed() {
            unstaged.push(FileStatus {
                path: path.clone(),
                status: "renamed".to_string(),
            });
        } else if status.is_wt_typechange() {
            unstaged.push(FileStatus {
                path: path.clone(),
                status: "typechange".to_string(),
            });
        }
    }
    
    Ok(RepoStatus { staged, unstaged })
}

/// Gets the diff for a specific file
#[tauri::command]
pub fn get_diff(repo_path: String, file_path: String, staged: bool) -> Result<String, JanusError> {
    let _repo = Repository::open(&repo_path).map_err(|e| {
        error!("Failed to open repository at {}: {}", repo_path, e);
        JanusError::GitError(format!("Failed to open repository: {}", e))
    })?;
    
    // For simplicity, we'll use git command line to get the diff
    // In a real implementation, you would use git2's diff functionality
    
    let output = if staged {
        std::process::Command::new("git")
            .args(&["diff", "--cached", "--", &file_path])
            .current_dir(&repo_path)
            .output()
            .map_err(|e| {
                error!("Failed to execute git diff command: {}", e);
                JanusError::IoError(format!("Failed to execute git diff command: {}", e))
            })?
    } else {
        std::process::Command::new("git")
            .args(&["diff", "--", &file_path])
            .current_dir(&repo_path)
            .output()
            .map_err(|e| {
                error!("Failed to execute git diff command: {}", e);
                JanusError::IoError(format!("Failed to execute git diff command: {}", e))
            })?
    };
    
    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr).to_string();
        error!("Git diff command failed: {}", error_message);
        return Err(JanusError::GitError(format!("Git diff command failed: {}", error_message)));
    }
    
    let diff_output = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(diff_output)
}

/// Stages a file
#[tauri::command]
pub fn stage_file(repo_path: String, file_path: String) -> Result<(), JanusError> {
    let _repo = Repository::open(&repo_path).map_err(|e| {
        error!("Failed to open repository at {}: {}", repo_path, e);
        JanusError::GitError(format!("Failed to open repository: {}", e))
    })?;
    
    // Use git command line for simplicity
    let output = std::process::Command::new("git")
        .args(&["add", &file_path])
        .current_dir(&repo_path)
        .output()
        .map_err(|e| {
            error!("Failed to execute git add command: {}", e);
            JanusError::IoError(format!("Failed to execute git add command: {}", e))
        })?;
    
    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr).to_string();
        error!("Git add command failed: {}", error_message);
        return Err(JanusError::GitError(format!("Git add command failed: {}", error_message)));
    }
    
    Ok(())
}

/// Unstages a file
#[tauri::command]
pub fn unstage_file(repo_path: String, file_path: String) -> Result<(), JanusError> {
    let _repo = Repository::open(&repo_path).map_err(|e| {
        error!("Failed to open repository at {}: {}", repo_path, e);
        JanusError::GitError(format!("Failed to open repository: {}", e))
    })?;
    
    // Use git command line for simplicity
    let output = std::process::Command::new("git")
        .args(&["reset", "--", &file_path])
        .current_dir(&repo_path)
        .output()
        .map_err(|e| {
            error!("Failed to execute git reset command: {}", e);
            JanusError::IoError(format!("Failed to execute git reset command: {}", e))
        })?;
    
    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr).to_string();
        error!("Git reset command failed: {}", error_message);
        return Err(JanusError::GitError(format!("Git reset command failed: {}", error_message)));
    }
    
    Ok(())
}

/// Creates a commit
#[tauri::command]
pub fn create_commit(repo_path: String, message: String) -> Result<CommitInfo, JanusError> {
    let repo = Repository::open(&repo_path).map_err(|e| {
        error!("Failed to open repository at {}: {}", repo_path, e);
        JanusError::GitError(format!("Failed to open repository: {}", e))
    })?;
    
    // Use git command line for simplicity
    let output = std::process::Command::new("git")
        .args(&["commit", "-m", &message])
        .current_dir(&repo_path)
        .output()
        .map_err(|e| {
            error!("Failed to execute git commit command: {}", e);
            JanusError::IoError(format!("Failed to execute git commit command: {}", e))
        })?;
    
    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr).to_string();
        error!("Git commit command failed: {}", error_message);
        return Err(JanusError::GitError(format!("Git commit command failed: {}", error_message)));
    }
    
    // Get the created commit
    let head = repo.head().map_err(|e| {
        error!("Failed to get HEAD reference: {}", e);
        JanusError::GitError(format!("Failed to get HEAD reference: {}", e))
    })?;
    
    let head_commit = head.peel_to_commit().map_err(|e| {
        error!("Failed to peel HEAD to commit: {}", e);
        JanusError::GitError(format!("Failed to peel HEAD to commit: {}", e))
    })?;
    
    commit_to_info(&head_commit)
}

/// Creates a new branch
#[tauri::command]
pub fn create_branch(repo_path: String, branch_name: String) -> Result<BranchInfo, JanusError> {
    let repo = Repository::open(&repo_path).map_err(|e| {
        error!("Failed to open repository at {}: {}", repo_path, e);
        JanusError::GitError(format!("Failed to open repository: {}", e))
    })?;
    
    // Get HEAD commit to create branch from
    let head = repo.head().map_err(|e| {
        error!("Failed to get HEAD reference: {}", e);
        JanusError::GitError(format!("Failed to get HEAD reference: {}", e))
    })?;
    
    let head_commit = head.peel_to_commit().map_err(|e| {
        error!("Failed to peel HEAD to commit: {}", e);
        JanusError::GitError(format!("Failed to peel HEAD to commit: {}", e))
    })?;
    
    // Create the branch
    let branch_ref = repo.branch(&branch_name, &head_commit, false).map_err(|e| {
        error!("Failed to create branch {}: {}", branch_name, e);
        JanusError::GitError(format!("Failed to create branch {}: {}", branch_name, e))
    })?;
    
    let branch_oid = branch_ref.get().target().map_or_else(|| Oid::zero(), |oid| oid);
    
    Ok(BranchInfo {
        name: branch_name,
        is_head: false,
        upstream: None,
        commit_id: branch_oid.to_string(),
    })
}

/// Deletes a branch
#[tauri::command]
pub fn delete_branch(repo_path: String, branch_name: String) -> Result<(), JanusError> {
    let repo = Repository::open(&repo_path).map_err(|e| {
        error!("Failed to open repository at {}: {}", repo_path, e);
        JanusError::GitError(format!("Failed to open repository: {}", e))
    })?;
    
    // Find the branch
    let mut branch = repo.find_branch(&branch_name, BranchType::Local).map_err(|e| {
        error!("Failed to find branch {}: {}", branch_name, e);
        JanusError::GitError(format!("Failed to find branch {}: {}", branch_name, e))
    })?;
    
    // Delete the branch
    branch.delete().map_err(|e| {
        error!("Failed to delete branch {}: {}", branch_name, e);
        JanusError::GitError(format!("Failed to delete branch {}: {}", branch_name, e))
    })?;
    
    Ok(())
}

/// Lists files in the repository
#[tauri::command]
pub fn list_files(repo_path: String, directory: Option<String>) -> Result<Vec<FileEntry>, JanusError> {
    let repo_path_buf = PathBuf::from(&repo_path);
    let target_dir = match directory {
        Some(dir) => repo_path_buf.join(dir),
        None => repo_path_buf.clone(),
    };
    
    if !target_dir.exists() {
        return Err(JanusError::IoError(format!("Directory does not exist: {:?}", target_dir)));
    }
    
    let mut entries = Vec::new();
    
    for entry in std::fs::read_dir(&target_dir).map_err(|e| {
        error!("Failed to read directory {:?}: {}", target_dir, e);
        JanusError::IoError(format!("Failed to read directory: {}", e))
    })? {
        let entry = entry.map_err(|e| {
            error!("Failed to read directory entry: {}", e);
            JanusError::IoError(format!("Failed to read directory entry: {}", e))
        })?;
        
        let path = entry.path();
        let relative_path = path.strip_prefix(&repo_path_buf)
            .map_err(|e| JanusError::IoError(format!("Failed to get relative path: {}", e)))?
            .to_string_lossy()
            .to_string();
        
        let file_type = if path.is_dir() {
            "directory".to_string()
        } else {
            "file".to_string()
        };
        
        let name = path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        entries.push(FileEntry {
            name,
            path: relative_path,
            type_: file_type,
        });
    }
    
    // Sort: directories first, then files, both alphabetically
    entries.sort_by(|a, b| {
        if a.type_ == b.type_ {
            a.name.cmp(&b.name)
        } else if a.type_ == "directory" {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    
    Ok(entries)
}

/// Checks out the specified branch
#[tauri::command]
pub fn checkout_branch(repo_path: String, branch_name: String) -> Result<(), JanusError> {
    let repo = Repository::open(&repo_path).map_err(|e| {
        error!("Failed to open repository at {}: {}", repo_path, e);
        JanusError::GitError(format!("Failed to open repository: {}", e))
    })?;
    
    // Find the branch
    let branch = repo.find_branch(&branch_name, BranchType::Local).map_err(|e| {
        error!("Failed to find branch {}: {}", branch_name, e);
        JanusError::GitError(format!("Failed to find branch {}: {}", branch_name, e))
    })?;
    
    // Get the branch reference
    let branch_ref = branch.get();
    
    // Checkout the branch
    let obj = branch_ref.peel(git2::ObjectType::Any).map_err(|e| {
        error!("Failed to peel branch reference: {}", e);
        JanusError::GitError(format!("Failed to peel branch reference: {}", e))
    })?;
    
    repo.checkout_tree(&obj, None).map_err(|e| {
        error!("Failed to checkout tree: {}", e);
        JanusError::GitError(format!("Failed to checkout tree: {}", e))
    })?;
    
    // Set HEAD to the new branch
    repo.set_head(branch_ref.name().unwrap_or("")).map_err(|e| {
        error!("Failed to set HEAD: {}", e);
        JanusError::GitError(format!("Failed to set HEAD: {}", e))
    })?;
    
    info!("Successfully checked out branch: {}", branch_name);
    Ok(())
}

#[cfg(test)]
mod tests {
    include!("git_test.rs");
} 