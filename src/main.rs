// src/main.rs
mod args;
mod config;
mod conflict;
mod diff;
mod editor;
mod error;
mod git;
mod hooks;
mod operations;
mod rebase;
mod types;
mod ui;
mod utils;

use anyhow::Result;
use tracing::{error, info};

use crate::args::Args;
use crate::operations::executor::OperationExecutor;
use crate::utils::logger;

const VERSION: &str = "1.0.0";
const DEFAULT_CONCURRENCY_LEVEL: usize = 16;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::from_args()?;

    // Initialize logging
    logger::setup_logging(args.verbose)?;

    // Print application header
    print_header();

    // Validate environment and options
    validate_environment(&args).await?;

    // Create and run the operation executor
    let executor = OperationExecutor::new(DEFAULT_CONCURRENCY_LEVEL);
    match executor.execute(args).await {
        Ok(_) => {
            info!("Operation completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Operation failed: {}", e);
            Err(e)
        }
    }
}

fn print_header() {
    use colored::*;
    println!("{}", "MONARCH".bright_green().bold());
    println!("{}", "Advanced Git Management Suite".bright_cyan());
    println!("Version: {}\n", VERSION);
}

async fn validate_environment(args: &Args) -> Result<()> {
    // Validate source directory
    if !args.source_dir.is_dir() {
        anyhow::bail!(
            "Source directory '{}' does not exist",
            args.source_dir.display()
        );
    }

    // Validate target directory for clone operations
    if matches!(args.operation, types::Operation::Clone)
        && !args.reverse
        && args.target_dir.is_none()
    {
        anyhow::bail!(
            "Target directory required for clone operation unless --reverse is specified"
        );
    }

    // Check git installation
    let git_version = tokio::process::Command::new("git")
        .arg("--version")
        .output()
        .await?;

    if !git_version.status.success() {
        anyhow::bail!("Git command not found. Please install Git and try again");
    }

    if args.verbose {
        let version = String::from_utf8_lossy(&git_version.stdout);
        info!("Using {}", version.trim());
    }

    Ok(())
}