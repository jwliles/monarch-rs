// src/args.rs
use std::path::PathBuf;
use structopt::StructOpt;

use crate::types::Operation;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "monolith",
    about = "Advanced Git management suite for multiple repositories",
    version = "1.0"
)]
pub struct Args {
    /// Operation to perform
    #[structopt(subcommand)]
    pub operation: Operation,

    /// Source directory containing repositories
    #[structopt(short, long, parse(from_os_str))]
    pub source_dir: PathBuf,

    /// Target directory for clone operations
    #[structopt(short, long, parse(from_os_str))]
    pub target_dir: Option<PathBuf>,

    /// Enable verbose logging
    #[structopt(short, long)]
    pub verbose: bool,

    /// Reverse the operation (clone from target to source)
    #[structopt(long)]
    pub reverse: bool,

    /// Filter repositories by name pattern
    #[structopt(short, long)]
    pub filter: Option<String>,
}

impl Args {
    pub fn from_args() -> anyhow::Result<Self> {
        Ok(Self::from_args_safe()?)
    }
}