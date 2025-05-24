//! Interactive conflict resolution

use crate::conflict::detector::Conflict;

/// Resolution strategy
pub enum ResolutionStrategy {
    /// Keep our version
    Ours,
    /// Keep their version
    Theirs,
    /// Manually edit
    Manual,
    /// Keep both versions
    Both,
    /// Skip file
    Skip,
    /// Abort resolution
    Abort,
}

/// Resolves a conflict
pub fn resolve_conflict(conflict: &Conflict, strategy: ResolutionStrategy) -> anyhow::Result<()> {
    todo!("Implement conflict resolution")
}

/// Gets available resolution strategies for a conflict
pub fn get_resolution_strategies(conflict: &Conflict) -> Vec<ResolutionStrategy> {
    todo!("Implement strategy retrieval")
}

/// Launches an interactive conflict resolution session
pub fn start_interactive_resolution(repo_path: &str) -> anyhow::Result<()> {
    todo!("Implement interactive conflict resolution")
}
