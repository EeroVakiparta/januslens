use crate::error::JanusError;
use git2::{Branch, BranchType, Commit, Repository, Oid, ErrorCode};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use log::{debug, error, info};

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoInfo {
    pub path: String,
    pub name: String,
    pub last_accessed: u64,
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

/// Lists recently accessed repositories
#[tauri::command]
pub fn list_repositories() -> Result<Vec<RepoInfo>, JanusError> {
    // In a real implementation, this would load from a config file or database
    // For now, we'll return an empty list
    Ok(Vec::new())
}

/// Opens a Git repository at the specified path
#[tauri::command]
pub fn open_repository(path: String) -> Result<RepoInfo, JanusError> {
    let repo_path = Path::new(&path);
    
    // Attempt to open the repository
    let repo = Repository::open(repo_path).map_err(|e| {
        error!("Failed to open repository at {}: {}", path, e);
        JanusError::GitError(format!("Failed to open repository: {}", e))
    })?;
    
    // Get repository information
    let repo_name = repo_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Unknown")
        .to_string();
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    info!("Successfully opened repository: {}", repo_name);
    
    Ok(RepoInfo {
        path,
        name: repo_name,
        last_accessed: now,
    })
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
            let commit_id = branch.get().target().unwrap_or_default().to_string();
            let is_head = head_oid.map_or(false, |h| h == branch.get().target().unwrap_or_default());
            
            let upstream_name = branch.upstream()
                .ok()
                .and_then(|b| b.name().ok().flatten())
                .map(|n| n.to_string());
            
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