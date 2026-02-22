//! Git operations for Rift
//!
//! Handles cloning git repositories for dependencies and plugins.

use anyhow::{Result, anyhow};
use git2::{FetchOptions, Oid, build::RepoBuilder};
use std::fs;
use std::path::{Path, PathBuf};

/// Git cache directory under the user's home directory
const RIFT_CACHE_DIR: &str = ".rift";
const GIT_CACHE_DIR: &str = "git";

/// Get the Rift cache directory
pub fn cache_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow!("Could not determine home directory"))?;
    Ok(home.join(RIFT_CACHE_DIR).join(GIT_CACHE_DIR))
}

/// Get the cache directory for a specific git repository
///
/// The cache path is derived from the URL to ensure:
/// - Same URL always maps to same cache location
/// - Different URLs don't collide
pub fn repo_cache_dir(url: &str) -> Result<PathBuf> {
    let cache = cache_dir()?;

    // Create a deterministic directory name from the URL
    // Format: <domain>/<path>.git
    // e.g., https://github.com/user/repo -> github.com/user/repo.git

    let url_without_proto = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .or_else(|| url.strip_prefix("git://"))
        .unwrap_or(url);

    let clean_path: String = url_without_proto
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '/' || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect();

    let cache_path = cache.join(clean_path);

    // Ensure it ends with .git to mark it as a bare/repository clone
    let path_str = cache_path.to_string_lossy();
    if !path_str.ends_with(".git") {
        Ok(cache_path.with_extension("git"))
    } else {
        Ok(cache_path)
    }
}

/// Clone or update a git repository to the cache
///
/// Returns the path to the cached repository
pub fn clone_or_update(url: &str, ref_: Option<&str>) -> Result<PathBuf> {
    let cache_path = repo_cache_dir(url)?;

    // Ensure cache directory exists
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)?;
    }

    if cache_path.exists() {
        // Repository exists, fetch and update
        update_repo(&cache_path, ref_)?;
    } else {
        // Clone fresh
        clone_repo(url, &cache_path, ref_)?;
    }

    Ok(cache_path)
}

/// Clone a git repository to a specific path
fn clone_repo(url: &str, path: &Path, ref_: Option<&str>) -> Result<()> {
    let mut builder = RepoBuilder::new();

    // Clone with bare repository for efficiency
    builder.bare(true);

    let repo = builder.clone(url, path)?;

    // If a ref is specified, fetch and checkout
    if let Some(r) = ref_ {
        checkout_ref(&repo, r)?;
    }

    Ok(())
}

/// Update an existing repository by fetching
fn update_repo(path: &Path, ref_: Option<&str>) -> Result<()> {
    let repo = git2::Repository::open(path)?;

    // Fetch all remotes
    let mut remote = repo.find_remote("origin")?;
    let mut fetch_options = FetchOptions::new();
    remote.fetch(&["refs/*:refs/*"], Some(&mut fetch_options), None)?;

    // If a ref is specified, update to it
    if let Some(r) = ref_ {
        checkout_ref(&repo, r)?;
    }

    Ok(())
}

/// Checkout a specific reference (branch, tag, or commit)
fn checkout_ref(repo: &git2::Repository, ref_: &str) -> Result<()> {
    // Try to resolve as a branch
    if let Ok(branch) = get_branch_oid(repo, ref_) {
        set_head_to_oid(repo, branch)?;
        return Ok(());
    }

    // Try to resolve as a tag
    if let Ok(tag) = get_tag_oid(repo, ref_) {
        set_head_to_oid(repo, tag)?;
        return Ok(());
    }

    // Try to resolve as a commit hash
    if let Ok(commit) = Oid::from_str(ref_) {
        if repo.find_commit(commit).is_ok() {
            set_head_to_oid(repo, commit)?;
            return Ok(());
        }
    }

    // If we have a remote ref, fetch and try again
    let remote_ref = format!("refs/remotes/origin/{}", ref_);
    if let Ok(object) = repo.revparse_single(&remote_ref) {
        if let Some(commit) = object.as_commit() {
            set_head_to_oid(repo, commit.id())?;
            return Ok(());
        }
    }

    Err(anyhow!("Could not resolve ref: {}", ref_))
}

/// Get the OID for a branch name
fn get_branch_oid(repo: &git2::Repository, branch_name: &str) -> Result<Oid> {
    let branch_ref = format!("refs/heads/{}", branch_name);
    repo.refname_to_id(&branch_ref)
        .or_else(|_| {
            // Try remote branch
            let remote_branch = format!("refs/remotes/origin/{}", branch_name);
            repo.refname_to_id(&remote_branch)
        })
        .map_err(|_| anyhow!("Branch '{}' not found", branch_name))
}

/// Get the OID for a tag name
fn get_tag_oid(repo: &git2::Repository, tag_name: &str) -> Result<Oid> {
    let tag_ref = format!("refs/tags/{}", tag_name);
    repo.refname_to_id(&tag_ref)
        .map_err(|_| anyhow!("Tag '{}' not found", tag_name))
}

/// Set HEAD to a specific OID
fn set_head_to_oid(repo: &git2::Repository, oid: Oid) -> Result<()> {
    // For bare repos, we just set HEAD directly
    let commit = repo.find_commit(oid)?;
    repo.set_head_detached(commit.id())?;
    Ok(())
}

/// Get the working directory for a cached repository
///
/// Since we use bare repositories, the working tree needs to be checked out
/// to a temporary or project-specific location. For now, return the cache path.
pub fn work_dir_for(url: &str) -> Result<PathBuf> {
    repo_cache_dir(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_dir_path() {
        let url = "https://github.com/user/repo";
        let path = repo_cache_dir(url).unwrap();
        let path_str = path.to_string_lossy();
        assert!(path_str.contains("github.com"));
        assert!(path_str.ends_with(".git"));
    }

    #[test]
    fn test_cache_dir_with_github_url() {
        let url = "https://github.com/laper32/rift";
        let path = repo_cache_dir(url).unwrap();
        let path_str = path.to_string_lossy();
        assert!(path_str.contains("github.com"));
        assert!(path_str.contains("laper32"));
    }
}
