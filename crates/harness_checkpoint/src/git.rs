//! Git-based checkpoint implementation

use crate::checkpoint::{Checkpoint, CheckpointOptions, CheckpointStatus};
use crate::error::{CheckpointError, Result};
use chrono::Utc;
use git2::{build::CheckoutBuilder, Repository, Signature, StatusOptions};
use std::path::Path;
use uuid::Uuid;

/// Create a git checkpoint
pub fn create_git_checkpoint(
    repo_path: &Path,
    spec_id: &str,
    options: &CheckpointOptions,
) -> Result<Checkpoint> {
    // Open or create repository
    let repo = Repository::open(repo_path)
        .or_else(|_| Repository::init(repo_path))
        .map_err(|e| CheckpointError::GitError(e.message().to_string()))?;

    let mut checkpoint = Checkpoint {
        id: Uuid::new_v4(),
        spec_id: spec_id.to_string(),
        git_sha: None,
        git_message: None,
        config_snapshot: None,
        db_snapshot_id: None,
        metrics_baseline: None,
        created_at: Utc::now(),
        status: CheckpointStatus::Creating,
        metadata: std::collections::HashMap::new(),
    };

    // Stage all changes
    let mut index = repo
        .index()
        .map_err(|e| CheckpointError::GitError(e.message().to_string()))?;

    if options.include_uncommitted {
        // Add all files
        index
            .add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)
            .map_err(|e| CheckpointError::GitError(e.message().to_string()))?;

        index
            .write()
            .map_err(|e| CheckpointError::GitError(e.message().to_string()))?;
    }

    // Create commit
    let message = options
        .message
        .clone()
        .unwrap_or_else(|| format!("Checkpoint for spec: {}", spec_id));

    let signature = Signature::now("heliosHarness", "checkpoint@helios.local")
        .unwrap_or_else(|_| Signature::now("heliosHarness", "checkpoint@helios.local").unwrap());

    let oid = index
        .write_tree()
        .map_err(|e| CheckpointError::GitError(e.message().to_string()))?;

    let tree = repo
        .find_tree(oid)
        .map_err(|e| CheckpointError::GitError(e.message().to_string()))?;

    // Get parent commit
    let parent_commit = repo.head().ok().and_then(|h| h.peel_to_commit().ok());

    let parents: Vec<&git2::Commit> = parent_commit.iter().collect();

    let commit_oid = repo
        .commit(
            Some("HEAD"),
            &signature,
            &signature,
            &message,
            &tree,
            &parents,
        )
        .map_err(|e| CheckpointError::GitError(e.message().to_string()))?;

    let commit = repo
        .find_commit(commit_oid)
        .map_err(|e| CheckpointError::GitError(e.message().to_string()))?;

    checkpoint.git_sha = Some(commit.id().to_string());
    checkpoint.git_message = Some(message);
    checkpoint.status = CheckpointStatus::Complete;

    Ok(checkpoint)
}

/// Restore from git checkpoint
pub fn restore_git_checkpoint(repo_path: &Path, git_sha: &str) -> Result<()> {
    let repo = Repository::open(repo_path)
        .map_err(|e| CheckpointError::RepositoryNotFound(e.message().to_string()))?;

    // Parse the commit SHA
    let oid = git_sha
        .parse::<git2::Oid>()
        .map_err(|e| CheckpointError::GitError(format!("Invalid SHA: {}", e)))?;

    // Checkout the commit
    let mut checkout_builder = CheckoutBuilder::new();
    checkout_builder.force().allow_conflicts(true);

    let commit = repo
        .find_commit(oid)
        .map_err(|e| CheckpointError::GitError(e.message().to_string()))?;

    let tree = commit
        .tree()
        .map_err(|e| CheckpointError::GitError(e.message().to_string()))?;

    repo.checkout_tree(tree.as_object(), Some(&mut checkout_builder))
        .map_err(|e| CheckpointError::RestoreFailed(e.message().to_string()))?;

    // Reset HEAD to the commit
    repo.set_head_detached(oid)
        .map_err(|e| CheckpointError::RestoreFailed(e.message().to_string()))?;

    Ok(())
}

/// Get current git status
pub fn get_git_status(repo_path: &Path) -> Result<GitStatus> {
    let repo = Repository::open(repo_path)
        .map_err(|e| CheckpointError::RepositoryNotFound(e.message().to_string()))?;

    let mut opts = StatusOptions::new();
    opts.include_untracked(true);

    let statuses = repo
        .statuses(Some(&mut opts))
        .map_err(|e| CheckpointError::GitError(e.message().to_string()))?;

    let mut modified = Vec::new();
    let mut staged = Vec::new();
    let mut untracked = Vec::new();

    for entry in statuses.iter() {
        let path = entry.path().unwrap_or("").to_string();

        let status = entry.status();

        if status.is_index_new() || status.is_index_modified() {
            staged.push(path.clone());
        }
        if status.is_wt_modified() {
            modified.push(path.clone());
        }
        if status.is_wt_new() {
            untracked.push(path);
        }
    }

    let is_clean = modified.is_empty() && staged.is_empty() && untracked.is_empty();

    Ok(GitStatus {
        modified,
        staged,
        untracked,
        is_clean,
    })
}

/// Git status
#[derive(Debug, Clone)]
pub struct GitStatus {
    pub modified: Vec<String>,
    pub staged: Vec<String>,
    pub untracked: Vec<String>,
    pub is_clean: bool,
}

/// Get current HEAD SHA
pub fn get_current_sha(repo_path: &Path) -> Result<String> {
    let repo = Repository::open(repo_path)
        .map_err(|e| CheckpointError::RepositoryNotFound(e.message().to_string()))?;

    let head = repo
        .head()
        .map_err(|e| CheckpointError::GitError(e.message().to_string()))?;

    let oid = head
        .peel_to_commit()
        .map_err(|e| CheckpointError::GitError(e.message().to_string()))?;

    Ok(oid.id().to_string())
}
