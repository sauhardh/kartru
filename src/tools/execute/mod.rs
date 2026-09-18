mod feat;

pub use feat::bash::{BashArgs, BashTool};
pub use feat::read::{ReadArgs, ReadTool};
pub use feat::write::{WriteArgs, WriteTool};

use crate::tools::error::ToolError;

pub trait ExecutableTool: Send + Sync {
    fn name(&self) -> &'static str;

    fn execute(&self, arguments: &str) -> Result<String, ToolError>;
}
