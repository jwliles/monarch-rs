//! Types and structures for hook management

/// Git hook types
pub enum HookType {
    /// pre-commit hook
    PreCommit,
    /// commit-msg hook
    CommitMsg,
    /// pre-push hook
    PrePush,
    /// post-checkout hook
    PostCheckout,
    /// pre-rebase hook
    PreRebase,
    /// post-merge hook
    PostMerge,
    /// prepare-commit-msg hook
    PrepareCommitMsg,
    /// Custom hook type
    Custom(String),
}

/// Hook metadata
pub struct HookMetadata {
    /// Hook name
    pub name: String,
    /// Hook description
    pub description: String,
    /// Hook type
    pub hook_type: HookType,
    /// Author of the hook
    pub author: String,
    /// Creation date
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last modified date
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Tags for organization
    pub tags: Vec<String>,
}

/// Hook script content
pub struct HookScript {
    /// Hook metadata
    pub metadata: HookMetadata,
    /// Script content
    pub content: String,
    /// Script language
    pub language: ScriptLanguage,
}

/// Script language types
pub enum ScriptLanguage {
    /// Shell script
    Shell,
    /// Python script
    Python,
    /// Ruby script
    Ruby,
    /// JavaScript script
    JavaScript,
    /// Custom language
    Custom(String),
}
