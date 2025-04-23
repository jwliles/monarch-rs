// src/utils/logger.rs
use anyhow::Result;
use tracing_subscriber::EnvFilter;

pub fn setup_logging(verbose: bool) -> Result<()> {
    let filter = if verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    if let Err(e) = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .try_init() {
        // Convert the error to a string and wrap it in anyhow::Error
        anyhow::bail!("Failed to initialize tracing subscriber: {}", e);
    }

    Ok(())
}