// src/operations/executor.rs
use crate::args::Args;
use crate::git;
use crate::types::{Operation, OperationResult, RepositoryOperation};
use tracing::{debug, info};

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct OperationExecutor {
    max_concurrency: usize,
}

impl OperationExecutor {
    pub fn new(max_concurrency: usize) -> Self {
        Self { max_concurrency }
    }

    pub async fn execute(&self, args: Args) -> anyhow::Result<()> {
        info!("Finding repositories in {}", args.source_dir.display());

        // Find all Git repositories
        let repositories = git::find_git_repositories(&args.source_dir).await?;
        
        if repositories.is_empty() {
            info!("No Git repositories found in {}", args.source_dir.display());
            return Ok(());
        }

        info!("Found {} Git repositories", repositories.len());

        // Store the operation type and clone args for reuse
        let op_type = args.operation.clone();
        let args_clone = args.clone();
        let filter = args.filter.clone();
        
        // Create semaphore for concurrency control
        let semaphore = Arc::new(Semaphore::new(self.max_concurrency));
        
        // Execute operation on each repository concurrently
        let mut handles = vec![];

        for repo in repositories {
            if !repo.is_git_repository() {
                info!("Skipping non-Git directory: {}", repo.path.display());
                continue;
            }

            // Apply filter if specified
            if let Some(pattern) = &filter {
                let repo_name = repo.path.file_name().unwrap().to_string_lossy();
                if !repo_name.contains(pattern) {
                    debug!("Filtering out repository: {}", repo.path.display());
                    continue;
                }
            }

            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let repo_path = repo.path.clone();
            let op_type_clone = op_type.clone();
            let args_clone2 = args_clone.clone();
            
            let handle = tokio::spawn(async move {
                let repo_path_clone = repo_path.clone();
                info!("Running operation on {}", repo_path.display());
                
                // Create a new operation instance for each repository
                let operation = crate::operations::create_operation(&op_type_clone, &args_clone2);
                let result = operation.execute(&repo_path).await;
                
                match result {
                    Ok(OperationResult::Success(msg)) => {
                        info!("✅ Success: {}", msg);
                    }
                    Ok(OperationResult::Failure(msg)) => {
                        info!("❌ Failed: {}", msg);
                    }
                    Ok(OperationResult::Skipped(msg)) => {
                        info!("⏭️ Skipped: {}", msg);
                    }
                    Err(e) => {
                        info!("❌ Error: {} - {}", repo_path_clone.display(), e);
                    }
                }
                
                // Drop permit when done to allow another task to run
                drop(permit);
            });
            
            handles.push(handle);
        }

        // Wait for all operations to complete
        for handle in handles {
            handle.await?;
        }

        Ok(())
    }
    
    fn _filter_repositories<'a>(&self, 
        repositories: &'a [git::Repository], 
        _operation: &Operation,
        pattern: Option<&str>
    ) -> Vec<&'a git::Repository> {
        repositories.iter()
            .filter(|repo| {
                if let Some(pattern) = pattern {
                    let repo_name = repo.path.file_name().unwrap().to_string_lossy();
                    if !repo_name.contains(pattern) && !regex::Regex::new(&regex::escape(pattern)).unwrap().is_match(&repo_name) {
                        return false;
                    }
                }
                
                true
            })
            .collect()
    }
}