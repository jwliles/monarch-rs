//! # Monarch: Advanced Git Management Suite
//!
//! Monarch is a powerful platform for managing multiple Git repositories concurrently 
//! with a unified interface, intelligent hook management, and advanced visualization.
//!
//! ## Features
//!
//! - **Multi-Repository Management**: Concurrent operations across multiple repositories
//! - **Status-at-a-Glance**: Visual monitoring of repository status with customizable indicators
//! - **Command Sequencing**: Chain Git commands into visual pipelines
//! - **Intelligent Error Handling**: Contextual suggestions for resolving Git errors
//! - **Visual Hook Builder**: Create and manage Git hooks with a user-friendly interface
//! - **Cross-Repository Analysis**: Track patterns and activity across your repositories

pub mod args;
pub mod config;
pub mod conflict;
pub mod diff;
pub mod editor;
pub mod error;
pub mod git;
pub mod hooks;
pub mod operations;
pub mod rebase;
pub mod types;
pub mod ui;
pub mod utils;

/// Re-export commonly used types
pub use error::MonarchError;
pub use types::Operation;