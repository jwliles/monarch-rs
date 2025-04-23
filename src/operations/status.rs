// src/operations/status.rs
use crate::types::{OperationResult, RepositoryOperation};
use anyhow::{Context, Result};
use std::path::Path;
use tracing::{debug, info};

pub struct StatusOperation;

impl StatusOperation {
    pub fn new() -> Self {
        Self
    }
}

impl RepositoryOperation for StatusOperation {
    fn name(&self) -> &'static str {
        "Status"
    }

    fn description(&self) -> &'static str {
        "Show status of repositories"
    }

    fn execute<'a>(&'a self, repo_path: &'a Path) -> 
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<OperationResult>> + Send + 'a>> {
        Box::pin(async move {
            // Check if it's a valid git repository
            if !repo_path.join(".git").is_dir() {
                return Ok(OperationResult::Skipped(format!(
                    "Not a git repository: {}",
                    repo_path.display()
                )));
            }

            // Run git status command
            let mut cmd = tokio::process::Command::new("git");
            cmd.current_dir(repo_path).arg("status").arg("--porcelain");

            debug!("Running status in {}: {:?}", repo_path.display(), cmd);
            
            let output = cmd
                .output()
                .await
                .context("Failed to execute git status command")?;

            if output.status.success() {
                let status_output = String::from_utf8_lossy(&output.stdout).to_string();
                
                if status_output.is_empty() {
                    // Clean repository
                    Ok(OperationResult::Success(format!(
                        "Clean repository: {}",
                        repo_path.display()
                    )))
                } else {
                    // Repository has uncommitted changes
                    Ok(OperationResult::Success(format!(
                        "Uncommitted changes in {}: \n{}",
                        repo_path.display(),
                        status_output
                    )))
                }
            } else {
                let error = String::from_utf8_lossy(&output.stderr).to_string();
                Ok(OperationResult::Failure(format!(
                    "Failed to get status for {}: {}",
                    repo_path.display(),
                    error
                )))
            }
        })
    }
}