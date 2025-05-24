//! Diff rendering and display

/// Represents a diff chunk
pub struct DiffChunk {
    /// Start line in the original file
    pub original_start: usize,
    /// Number of lines in the original file
    pub original_count: usize,
    /// Start line in the modified file
    pub modified_start: usize,
    /// Number of lines in the modified file
    pub modified_count: usize,
    /// Lines in the chunk
    pub lines: Vec<DiffLine>,
}

/// Represents a line in a diff
pub enum DiffLine {
    /// Line is common to both files
    Common(String),
    /// Line was added
    Added(String),
    /// Line was removed
    Removed(String),
}

/// Renders a diff to HTML
pub fn render_html(chunks: &[DiffChunk], syntax_highlight: bool) -> anyhow::Result<String> {
    todo!("Implement HTML diff rendering")
}

/// Renders a diff to ANSI terminal output
pub fn render_ansi(chunks: &[DiffChunk]) -> anyhow::Result<String> {
    todo!("Implement ANSI diff rendering")
}
