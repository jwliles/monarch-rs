// src/operations/run.rs
use crate::types::{OperationResult, RepositoryOperation};
use anyhow::{Context, Result};
use std::path::Path;
use tracing::{debug, info};

pub struct RunOperation {
    command: Vec<String>,
}

impl RunOperation {
    pub fn new(command: Vec<String>) -> Self {
        Self { command }
    }
}

impl RepositoryOperation for RunOperation {
    fn name(&self) -> &'static str {
        "Run"
    }

    fn description(&self) -> &'static str {
        "Run arbitrary git command"
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

            // Create git command
            let mut cmd = tokio::process::Command::new("git");
            cmd.current_dir(repo_path);
            
            // Add all arguments
            for arg in &self.command {
                cmd.arg(arg);
            }

            debug!("Running command in {}: {:?}", repo_path.display(), cmd);
            
            let output = cmd
                .output()
                .await
                .context("Failed to execute git command")?;

            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            
            if output.status.success() {
                Ok(OperationResult::Success(format!(
                    "Command completed successfully in {}:\n{}{}",
                    repo_path.display(),
                    stdout,
                    if !stderr.is_empty() { format!("\nStderr: {}", stderr) } else { String::new() }
                )))
            } else {
                Ok(OperationResult::Failure(format!(
                    "Command failed in {}:\n{}{}",
                    repo_path.display(),
                    stdout,
                    if !stderr.is_empty() { format!("\nStderr: {}", stderr) } else { String::new() }
                )))
            }
        })
    }
}