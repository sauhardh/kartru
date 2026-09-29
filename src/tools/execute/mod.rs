mod mcp;
mod native;

use async_trait::async_trait;
pub use native::bash::{BashArgs, BashTool};
pub use native::read::{ReadArgs, ReadTool};
pub use native::write::{WriteArgs, WriteTool};

use crate::tools::define::ToolDefinition;
use crate::tools::error::ToolError;

#[async_trait]
pub trait ExecutableTool: Send + Sync {
    fn name(&self) -> &'static str;

    fn definition(&self) -> ToolDefinition;

    async fn execute(&self, arguments: &str) -> Result<String, ToolError>;
}
