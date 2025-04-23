// src/operations/hook.rs
use crate::types::{HookSubCommand, OperationResult, RepositoryOperation};
use anyhow::{Context, Result};
use git2::Repository as Git2Repository;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

const HOOK_TYPES: &[&str] = &[
    "applypatch-msg",
    "pre-applypatch",
    "post-applypatch",
    "pre-commit",
    "prepare-commit-msg",
    "commit-msg",
    "post-commit",
    "pre-rebase",
    "post-checkout",
    "post-merge",
    "pre-push",
    "pre-receive",
    "update",
    "post-receive",
    "post-update",
    "push-to-checkout",
    "pre-auto-gc",
    "post-rewrite",
    "sendemail-validate",
];

pub struct HookOperation {
    subcommand: HookSubCommand,
}

impl HookOperation {
    pub fn new(subcommand: HookSubCommand) -> Self {
        Self { subcommand }
    }

    async fn list_hooks(&self, repo_path: &Path) -> Result<OperationResult> {
        let git_hooks_path = repo_path.join(".git/hooks");
        
        if !git_hooks_path.exists() {
            return Ok(OperationResult::Failure(format!(
                "Hooks directory not found: {}",
                git_hooks_path.display()
            )));
        }

        let mut hooks = Vec::new();
        
        for entry in fs::read_dir(&git_hooks_path)
            .context(format!("Failed to read hooks directory: {}", git_hooks_path.display()))?
        {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if !name.ends_with(".sample") && HOOK_TYPES.contains(&name) {
                        hooks.push(name.to_string());
                    }
                }
            }
        }

        if hooks.is_empty() {
            Ok(OperationResult::Success(format!(
                "No hooks found in {}",
                repo_path.display()
            )))
        } else {
            Ok(OperationResult::Success(format!(
                "Hooks in {}: {}",
                repo_path.display(),
                hooks.join(", ")
            )))
        }
    }

    async fn create_hook(
        &self,
        repo_path: &Path,
        hook_type: &str,
        script_path: &Path,
    ) -> Result<OperationResult> {
        // Validate hook type
        if !HOOK_TYPES.contains(&hook_type) {
            return Ok(OperationResult::Failure(format!(
                "Invalid hook type: {}. Valid types are: {}",
                hook_type,
                HOOK_TYPES.join(", ")
            )));
        }

        // Ensure script path exists
        if !script_path.exists() {
            return Ok(OperationResult::Failure(format!(
                "Script file not found: {}",
                script_path.display()
            )));
        }

        let git_hooks_path = repo_path.join(".git/hooks");
        
        if !git_hooks_path.exists() {
            return Ok(OperationResult::Failure(format!(
                "Hooks directory not found: {}",
                git_hooks_path.display()
            )));
        }

        let target_hook_path = git_hooks_path.join(hook_type);
        
        // Read script content
        let script_content = fs::read_to_string(script_path)
            .context(format!("Failed to read script: {}", script_path.display()))?;
        
        // Write to hook file
        let mut file = fs::File::create(&target_hook_path)
            .context(format!("Failed to create hook: {}", target_hook_path.display()))?;
        
        file.write_all(script_content.as_bytes())
            .context("Failed to write hook content")?;
        
        // Set executable permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&target_hook_path)?.permissions();
            perms.set_mode(0o755);  // rwxr-xr-x
            fs::set_permissions(&target_hook_path, perms)?;
        }

        Ok(OperationResult::Success(format!(
            "Created hook {} in {}",
            hook_type,
            repo_path.display()
        )))
    }

    async fn install_hook(&self, repo_path: &Path, hook_type: &str) -> Result<OperationResult> {
        // Validate hook type
        if !HOOK_TYPES.contains(&hook_type) {
            return Ok(OperationResult::Failure(format!(
                "Invalid hook type: {}. Valid types are: {}",
                hook_type,
                HOOK_TYPES.join(", ")
            )));
        }

        let git_hooks_path = repo_path.join(".git/hooks");
        
        if !git_hooks_path.exists() {
            return Ok(OperationResult::Failure(format!(
                "Hooks directory not found: {}",
                git_hooks_path.display()
            )));
        }

        // Check if there's a sample hook we can use
        let sample_hook_path = git_hooks_path.join(format!("{}.sample", hook_type));
        if !sample_hook_path.exists() {
            return Ok(OperationResult::Failure(format!(
                "No sample hook found for {} in {}",
                hook_type,
                repo_path.display()
            )));
        }

        let target_hook_path = git_hooks_path.join(hook_type);
        
        // Copy sample hook to active hook
        fs::copy(&sample_hook_path, &target_hook_path)
            .context(format!("Failed to copy sample hook from {}", sample_hook_path.display()))?;
        
        // Set executable permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&target_hook_path)?.permissions();
            perms.set_mode(0o755);  // rwxr-xr-x
            fs::set_permissions(&target_hook_path, perms)?;
        }

        Ok(OperationResult::Success(format!(
            "Installed {} hook in {}",
            hook_type,
            repo_path.display()
        )))
    }

    async fn remove_hook(&self, repo_path: &Path, hook_type: &str) -> Result<OperationResult> {
        // Validate hook type
        if !HOOK_TYPES.contains(&hook_type) {
            return Ok(OperationResult::Failure(format!(
                "Invalid hook type: {}. Valid types are: {}",
                hook_type,
                HOOK_TYPES.join(", ")
            )));
        }

        let hook_path = repo_path.join(".git/hooks").join(hook_type);
        
        if !hook_path.exists() {
            return Ok(OperationResult::Skipped(format!(
                "Hook {} not found in {}",
                hook_type,
                repo_path.display()
            )));
        }

        fs::remove_file(&hook_path)
            .context(format!("Failed to remove hook: {}", hook_path.display()))?;

        Ok(OperationResult::Success(format!(
            "Removed {} hook from {}",
            hook_type,
            repo_path.display()
        )))
    }
}

