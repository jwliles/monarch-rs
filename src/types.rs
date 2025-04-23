// src/types.rs
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub enum Operation {
    /// Clone repositories from source to target directory
    Clone,
    
    /// Pull latest changes for all repositories in source directory
    Pull,
    
    /// Push local changes for all repositories in source directory
    Push,
    
    /// List all repositories and their status
    Status,
    
    /// Run arbitrary git command on all repositories
    Run {
        /// Git command to run
        #[structopt(required = true)]
        command: Vec<String>,
    },
    
    /// Manage Git hooks across repositories
    Hook {
        /// Hook subcommand to execute
        #[structopt(subcommand)]
        subcommand: HookSubCommand,
    },
}

#[derive(Debug, StructOpt, Clone)]
pub enum HookSubCommand {
    /// List available hooks for repositories
    List,
    
    /// Create a new hook or update an existing one
    Create {
        /// Type of hook to create (pre-commit, post-commit, etc.)
        #[structopt(required = true)]
        hook_type: String,
        
        /// Path to the script file to use for the hook
        #[structopt(required = true)]
        script_path: std::path::PathBuf,
    },
    
    /// Install hooks to repositories
    Install {
        /// Type of hook to install (pre-commit, post-commit, etc.)
        #[structopt(required = true)]
        hook_type: String,
    },
    
    /// Remove hooks from repositories
    Remove {
        /// Type of hook to remove (pre-commit, post-commit, etc.)
        #[structopt(required = true)]
        hook_type: String,
    },
}

/// Represents the result of an operation on a repository
#[derive(Debug)]
pub enum OperationResult {
    Success(String),
    Failure(String),
    Skipped(String),
}

/// Trait for repository operations
pub trait RepositoryOperation: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    
    // Use Box<dyn Future> instead of async fn for dynamic dispatch
    fn execute<'a>(&'a self, repo_path: &'a std::path::Path) -> 
        std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<OperationResult>> + Send + 'a>>;
}