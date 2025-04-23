// src/git/repository.rs
use anyhow::{Context, Result};
use git2::{Repository as Git2Repository, RepositoryState};
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, info};

pub struct Repository {
    pub path: PathBuf,
    pub size: u64,
    pub inner: Option<Git2Repository>,
}

impl Repository {
    pub async fn new(path: PathBuf) -> Result<Self> {
        let size = calculate_repository_size(&path).await;
        let inner = Git2Repository::open(&path).ok();
        
        Ok(Self { 
            path, 
            size,
            inner,
        })
    }

    pub fn is_git_repository(&self) -> bool {
        self.inner.is_some() || self.path.join(".git").is_dir()
    }
    
    pub fn get_state(&self) -> Option<RepositoryState> {
        self.inner.as_ref().map(|repo| repo.state())
    }
    
    pub fn has_hook(&self, hook_name: &str) -> bool {
        if let Some(repo) = &self.inner {
            if let Some(hooks_path) = repo.path().parent().map(|p| p.join(".git/hooks")) {
                return hooks_path.join(hook_name).exists();
            }
        }
        false
    }
    
    pub fn hooks_path(&self) -> Option<PathBuf> {
        self.inner.as_ref().map(|repo| {
            let git_path = repo.path();
            git_path.join("hooks")
        })
    }
}

/// Recursively finds all Git repositories in the given directory
pub async fn find_git_repositories(dir: &Path) -> Result<Vec<Repository>> {
    _find_git_repositories(dir).await
}

async fn _find_git_repositories(dir: &Path) -> Result<Vec<Repository>> {
    let mut all_repos = Vec::new();
    let mut dirs_to_process = vec![dir.to_path_buf()];
    
    while let Some(current_dir) = dirs_to_process.pop() {
        let mut entries = fs::read_dir(&current_dir)
            .await
            .context(format!("Failed to read directory: {}", current_dir.display()))?;
            
        while let Some(entry) = entries
            .next_entry()
            .await
            .context("Failed to read directory entry")?
        {
            let path = entry.path();
            
            if path.join(".git").is_dir() {
                let repo = Repository::new(path)
                    .await
                    .context("Failed to create repository instance")?;
                all_repos.push(repo);
            } else if path.is_dir() {
                // Skip certain directories that we know won't contain repos
                if should_skip_directory(&path) {
                    debug!("Skipping directory: {}", path.display());
                    continue;
                }
                
                // Add to processing queue
                dirs_to_process.push(path);
            }
        }
    }
    
    Ok(all_repos)
}

/// Calculates the total size of a repository
pub async fn calculate_repository_size(path: &Path) -> u64 {
    match get_directory_size(path).await {
        Ok(size) => size,
        Err(e) => {
            debug!("Error calculating size for {}: {}", path.display(), e);
            0
        }
    }
}

/// Gets the total size of a directory recursively
async fn get_directory_size(path: &Path) -> Result<u64> {
    _get_directory_size(path).await
}

/// Internal implementation to avoid the recursion in async fn warning
async fn _get_directory_size(path: &Path) -> Result<u64> {
    // Using a non-recursive implementation to avoid Box::pin overhead
    let mut total_size = 0;
    let mut stack = vec![path.to_path_buf()];
    
    while let Some(current_path) = stack.pop() {
        let mut entries = fs::read_dir(&current_path)
            .await
            .context(format!("Failed to read directory: {}", current_path.display()))?;
        
        let mut entries_vec = Vec::new();
        
        while let Some(entry) = entries.next_entry().await? {
            entries_vec.push(entry);
        }
        
        for entry in entries_vec {
            let metadata = entry
                .metadata()
                .await
                .context("Failed to read file metadata")?;
            
            if metadata.is_file() {
                total_size += metadata.len();
            } else if metadata.is_dir() {
                stack.push(entry.path());
            }
        }
    }
    
    Ok(total_size)
}

/// Determines if a directory should be skipped during repository search
fn should_skip_directory(path: &Path) -> bool {
    let skip_dirs = [
        "node_modules",
        "target",
        "dist",
        "build",
        ".git",
        ".idea",
        ".vscode",
    ];

    if let Some(dir_name) = path.file_name() {
        if let Some(dir_str) = dir_name.to_str() {
            return skip_dirs.contains(&dir_str) || dir_str.starts_with('.');
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;
    use tokio::fs::create_dir_all;

    #[tokio::test]
    async fn test_find_git_repositories() {
        let temp_dir = TempDir::new().unwrap();
        let repo1_path = temp_dir.path().join("repo1");
        let repo2_path = temp_dir.path().join("dir").join("repo2");

        // Create test repositories
        create_dir_all(&repo1_path.join(".git")).await.unwrap();
        create_dir_all(&repo2_path.join(".git")).await.unwrap();

        let repos = find_git_repositories(temp_dir.path()).await.unwrap();
        assert_eq!(repos.len(), 2);
    }

    #[tokio::test]
    async fn test_calculate_repository_size() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");

        // Create a test file with known content
        File::create(&test_file)
            .unwrap()
            .write_all(b"test content")
            .unwrap();

        let size = calculate_repository_size(temp_dir.path()).await;
        assert_eq!(size, 12); // "test content" is 12 bytes
    }

    #[tokio::test]
    async fn test_should_skip_directory() {
        assert!(should_skip_directory(Path::new("node_modules")));
        assert!(should_skip_directory(Path::new(".git")));
        assert!(!should_skip_directory(Path::new("src")));
    }
}