impl RepositoryOperation for HookOperation {
    fn name(&self) -> &'static str {
        "Hook"
    }

    fn description(&self) -> &'static str {
        "Manage Git hooks across repositories"
    }

    fn execute<'a>(&'a self, repo_path: &'a Path) -> 
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<OperationResult>> + Send + 'a>> {
        Box::pin(async move {
        // Ensure this is a Git repository
        match Git2Repository::open(repo_path) {
            Ok(_) => {
                debug!("Valid Git repository: {}", repo_path.display());
            }
            Err(e) => {
                debug!("Invalid Git repository {}: {}", repo_path.display(), e);
                return Ok(OperationResult::Skipped(format!(
                    "Not a valid Git repository: {}",
                    repo_path.display()
                )));
            }
        }

        match &self.subcommand {
            HookSubCommand::List => self.list_hooks(repo_path).await,
            
            HookSubCommand::Create { hook_type, script_path } => {
                self.create_hook(repo_path, hook_type, script_path).await
            }
            
            HookSubCommand::Install { hook_type } => {
                self.install_hook(repo_path, hook_type).await
            }
            
            HookSubCommand::Remove { hook_type } => {
                self.remove_hook(repo_path, hook_type).await
            }
        }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;
    use tokio::fs::create_dir_all;

    #[tokio::test]
    async fn test_hook_list() {
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();
        let git_dir = repo_path.join(".git");
        let hooks_dir = git_dir.join("hooks");
        
        // Create git directories
        create_dir_all(&hooks_dir).await.unwrap();
        
        // Create a test hook file
        let hook_path = hooks_dir.join("pre-commit");
        File::create(&hook_path).unwrap().write_all(b"#!/bin/sh\nexit 0\n").unwrap();
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&hook_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&hook_path, perms).unwrap();
        }
        
        // Create a sample hook file (should be ignored in list)
        let sample_hook_path = hooks_dir.join("post-commit.sample");
        File::create(&sample_hook_path).unwrap().write_all(b"#!/bin/sh\nexit 0\n").unwrap();
        
        let hook_op = HookOperation::new(HookSubCommand::List);
        let result = hook_op.execute(repo_path).await.unwrap();
        
        match result {
            OperationResult::Success(msg) => {
                assert!(msg.contains("pre-commit"));
                assert!(!msg.contains("post-commit.sample"));
            }
            _ => panic!("Expected Success result"),
        }
    }
    
    #[tokio::test]
    async fn test_hook_create() {
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();
        let git_dir = repo_path.join(".git");
        let hooks_dir = git_dir.join("hooks");
        
        // Create git directories
        create_dir_all(&hooks_dir).await.unwrap();
        
        // Create a script file to use as hook source
        let script_dir = temp_dir.path().join("scripts");
        create_dir_all(&script_dir).await.unwrap();
        let script_path = script_dir.join("test-hook.sh");
        File::create(&script_path).unwrap().write_all(b"#!/bin/sh\necho 'Test hook'\nexit 0\n").unwrap();
        
        let hook_op = HookOperation::new(HookSubCommand::Create {
            hook_type: "pre-commit".to_string(),
            script_path: script_path.clone(),
        });
        
        let result = hook_op.execute(repo_path).await.unwrap();
        
        match result {
            OperationResult::Success(_) => {
                let created_hook_path = hooks_dir.join("pre-commit");
                assert!(created_hook_path.exists());
                let content = fs::read_to_string(created_hook_path).unwrap();
                assert_eq!(content, "#!/bin/sh\necho 'Test hook'\nexit 0\n");
            }
            _ => panic!("Expected Success result"),
        }
    }
}