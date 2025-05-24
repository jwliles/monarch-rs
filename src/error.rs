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

#[derive(Debug)]
pub enum MonarchError {
    Git(GitError),
    Io(std::io::Error),
    Other(String),
}

impl fmt::Display for MonarchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MonarchError::Git(err) => write!(f, "Git error: {}", err),
            MonarchError::Io(err) => write!(f, "IO error: {}", err),
            MonarchError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for MonarchError {}

impl From<GitError> for MonarchError {
    fn from(err: GitError) -> Self {
        MonarchError::Git(err)
    }
}

impl From<std::io::Error> for MonarchError {
    fn from(err: std::io::Error) -> Self {
        MonarchError::Io(err)
    }
}

impl From<String> for MonarchError {
    fn from(err: String) -> Self {
        MonarchError::Other(err)
    }
}

impl From<&str> for MonarchError {
    fn from(err: &str) -> Self {
        MonarchError::Other(err.to_string())
    }
}