pub mod mcp;
mod native;

use async_trait::async_trait;
pub use native::bash::BashTool;
pub use native::read::ReadTool;
pub use native::write::WriteTool;

use crate::tools::define::ToolDefinition;
use crate::tools::error::ToolError;

#[async_trait]
pub trait ExecutableTool: Send + Sync {
    fn name(&self) -> &str;

    fn definition(&self) -> ToolDefinition;

    async fn execute(&self, arguments: &str) -> Result<String, ToolError>;
}
