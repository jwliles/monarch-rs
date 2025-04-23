// src/operations/push.rs
use crate::types::{OperationResult, RepositoryOperation};
use anyhow::{Context, Result};
use std::path::Path;
use tracing::{debug, info};

pub struct PushOperation;

impl PushOperation {
    pub fn new() -> Self {
        Self
    }
}

impl RepositoryOperation for PushOperation {
    fn name(&self) -> &'static str {
        "Push"
    }

    fn description(&self) -> &'static str {
        "Push local changes to remote"
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

            // Run git push command
            let mut cmd = tokio::process::Command::new("git");
            cmd.current_dir(repo_path).arg("push");

            debug!("Running push in {}: {:?}", repo_path.display(), cmd);
            
            let output = cmd
                .output()
                .await
                .context("Failed to execute git push command")?;

            if output.status.success() {
                let message = String::from_utf8_lossy(&output.stdout).to_string();
                if message.contains("Everything up-to-date") {
                    Ok(OperationResult::Skipped(format!(
                        "Everything up-to-date: {}",
                        repo_path.display()
                    )))
                } else {
                    Ok(OperationResult::Success(format!(
                        "Pushed changes for {}",
                        repo_path.display()
                    )))
                }
            } else {
                let error = String::from_utf8_lossy(&output.stderr).to_string();
                Ok(OperationResult::Failure(format!(
                    "Failed to push {}: {}",
                    repo_path.display(),
                    error
                )))
            }
        })
    }
}