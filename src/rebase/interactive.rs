//! Interactive rebase operations

use crate::rebase::tree::CommitTree;

/// Rebase operation types
pub enum RebaseOperation {
    /// Keep the commit as is
    Pick,
    /// Modify the commit message
    Reword,
    /// Edit the commit
    Edit,
    /// Discard the commit
    Drop,
    /// Combine the commit with the previous commit
    Squash,
    /// Combine the commit with the previous commit but discard the message
    Fixup,
    /// Execute a command after the commit
    Exec(String),
}

/// Rebase plan for a series of commits
pub struct RebasePlan {
    /// Commit operations in order
    pub operations: Vec<(String, RebaseOperation)>,
    /// Base commit ID
    pub base_commit: String,
}

/// Creates a rebase plan from the given commit tree
pub fn create_rebase_plan(_tree: &CommitTree, _base_commit: &str) -> anyhow::Result<RebasePlan> {
    todo!("Implement rebase plan creation")
}

/// Executes a rebase plan on the given repository
pub fn execute_rebase_plan(_repo_path: &str, _plan: &RebasePlan) -> anyhow::Result<()> {
    todo!("Implement rebase plan execution")
}
