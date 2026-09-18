#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("unknown tool: {0}")]
    UnknownTool(String),

    #[error("Invalid Arguments: {0}")]
    InvalidArguments(#[from] serde_json::Error),

    #[error("Execution Failed: {0}")]
    Execution(#[from] std::io::Error),
}
