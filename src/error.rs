// src/error.rs
use std::fmt;

#[derive(Debug)]
pub enum GitError {
    CommandFailed(String),
    RepositoryNotFound(String),
    InvalidPath(String),
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitError::CommandFailed(cmd) => write!(f, "Git command failed: {}", cmd),
            GitError::RepositoryNotFound(path) => write!(f, "Git repository not found: {}", path),
            GitError::InvalidPath(path) => write!(f, "Invalid path: {}", path),
        }
    }
}

impl std::error::Error for GitError {}