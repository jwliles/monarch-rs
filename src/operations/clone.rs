// src/operations/clone.rs
use crate::types::{OperationResult, RepositoryOperation};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::{debug, info};

pub struct CloneOperation {
    target_dir: PathBuf,
    reverse: bool,
}

impl CloneOperation {
    pub fn new(target_dir: PathBuf, reverse: bool) -> Self {
        Self { target_dir, reverse }
    }
}

impl RepositoryOperation for CloneOperation {
    fn name(&self) -> &'static str {
        "Clone"
    }

    fn description(&self) -> &'static str {
        "Clone repositories from source to target directory"
    }

    fn execute<'a>(&'a self, repo_path: &'a Path) -> 
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<OperationResult>> + Send + 'a>> {
        Box::pin(async move {
            // Get repository name
            let repo_name = repo_path
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("Invalid repository path"))?
                .to_string_lossy()
                .to_string();

            // Determine target path
            let target_path = self.target_dir.join(&repo_name);

            // Skip if target already exists
            if target_path.exists() {
                debug!("Target already exists: {}", target_path.display());
                return Ok(OperationResult::Skipped(format!(
                    "Target already exists: {}",
                    target_path.display()
                )));
            }

            // Create target directory if it doesn't exist
            if !self.target_dir.exists() {
                tokio::fs::create_dir_all(&self.target_dir)
                    .await
                    .context("Failed to create target directory")?;
            }

            // Run git clone command
            let git_url = format!("file://{}", repo_path.display());
            let mut cmd = tokio::process::Command::new("git");
            cmd.arg("clone").arg(git_url).arg(&target_path);

            debug!("Running: {:?}", cmd);
            
            let status = cmd
                .status()
                .await
                .context("Failed to execute git clone command")?;

            if status.success() {
                Ok(OperationResult::Success(format!(
                    "Cloned {} to {}",
                    repo_name,
                    target_path.display()
                )))
            } else {
                Ok(OperationResult::Failure(format!(
                    "Failed to clone {} to {}",
                    repo_name,
                    target_path.display()
                )))
            }
        })
    }
}