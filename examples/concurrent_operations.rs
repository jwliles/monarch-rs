use anyhow::Result;
use monolith::operations::executor::OperationExecutor;
use monolith::types::Operation;
use std::path::PathBuf;
use tracing::info;

/// Example of running concurrent operations across multiple repositories
fn main() -> Result<()> {
    // Initialize logging
    monolith::utils::logger::setup_logging(true)?;

    info!("Starting concurrent operations example");

    // Create an executor with concurrency level 4
    let executor = OperationExecutor::new(4);

    // Define source directory
    let source_dir = PathBuf::from("/path/to/repositories");

    // Pull the latest changes from all repositories
    let pull_operation = Operation::Pull;
    
    // This would be wrapped in a proper Args struct in the real application
    // For this example, we're showing the conceptual flow
    println!("Pulling latest changes from all repositories in {}", source_dir.display());
    
    // In a real application, this would execute the pull operation concurrently
    // across all repositories in the source directory
    println!("Operation completed successfully");

    // Check status of all repositories
    let status_operation = Operation::Status;
    
    println!("Checking status of all repositories in {}", source_dir.display());
    
    // In a real application, this would return the status of all repositories
    println!("Status check completed successfully");

    // Run a custom command on all repositories
    let custom_command = "git gc";
    let run_operation = Operation::Run(custom_command.to_string());
    
    println!("Running '{}' on all repositories in {}", custom_command, source_dir.display());
    
    // In a real application, this would execute the custom command concurrently
    println!("Custom command execution completed successfully");

    Ok(())
}